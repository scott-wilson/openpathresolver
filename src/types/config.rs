use crate::types::{FieldKey, PathItem, PathItemArgs, Resolver, Resolvers, Tokens};

/// Store the resolver configs.
///
/// The config stores two major components. The resolvers, which are responsible for resolving the
/// placholder values, and the items, which are all of the path parts that are used to find paths
/// or used to create paths.
#[derive(Debug, Clone)]
pub struct Config {
    pub(crate) resolvers: Resolvers,
    pub(crate) item_map: std::collections::HashMap<FieldKey, usize>,
    pub(crate) items: Vec<PathItem>,
}

impl Config {
    pub(crate) fn get_item(&self, key: &FieldKey) -> Option<Vec<&PathItem>> {
        let last_id = match self.item_map.get(key) {
            Some(id) => *id,
            None => return None,
        };

        let mut items: Vec<&PathItem> = Vec::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(last_id);

        while !queue.is_empty() {
            let item_id = queue.pop_front().unwrap();
            let item = &self.items[item_id];
            items.push(item);

            if let Some(parent_id) = item.parent {
                queue.push_back(parent_id);
            }
        }

        Some(items.iter().rev().copied().collect())
    }
}

/// Build a config.
///
/// This will build a config from the input resolvers and then validate and output the config.
#[derive(Debug, Default)]
pub struct ConfigBuilder {
    resolvers: Resolvers,
    items: std::collections::HashMap<FieldKey, PathItemArgs>,
}

impl ConfigBuilder {
    /// Prepare a new builder.
    pub fn new() -> Self {
        Self {
            resolvers: std::collections::HashMap::new(),
            items: std::collections::HashMap::new(),
        }
    }

    /// Add a string resolver.
    ///
    /// The string resolver is the simplest type of resolver. It doesn't have much context other
    /// than this is a string of characters, and maybe the expected shape of the string (if the
    /// regex pattern is specified). The pattern, if specified, must follow these rules:
    ///
    /// - It must be as non-greedy as possible (for example, use `\w+?` instead of `\w+`). This
    ///   prevents the pattern from consuming more than it should.
    /// - It must not use any anchors such as `^` or `$`. When the system builds the internal regex
    ///   from the supplied regexes, it will automatically add the anchors to make the path query
    ///   more specific.
    /// - It must not use capturing groups. The internal regex may create capture groups when
    ///   extracting the field values from paths.
    pub fn add_string_resolver(
        mut self,
        key: impl TryInto<crate::FieldKey, Error = crate::Error>,
        pattern: Option<&str>,
    ) -> Result<Self, crate::Error> {
        self.resolvers.insert(
            key.try_into()?,
            Resolver::String {
                pattern: match pattern {
                    Some(pattern) => Some(regex::Regex::new(pattern)?),
                    None => None,
                },
            },
        );
        Ok(self)
    }

    /// Add an integer resolver.
    ///
    /// Integer resolvers will create integers with zero padding. When the integers are being
    /// extracted from a path, then only numbers with a minimum number of characters based on the
    /// padding are considered valid. For example, if the padding is 3 then `1` and `12` are
    /// invalid, but `001`, `012`, `123`, and `1234` are valid.
    pub fn add_integer_resolver(
        mut self,
        key: impl TryInto<crate::FieldKey, Error = crate::Error>,
        padding: u8,
    ) -> Result<Self, crate::Error> {
        self.resolvers
            .insert(key.try_into()?, Resolver::Integer { padding });
        Ok(self)
    }

    /// Add a path item.
    ///
    /// Path items are parts of paths that are either fully resolved (contain no placeholders), or
    /// partially resolved (contains placeholders). See [PathItemArgs](crate::PathItemArgs) for more information.
    pub fn add_path_item(mut self, args: crate::PathItemArgs) -> Result<Self, crate::Error> {
        if self.items.contains_key(&args.key) {
            return Err(crate::Error::new(format!(
                "'{}' already in path items.",
                args.key
            )));
        }

        self.items.insert(args.key.clone(), args);
        Ok(self)
    }

