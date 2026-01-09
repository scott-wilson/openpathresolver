/// A helper trait for running the create workspace logic.
///
/// The [create_workspace] function hands over the responsibility of creating files and folders to
/// the type that implements this trait. The resolver does not assume anything about the
/// permissions or ownership aside from "this path is owned by a user" or "this path is read only".
#[async_trait::async_trait]
pub trait CreateWorkspaceIoFunction {
    /// The function that gets called by the [create_workspace] function when building the
    /// workspace.
    async fn call(
        &self,
        config: std::sync::Arc<crate::Config>,
        template_fields: std::sync::Arc<crate::types::TemplateAttributes>,
        path_item: crate::ResolvedPathItem,
    ) -> Result<(), crate::Error>;
}

/// Build a workspace by creating the files and folders for the given fields.
///
/// The create workspace function will use the `path_fields` to decide if a path should be built or
/// not. In other words, this will create paths that can be resolved with the path fields, but
/// other paths will not be created.
///
/// # Example
///
/// ```rust
/// # use openpathresolver::{ConfigBuilder, create_workspace, Owner, PathItemArgs, PathType, Permission, Error, CreateWorkspaceIoFunction, FieldKey, TemplateValue, ResolvedPathItem, Config};
/// struct Func;
///
/// #[async_trait::async_trait]
/// impl CreateWorkspaceIoFunction for Func {
///     async fn call(
///         &self,
///         _config: std::sync::Arc<Config>,
///         _template_fields: std::sync::Arc<std::collections::HashMap<FieldKey, TemplateValue>>,
///         _path_item: ResolvedPathItem,
///     ) -> Result<(), Error> {
///         Ok(())
///     }
/// }
///
/// # #[tokio::main]
/// # async fn main() {
/// let config = ConfigBuilder::new()
///     .add_path_item(PathItemArgs {
///         key: "key1".try_into().unwrap(),
///         path: "/path/to/{thing}".into(),
///         parent: None,
///         permission: Permission::default(),
///         owner: Owner::default(),
///         path_type: PathType::default(),
///         deferred: false,
///         metadata: std::collections::HashMap::new(),
///     })
///     .unwrap()
///     .add_path_item(PathItemArgs {
///         key: "key2".try_into().unwrap(),
///         path: "/path/to/a/{thing}".into(),
///         parent: None,
///         permission: Permission::default(),
///         owner: Owner::default(),
///         path_type: PathType::default(),
///         deferred: false,
///         metadata: std::collections::HashMap::new(),
///     })
///     .unwrap()
///     .add_path_item(PathItemArgs {
///         key: "key3".try_into().unwrap(),
///         path: "/path/to/b/{thing}".into(),
///         parent: None,
///         permission: Permission::default(),
///         owner: Owner::default(),
///         path_type: PathType::default(),
///         deferred: false,
///         metadata: std::collections::HashMap::new(),
///     })
///     .unwrap()
///     .build()
///     .unwrap();
///
/// let path_fields = {
///     let mut fields = std::collections::HashMap::new();
///     fields.insert("thing".try_into().unwrap(), "value".into());
///
///     fields
/// };
/// let template_fields = {
///     let mut fields = std::collections::HashMap::new();
///     fields.insert("thing".try_into().unwrap(), "value".into());
///
///     fields
/// };
///
/// create_workspace(
///     std::sync::Arc::new(config),
///     &path_fields,
///     std::sync::Arc::new(template_fields),
///     Func,
/// )
/// .await
/// .unwrap();
/// # }
/// ```
pub async fn create_workspace<Func: CreateWorkspaceIoFunction + Send + Sync + 'static>(
    config: std::sync::Arc<crate::Config>,
    path_fields: &crate::types::PathAttributes,
    template_fields: std::sync::Arc<crate::types::TemplateAttributes>,
    io_function: Func,
) -> Result<(), crate::Error> {
    let resolved_items = get_workspace(config.as_ref(), path_fields)?;
    let mut parent_resolved_map = std::collections::BTreeMap::new();

    for resolved_item in &resolved_items {
        let parent = resolved_item.value.parent();
        parent_resolved_map
            .entry(parent)
            .or_insert(Vec::new())
            .push(resolved_item.clone());
    }

    let mut workers_set = tokio::task::JoinSet::new();
    let io_function = std::sync::Arc::new(io_function);

    for (_, child_resolved_items) in parent_resolved_map {
        for resolved_item in child_resolved_items {
            let io_function = io_function.clone();
            let config = config.clone();
            let template_fields = template_fields.clone();
            workers_set.spawn(async move {
                io_function
                    .call(config, template_fields, resolved_item)
                    .await
            });
        }

        while let Some(response) = workers_set.join_next().await {
            // TODO: Don't use unwrap here.
            response.unwrap()?;
        }
    }

    Ok(())
}

