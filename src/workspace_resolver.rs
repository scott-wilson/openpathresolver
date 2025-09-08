pub fn create_workspace(
    config: &crate::Config,
    path_fields: &crate::types::PathAttributes,
    template_fields: &crate::types::TemplateAttributes,
    io_function: impl Fn(
        &crate::Config,
        &crate::ResolvedPathItem,
        &crate::types::TemplateAttributes,
    ) -> Result<(), crate::Error>,
) -> Result<(), crate::Error> {
    let resolved_items = get_workspace(config, path_fields)?;
    let mut parent_resolved_map = std::collections::BTreeMap::new();

    for resolved_item in &resolved_items {
        let parent = resolved_item.value.parent();
        parent_resolved_map
            .entry(parent)
            .or_insert(Vec::new())
            .push(resolved_item);
    }

    for (_, child_resolved_items) in parent_resolved_map {
        for resolved_item in child_resolved_items {
            io_function(config, resolved_item, template_fields)?;
        }
    }

    Ok(())
}

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
        if !item.value.is_resolved_by(path_fields) {
            return Ok(());
        }
        let value = {
            let mut path_part = String::new();
            item.value
                .draw(&mut path_part, path_fields, &config.resolvers)?;

            parent_resolved_item.value.join(path_part)
        };
        let permission = match item.permission {
            crate::types::Permission::Inherit => parent_resolved_item.permission,
            _ => item.permission,
        };
        let owner = match item.owner {
            crate::types::Owner::Inherit => parent_resolved_item.owner.clone(),
            _ => item.owner.clone(),
        };
        let copy_file = item.copy_file.clone();
        let key = index_key_map.get(&index).cloned();
        let deferred = match parent_resolved_item.deferred {
            true => {
                if item.value.has_variable_tokens() && item.value.is_resolved_by(path_fields) {
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
            copy_file,
            deferred,
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

    for (item, index) in queue.iter() {
        let key = index_key_map.get(index).cloned();
        let resolved_item = crate::ResolvedPathItem {
            key,
            value: std::path::PathBuf::new(),
            permission: item.permission.to_owned(),
            owner: item.owner.to_owned(),
            copy_file: item.copy_file.to_owned(),
            deferred: item.deferred,
        };
        recursive_build_items(
            config,
            &resolved_item,
            item,
            *index,
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
    use super::*;

    #[test]
    fn test_get_workspace_success() {
        let config = crate::ConfigBuilder::new()
            .add_path_item(
                "key",
                "/path/to/{thing}",
                None,
                &crate::Permission::default(),
                &crate::Owner::default(),
                &crate::CopyFile::default(),
                false,
            )
            .unwrap()
            .build()
            .unwrap();

        let fields = {
            let mut fields = crate::types::PathAttributes::new();
            fields.insert("thing".try_into().unwrap(), "value".into());

            fields
        };
        let resolved_items = get_workspace(&config, &fields).unwrap();

        assert_eq!(resolved_items.len(), 4);
        assert_eq!(resolved_items[0].value.to_string_lossy(), "/");
        assert_eq!(resolved_items[1].value.to_string_lossy(), "/path");
        assert_eq!(resolved_items[2].value.to_string_lossy(), "/path/to");
        assert_eq!(resolved_items[3].value.to_string_lossy(), "/path/to/value");
    }
}