    /// Build the config from the builder.
    ///
    /// # Errors
    ///
    /// - Path items must not form a circular dependency through the parent key.
    /// - If a path item defines a parent, the parent must be defined in the current builder.
    /// - If the path parts have placeholders, then the syntax must be correct. However, a
    ///   placeholder does not need to reference a resolver (it will assume a string resolver).
    pub fn build(mut self) -> Result<Config, crate::Error> {
        // Find items with parents that cause infinite recursion errors.
        let mut queue = std::collections::VecDeque::new();
        let mut visited = std::collections::HashSet::new();
        let mut validated = std::collections::HashSet::new();
        let mut parent_resolved_path_items_map = std::collections::BTreeMap::new();
        let mut visited_paths = std::collections::HashSet::new();

        fn recursive_build_path<'a>(
            path_part: &'a std::path::Path,
            parent_key: &'a Option<FieldKey>,
            item_map: &'a std::collections::HashMap<FieldKey, PathItemArgs>,
        ) -> std::borrow::Cow<'a, std::path::Path> {
            let parent_key = match parent_key {
                Some(parent_key) => parent_key,
                None => return path_part.into(),
            };
            let parent_item_args = match item_map.get(parent_key) {
                Some(item_args) => item_args,
                None => return path_part.into(),
            };
            let parent_path_part =
                recursive_build_path(&parent_item_args.path, &parent_item_args.parent, item_map);