/// Get all of the path items that would be created with the [create_workspace] function.
///
/// The only paths that will be returned are paths that can be fully resolved with the given path
/// fields.
///
/// # Example
///
/// ```rust
/// # use openpathresolver::{ConfigBuilder, get_workspace, Owner, PathItemArgs, PathType, Permission};
/// let config = ConfigBuilder::new()
///     .add_path_item(PathItemArgs {
///         key: "key1".try_into().unwrap(),
///         path: "/path/to/{thing}".into(),
///         parent: None,
///         permission: Permission::default(),
///         owner: Owner::default(),
///         path_type: PathType::default(),
///         deferred: false,
///         metadata: std::collections::HashMap::new(),
///     })
///     .unwrap()
///     .add_path_item(PathItemArgs {
///         key: "key2".try_into().unwrap(),
///         path: "/path/to/a/{thing}".into(),
///         parent: None,
///         permission: Permission::default(),
///         owner: Owner::default(),
///         path_type: PathType::default(),
///         deferred: false,
///         metadata: std::collections::HashMap::new(),
///     })
///     .unwrap()
///     .add_path_item(PathItemArgs {
///         key: "key3".try_into().unwrap(),
///         path: "/path/to/b/{thing}".into(),
///         parent: None,
///         permission: Permission::default(),
///         owner: Owner::default(),
///         path_type: PathType::default(),
///         deferred: false,
///         metadata: std::collections::HashMap::new(),
///     })
///     .unwrap()
///     .build()
///     .unwrap();
///
/// let fields = {
///     let mut fields = std::collections::HashMap::new();
///     fields.insert("thing".try_into().unwrap(), "value".into());
///
///     fields
/// };
///
/// get_workspace(
///     &config,
///     &fields,
/// )
/// .unwrap();
/// ```
pub fn get_workspace(
    config: &crate::Config,
    path_fields: &crate::types::PathAttributes,
) -> Result<Vec<crate::ResolvedPathItem>, crate::Error> {
    let mut queue = std::collections::VecDeque::new();
    let mut parent_children_map = std::collections::HashMap::new();

    for (index, item) in config.items.iter().enumerate() {
        match item.parent {
            Some(parent_index) => parent_children_map
                .entry(parent_index)
                .or_insert(Vec::new())
                .push(index),
            None => queue.push_back((item, index)),
        };
    }

    #[allow(clippy::too_many_arguments)]
    fn recursive_build_items(
        config: &crate::Config,
        parent_resolved_item: &crate::ResolvedPathItem,
        item: &crate::types::PathItem,
        index: usize,
        path_fields: &crate::types::PathAttributes,
        parent_children_map: &std::collections::HashMap<usize, Vec<usize>>,
        index_key_map: &std::collections::HashMap<usize, crate::FieldKey>,
        resolved_items: &mut Vec<crate::ResolvedPathItem>,
    ) -> Result<(), crate::Error> {
        if !item.path.is_resolved_by(path_fields) {
            return Ok(());
        }
        let value = {
            let mut path_part = String::new();
            item.path
                .draw(&mut path_part, path_fields, &config.resolvers)?;

            parent_resolved_item.value.join(path_part)
        };
        let permission = match item.permission {
            crate::types::Permission::Inherit => parent_resolved_item.permission,
            _ => item.permission,
        };
        let owner = match item.owner {
            crate::types::Owner::Inherit => parent_resolved_item.owner,
            _ => item.owner,
        };
        let path_type = item.path_type;
        let key = index_key_map.get(&index).cloned();
        let deferred = match parent_resolved_item.deferred {
            true => {
                if item.path.has_variable_tokens() && item.path.is_resolved_by(path_fields) {
                    false
                } else {
                    item.deferred
                }
            }
            false => false,
        };

        let resolved_item = crate::ResolvedPathItem {
            key,
            value,
            permission,
            owner,
            path_type,
            deferred,
            metadata: std::collections::HashMap::new(),
        };

        let child_indexes = parent_children_map.get(&index);

        if let Some(child_indexes) = child_indexes {
            for child_index in child_indexes.iter() {
                let child_item = &config.items[*child_index];
                recursive_build_items(
                    config,
                    &resolved_item,
                    child_item,
                    *child_index,
                    path_fields,
                    parent_children_map,
                    index_key_map,
                    resolved_items,
                )?;
            }
        }

        resolved_items.push(resolved_item);

        Ok(())
    }

    let mut resolved_items = Vec::new();
    let index_key_map = config
        .item_map
        .iter()
        .map(|(k, v)| (*v, k.to_owned()))
        .collect::<std::collections::HashMap<_, _>>();

    for (item, index) in queue.into_iter() {
        let key = index_key_map.get(&index).cloned();
        let resolved_item = crate::ResolvedPathItem {
            key,
            value: std::path::PathBuf::new(),
            permission: item.permission,
            owner: item.owner,
            path_type: item.path_type,
            deferred: item.deferred,
            metadata: item.metadata.clone(),
        };
        recursive_build_items(
            config,
            &resolved_item,
            item,
            index,
            path_fields,
            &parent_children_map,
            &index_key_map,
            &mut resolved_items,
        )?;
    }

    resolved_items.sort_by(|a, b| a.value.cmp(&b.value));

    for parent_index in 0..resolved_items.len() {
        for child_index in (parent_index + 1)..resolved_items.len() {
            if !resolved_items[parent_index]
                .value
                .starts_with(&resolved_items[child_index].value)
            {
                continue;
            }

            if !resolved_items[child_index].deferred {
                resolved_items[child_index].deferred = resolved_items[parent_index].deferred;
            }
        }
    }

    let mut filtered_resolved_items = Vec::new();

    for resolved_item in resolved_items {
        if !resolved_item.deferred {
            filtered_resolved_items.push(resolved_item);
        }
    }

    Ok(filtered_resolved_items)
}