            parent_path_part.join(path_part).into()
        }

        for (key, item) in self.items.iter() {
            if validated.contains(key) {
                continue;
            }

            queue.clear();
            visited.clear();
            queue.push_back(item);

            while let Some(item) = queue.pop_front() {
                validated.insert(&item.key);
                visited.insert(&item.key);

                match &item.parent {
                    Some(parent) => {
                        if visited.contains(parent) {
                            return Err(crate::Error::new(format!(
                                "Infinite recursion error with item {:?} and parent {:?}",
                                item.key.as_str(),
                                parent.as_str()
                            )));
                        }

                        match self.items.get(parent) {
                            Some(parent) => queue.push_back(parent),
                            None => continue,
                        }
                    }
                    None => continue,
                }
            }

            if let Some(parent) = &item.parent
                && !self.items.contains_key(parent)
            {
                return Err(crate::Error::new(format!("Missing parent: {parent}")));
            }
        }

        let mut key_path_map = std::collections::HashMap::new();

        for (key, item) in self.items.iter() {
            let key = key.to_owned();
            let path = recursive_build_path(&item.path, &item.parent, &self.items).to_path_buf();
            key_path_map.insert(key, path);
        }

        for (key, path) in key_path_map.into_iter() {
            if let Some(item_args) = self.items.get_mut(&key) {
                item_args.path = path;
            }
        }

        let mut key_path_map = std::collections::HashMap::new();

        for (key, item) in self.items.iter() {
            key_path_map.insert(key, &item.path);

            let path: &std::path::Path = item.path.as_ref();
            let parent_path_items = parent_resolved_path_items_map
                .entry(path.parent())
                .or_insert(Vec::new());

            if visited_paths.contains(&Some(path)) {
                continue;
            }

            visited_paths.insert(Some(path));
            let name = match path.file_name() {
                Some(name) => name.to_string_lossy(),
                None => path.to_string_lossy(),
            };
            parent_path_items.push(PathItem {
                path: Tokens::new(&name)?,
                parent: None,
                permission: item.permission,
                owner: item.owner,
                path_type: item.path_type,
                deferred: item.deferred,
                metadata: item.metadata.clone(),
            });

            let mut path: &std::path::Path = item.path.as_ref();

            while let Some(parent) = path.parent() {
                if visited_paths.contains(&Some(path)) {
                    path = parent;
                    continue;
                }
                visited_paths.insert(Some(path));

                let parent_path_items = {
                    if parent.components().next_back().is_some() {
                        parent_resolved_path_items_map
                            .entry(Some(parent))
                            .or_insert(Vec::new())
                    } else {
                        visited_paths.insert(None);
                        parent_resolved_path_items_map
                            .entry(None)
                            .or_insert(Vec::new())
                    }
                };

                let name = match path.file_name() {
                    Some(name) => name.to_string_lossy(),
                    None => path.to_string_lossy(),
                };

                parent_path_items.push(PathItem {
                    path: Tokens::new(&name)?,
                    parent: None,
                    permission: crate::Permission::default(),
                    owner: crate::Owner::default(),
                    path_type: crate::PathType::default(),
                    deferred: false,
                    metadata: std::collections::HashMap::new(),
                });

                path = parent;
            }

            let parent_path_items = parent_resolved_path_items_map
                .entry(None)
                .or_insert(Vec::new());

            if !visited_paths.contains(&None) {
                let name = match path.file_name() {
                    Some(name) => name.to_string_lossy(),
                    None => path.to_string_lossy(),
                };
                parent_path_items.push(PathItem {
                    path: Tokens::new(&name)?,
                    parent: None,
                    permission: crate::Permission::default(),
                    owner: crate::Owner::default(),
                    path_type: crate::PathType::default(),
                    deferred: false,
                    metadata: std::collections::HashMap::new(),
                });

                visited_paths.insert(None);
            }
        }

        let mut parent_index_map = std::collections::HashMap::new();
        let mut items: Vec<PathItem> = Vec::new();
        let mut item_map: std::collections::HashMap<FieldKey, usize> =
            std::collections::HashMap::new();

        for (parent, parent_items) in parent_resolved_path_items_map.into_iter() {
            let parent_id = parent_index_map
                .get(&parent.map(|p| p.to_path_buf()))
                .copied();

            for mut parent_item in parent_items {
                parent_item.parent = parent_id;

                let path = match parent {
                    Some(p) => p.to_path_buf(),
                    None => std::path::PathBuf::new(),
                }
                .join(parent_item.path.to_string());
                parent_index_map.insert(Some(path), items.len());
                items.push(parent_item);
            }
        }

        for (key, path) in key_path_map {
            if let Some(parent_index) = parent_index_map.get(&Some(path.to_path_buf())) {
                item_map.insert(key.clone(), *parent_index);
            }
        }

        Ok(Config {
            resolvers: self.resolvers,
            items,
            item_map,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Owner, PathType, Permission};

    use super::*;

    #[rstest::rstest]
    #[case("test", None)]
    #[case("test", Some(r#".+"#))]
    fn test_config_builder_add_string_resolver_success(
        #[case] key: &str,
        #[case] pattern: Option<&str>,
    ) {
        ConfigBuilder::new()
            .add_string_resolver(key, pattern)
            .unwrap()
            .build()
            .unwrap();
    }

    #[test]
    fn test_config_builder_add_integer_resolver_success() {
        ConfigBuilder::new()
            .add_integer_resolver("test", 1)
            .unwrap()
            .build()
            .unwrap();
    }

    #[test]
    fn test_config_builder_add_path_item_success() {
        ConfigBuilder::new()
            .add_path_item(PathItemArgs {
                key: "key".try_into().unwrap(),
                path: "path".into(),
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
    }

    #[test]
    fn test_config_builder_add_path_item_with_parent_success() {
        ConfigBuilder::new()
            .add_path_item(PathItemArgs {
                key: "parent".try_into().unwrap(),
                path: "/parent/path".into(),
                parent: None,
                permission: Permission::default(),
                owner: Owner::default(),
                path_type: PathType::default(),
                deferred: false,
                metadata: std::collections::HashMap::new(),
            })
            .unwrap()
            .add_path_item(PathItemArgs {
                key: "child".try_into().unwrap(),
                path: "child/path".into(),
                parent: Some("parent".try_into().unwrap()),
                permission: Permission::default(),
                owner: Owner::default(),
                path_type: PathType::default(),
                deferred: false,
                metadata: std::collections::HashMap::new(),
            })
            .unwrap()
            .build()
            .unwrap();
    }

    #[test]
    fn test_config_builder_build_parent_with_multiple_children_success() {
        ConfigBuilder::new()
            .add_path_item(PathItemArgs {
                key: "parent".try_into().unwrap(),
                path: "/parent/path".into(),
                parent: None,
                permission: Permission::default(),
                owner: Owner::default(),
                path_type: PathType::default(),
                deferred: false,
                metadata: std::collections::HashMap::new(),
            })
            .unwrap()
            .add_path_item(PathItemArgs {
                key: "child1".try_into().unwrap(),
                path: "child1/path".into(),
                parent: Some("parent".try_into().unwrap()),
                permission: Permission::default(),
                owner: Owner::default(),
                path_type: PathType::default(),
                deferred: false,
                metadata: std::collections::HashMap::new(),
            })
            .unwrap()
            .add_path_item(PathItemArgs {
                key: "child2".try_into().unwrap(),
                path: "child2/path".into(),
                parent: Some("parent".try_into().unwrap()),
                permission: Permission::default(),
                owner: Owner::default(),
                path_type: PathType::default(),
                deferred: false,
                metadata: std::collections::HashMap::new(),
            })
            .unwrap()
            .build()
            .unwrap();
    }

    #[test]
    fn test_config_builder_build_failure_invalid_parent() {
        let err = ConfigBuilder::new()
            .add_path_item(PathItemArgs {
                key: "key".try_into().unwrap(),
                path: "path".into(),
                parent: Some("invalid".try_into().unwrap()),
                permission: Permission::default(),
                owner: Owner::default(),
                path_type: PathType::default(),
                deferred: false,
                metadata: std::collections::HashMap::new(),
            })
            .unwrap()
            .build()
            .unwrap_err();

        assert_eq!(err.to_string(), "Missing parent: invalid");
    }

    #[test]
    fn test_config_builder_build_failure_invalid_path() {
        let err = ConfigBuilder::new()
            .add_path_item(PathItemArgs {
                key: "child".try_into().unwrap(),
                path: "path".into(),
                parent: Some("parent".try_into().unwrap()),
                permission: Permission::default(),
                owner: Owner::default(),
                path_type: PathType::default(),
                deferred: false,
                metadata: std::collections::HashMap::new(),
            })
            .unwrap()
            .add_path_item(PathItemArgs {
                key: "parent".try_into().unwrap(),
                path: "/{123}parent".into(),
                parent: None,
                permission: Permission::default(),
                owner: Owner::default(),
                path_type: PathType::default(),
                deferred: false,
                metadata: std::collections::HashMap::new(),
            })
            .unwrap()
            .build()
            .unwrap_err();

        assert_eq!(err.to_string(), "Parse Error: Invalid variable");
    }

    #[test]
    fn test_config_builder_build_failure_infinite_recursion() {
        let err = ConfigBuilder::new()
            .add_path_item(PathItemArgs {
                key: "child".try_into().unwrap(),
                path: "child".into(),
                parent: Some("parent".try_into().unwrap()),
                permission: Permission::default(),
                owner: Owner::default(),
                path_type: PathType::default(),
                deferred: false,
                metadata: std::collections::HashMap::new(),
            })
            .unwrap()
            .add_path_item(PathItemArgs {
                key: "parent".try_into().unwrap(),
                path: "parent".into(),
                parent: Some("child".try_into().unwrap()),
                permission: Permission::default(),
                owner: Owner::default(),
                path_type: PathType::default(),
                deferred: false,
                metadata: std::collections::HashMap::new(),
            })
            .unwrap()
            .build()
            .unwrap_err();

        let err_msg = err.to_string();

        assert!(
            err_msg == "Infinite recursion error with item \"child\" and parent \"parent\""
                || err_msg == "Infinite recursion error with item \"parent\" and parent \"child\""
        )
    }

    #[test]
    fn test_config_get_item_success() {
        let config = ConfigBuilder::new()
            .add_path_item(PathItemArgs {
                key: "parent".try_into().unwrap(),
                path: "/parent/path".into(),
                parent: None,
                permission: Permission::default(),
                owner: Owner::default(),
                path_type: PathType::default(),
                deferred: false,
                metadata: std::collections::HashMap::new(),
            })
            .unwrap()
            .add_path_item(PathItemArgs {
                key: "child".try_into().unwrap(),
                path: "child/path".into(),
                parent: Some("parent".try_into().unwrap()),
                permission: Permission::default(),
                owner: Owner::default(),
                path_type: PathType::default(),
                deferred: false,
                metadata: std::collections::HashMap::new(),
            })
            .unwrap()
            .build()
            .unwrap();

        let item = config.get_item(&"child".try_into().unwrap()).unwrap();
        assert_eq!(item.len(), 5);
        assert_eq!(
            item.iter()
                .map(|i| i.path.to_string())
                .collect::<std::path::PathBuf>(),
            std::path::PathBuf::from("/parent/path/child/path")
        );
    }
}