#[cfg(test)]
mod tests {
    use crate::{Owner, PathItemArgs, PathType, Permission};

    use super::*;

    #[test]
    fn test_get_workspace_success() {
        let config = crate::ConfigBuilder::new()
            .add_path_item(PathItemArgs {
                key: "key1".try_into().unwrap(),
                path: "/path/to/{thing}".into(),
                parent: None,
                permission: Permission::default(),
                owner: Owner::default(),
                path_type: PathType::default(),
                deferred: false,
                metadata: std::collections::HashMap::new(),
            })
            .unwrap()
            .add_path_item(PathItemArgs {
                key: "key2".try_into().unwrap(),
                path: "/path/to/a/{thing}".into(),
                parent: None,
                permission: Permission::default(),
                owner: Owner::default(),
                path_type: PathType::default(),
                deferred: false,
                metadata: std::collections::HashMap::new(),
            })
            .unwrap()
            .add_path_item(PathItemArgs {
                key: "key3".try_into().unwrap(),
                path: "/path/to/b/{thing}".into(),
                parent: None,
                permission: Permission::default(),
                owner: Owner::default(),
                path_type: PathType::default(),
                deferred: false,
                metadata: std::collections::HashMap::new(),
            })
            .unwrap()
            .build()
            .unwrap();

        let fields = {
            let mut fields = crate::types::PathAttributes::new();
            fields.insert("thing".try_into().unwrap(), "value".into());

            fields
        };
        let resolved_items = get_workspace(&config, &fields).unwrap();

        assert_eq!(resolved_items.len(), 8);
        for (index, expected) in [
            "/",
            "/path",
            "/path/to",
            "/path/to/a",
            "/path/to/a/value",
            "/path/to/b",
            "/path/to/b/value",
            "/path/to/value",
        ]
        .into_iter()
        .enumerate()
        {
            assert_eq!(
                resolved_items[index]
                    .value
                    .to_string_lossy()
                    .replace("\\", "/"),
                expected
            );
        }
    }

    #[tokio::test]
    async fn test_create_workspace_success() {
        let config = crate::ConfigBuilder::new()
            .add_path_item(PathItemArgs {
                key: "key1".try_into().unwrap(),
                path: "/path/to/{thing}".into(),
                parent: None,
                permission: Permission::default(),
                owner: Owner::default(),
                path_type: PathType::default(),
                deferred: false,
                metadata: std::collections::HashMap::new(),
            })
            .unwrap()
            .add_path_item(PathItemArgs {
                key: "key2".try_into().unwrap(),
                path: "/path/to/a/{thing}".into(),
                parent: None,
                permission: Permission::default(),
                owner: Owner::default(),
                path_type: PathType::default(),
                deferred: false,
                metadata: std::collections::HashMap::new(),
            })
            .unwrap()
            .add_path_item(PathItemArgs {
                key: "key3".try_into().unwrap(),
                path: "/path/to/b/{thing}".into(),
                parent: None,
                permission: Permission::default(),
                owner: Owner::default(),
                path_type: PathType::default(),
                deferred: false,
                metadata: std::collections::HashMap::new(),
            })
            .unwrap()
            .build()
            .unwrap();

        let path_fields = {
            let mut fields = crate::types::PathAttributes::new();
            fields.insert("thing".try_into().unwrap(), "value".into());

            fields
        };
        let template_fields = {
            let mut fields = crate::types::TemplateAttributes::new();
            fields.insert("thing".try_into().unwrap(), "value".into());

            fields
        };

        struct Func;

        #[async_trait::async_trait]
        impl CreateWorkspaceIoFunction for Func {
            async fn call(
                &self,
                _config: std::sync::Arc<crate::Config>,
                _template_fields: std::sync::Arc<crate::types::TemplateAttributes>,
                _path_item: crate::ResolvedPathItem,
            ) -> Result<(), crate::Error> {
                Ok(())
            }
        }

        create_workspace(
            std::sync::Arc::new(config),
            &path_fields,
            std::sync::Arc::new(template_fields),
            Func,
        )
        .await
        .unwrap();
    }
}
