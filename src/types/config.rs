use crate::types::{
    CopyFile, FieldKey, Owner, PathItem, PathItemBuilder, Permission, Resolver, Resolvers, Tokens,
};

#[derive(Debug, Clone)]
pub struct Config {
    pub(crate) resolvers: Resolvers,
    pub(crate) item_map: std::collections::HashMap<FieldKey, usize>,
    pub(crate) items: Vec<PathItem>,
    pub(crate) template_map: std::collections::HashMap<FieldKey, String>,
}

impl Config {
    pub fn write_template_to_writer(
        &self,
        key: impl TryInto<FieldKey, Error = crate::Error>,
        template_fields: &crate::types::TemplateAttributes,
        writer: &mut impl std::io::Write,
    ) -> Result<(), crate::Error> {
        let key = key.try_into()?;

        let mut context = std::collections::HashMap::with_capacity(template_fields.len());

        for (template_key, template_value) in template_fields.iter() {
            context.insert(
                template_key.as_str(),
                minijinja::Value::from_serialize(template_value),
            );
        }

        let context = minijinja::Value::from(context);

        let template_str = match self.template_map.get(&key) {
            Some(t) => t,
            None => return Err(crate::Error::FieldError(key.to_string())),
        };

        let mut environment = minijinja::Environment::empty();

        environment.add_template(key.as_str(), template_str)?;
        let template = environment.get_template(key.as_str())?;

        template.render_to_write(context, writer)?;

        Ok(())
    }

    pub fn write_template_to_string(
        &self,
        key: impl TryInto<FieldKey, Error = crate::Error>,
        template_fields: &crate::types::TemplateAttributes,
    ) -> Result<String, crate::Error> {
        let key = key.try_into()?;

        let mut context = std::collections::HashMap::with_capacity(template_fields.len());

        for (template_key, template_value) in template_fields.iter() {
            context.insert(
                template_key.as_str(),
                minijinja::Value::from_serialize(template_value),
            );
        }

        let context = minijinja::Value::from(context);

        let template_str = match self.template_map.get(&key) {
            Some(t) => t,
            None => return Err(crate::Error::FieldError(key.to_string())),
        };

        let mut environment = minijinja::Environment::empty();

        environment.add_template(key.as_str(), template_str)?;
        let template = environment.get_template(key.as_str())?;

        Ok(template.render(context)?)
    }

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

#[derive(Debug, Default)]
pub struct ConfigBuilder {
    resolvers: Resolvers,
    items: std::collections::HashMap<FieldKey, PathItemBuilder>,
    template_map: std::collections::HashMap<FieldKey, String>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            resolvers: std::collections::HashMap::new(),
            items: std::collections::HashMap::new(),
            template_map: std::collections::HashMap::new(),
        }
    }

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

    pub fn add_integer_resolver(
        mut self,
        key: impl TryInto<crate::FieldKey, Error = crate::Error>,
        padding: u8,
    ) -> Result<Self, crate::Error> {
        self.resolvers
            .insert(key.try_into()?, Resolver::Integer { padding });
        Ok(self)
    }

    pub fn add_entity_resolver(
        mut self,
        key: impl TryInto<crate::FieldKey, Error = crate::Error>,
        entity: &str,
    ) -> Result<Self, crate::Error> {
        self.resolvers.insert(
            key.try_into()?,
            Resolver::Entity {
                key: entity.try_into()?,
            },
        );
        Ok(self)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_path_item(
        mut self,
        key: impl TryInto<crate::FieldKey, Error = crate::Error>,
        path: impl AsRef<std::path::Path>,
        parent: Option<&str>,
        permission: &Permission,
        owner: &Owner,
        copy_file: &CopyFile,
        deferred: bool,
    ) -> Result<Self, crate::Error> {
        let parent = match parent {
            Some(parent) => Some(parent.try_into()?),
            None => None,
        };
        let key: FieldKey = key.try_into()?;

        self.items.insert(
            key.clone(),
            PathItemBuilder {
                key,
                value: path.as_ref().to_path_buf(),
                parent,
                permission: *permission,
                owner: owner.clone(),
                copy_file: copy_file.clone(),
                deferred,
            },
        );
        Ok(self)
    }

    pub fn add_template(
        mut self,
        key: impl TryInto<crate::FieldKey, Error = crate::Error>,
        value: &str,
    ) -> Result<Self, crate::Error> {
        let key: FieldKey = key.try_into()?;
        self.template_map.insert(key, value.to_string());
        Ok(self)
    }

    pub fn build(self) -> Result<Config, crate::Error> {
        let mut items: Vec<PathItem> = Vec::new();
        let mut item_map: std::collections::HashMap<FieldKey, usize> =
            std::collections::HashMap::new();

        // Find items with parents that cause infinite recursion errors.
        let mut queue = std::collections::VecDeque::new();
        let mut visited = std::collections::HashSet::new();
        let mut validated = std::collections::HashSet::new();

        for (key, item) in self.items.iter() {
            if validated.contains(key) {
                continue;
            }

            queue.clear();
            visited.clear();
            queue.push_back(item);

            while !queue.is_empty() {
                let item = queue.pop_front().unwrap();
                validated.insert(&item.key);
                visited.insert(&item.key);

                match &item.parent {
                    Some(parent) => {
                        if visited.contains(parent) {
                            return Err(crate::Error::InfiniteRecursionError {
                                item: item.key.clone(),
                                parent: parent.clone(),
                            });
                        }

                        match self.items.get(parent) {
                            Some(parent) => queue.push_back(parent),
                            None => continue,
                        }
                    }
                    None => continue,
                }
            }
        }

        for (key, item) in self.items.iter() {
            if let Some(parent) = &item.parent {
                if !self.items.contains_key(parent) {
                    return Err(crate::Error::MissingParentError(parent.clone()));
                }
            }

            self.recursive_build_path_item(key, item, &mut items, &mut item_map)?;
        }

        Ok(Config {
            resolvers: self.resolvers,
            template_map: self.template_map,
            items,
            item_map,
        })
    }

    fn recursive_build_path_item(
        &self,
        key: &FieldKey,
        item: &PathItemBuilder,
        items: &mut Vec<PathItem>,
        key_map: &mut std::collections::HashMap<FieldKey, usize>,
    ) -> Result<usize, crate::Error> {
        let mut parent = match &item.parent {
            Some(parent_key) => match key_map.get(parent_key) {
                // Item already exists in key map.
                Some(parent_id) => Some(*parent_id),
                // Item doesn't exist in key map, so recursively build it.
                None => {
                    let parent = self.items.get(parent_key).unwrap();
                    let last_id =
                        self.recursive_build_path_item(parent_key, parent, items, key_map)?;

                    Some(last_id)
                }
            },
            None => None,
        };

        for component in item.value.iter() {
            let value = Tokens::new(&component.to_string_lossy())?;
            let permission = item.permission;
            let owner = item.owner.clone();
            let copy_file = item.copy_file.clone();
            let deferred = item.deferred;

            let path_item = PathItem {
                value,
                parent,
                permission,
                owner,
                copy_file,
                deferred,
            };
            items.push(path_item);

            parent = Some(items.len() - 1);
        }

        key_map.insert(key.clone(), items.len() - 1);

        Ok(items.len() - 1)
    }
}

#[cfg(test)]
mod tests {
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
    fn test_config_builder_add_entity_resolver_success() {
        ConfigBuilder::new()
            .add_entity_resolver("key", "entity")
            .unwrap()
            .build()
            .unwrap();
    }

    #[test]
    fn test_config_builder_add_path_item_success() {
        ConfigBuilder::new()
            .add_path_item(
                "key",
                "path",
                None,
                &Permission::default(),
                &Owner::default(),
                &CopyFile::default(),
                false,
            )
            .unwrap()
            .build()
            .unwrap();
    }

    #[test]
    fn test_config_builder_add_path_item_with_parent_success() {
        ConfigBuilder::new()
            .add_path_item(
                "parent",
                "/parent/path",
                None,
                &Permission::default(),
                &Owner::default(),
                &CopyFile::default(),
                false,
            )
            .unwrap()
            .add_path_item(
                "child",
                "child/path",
                Some("parent"),
                &Permission::default(),
                &Owner::default(),
                &CopyFile::default(),
                false,
            )
            .unwrap()
            .build()
            .unwrap();
    }

    #[test]
    fn test_config_builder_add_path_item_failure_invalid_parent_key() {
        let err = ConfigBuilder::new()
            .add_path_item(
                "key",
                "path",
                Some("!"),
                &Permission::default(),
                &Owner::default(),
                &CopyFile::default(),
                false,
            )
            .unwrap_err();

        match err {
            crate::Error::ParseError(msg) => {
                assert_eq!(msg, "Invalid field key");
            }
            _ => panic!("Unexpected error type."),
        }
    }

    #[test]
    fn test_config_builder_add_path_item_failure_invalid_key() {
        let err = ConfigBuilder::new()
            .add_path_item(
                "!",
                "path",
                None,
                &Permission::default(),
                &Owner::default(),
                &CopyFile::default(),
                false,
            )
            .unwrap_err();
        match err {
            crate::Error::ParseError(msg) => {
                assert_eq!(msg, "Invalid field key");
            }
            _ => panic!("Unexpected error type."),
        }
    }

    #[test]
    fn test_config_builder_build_parent_with_multiple_children_success() {
        ConfigBuilder::new()
            .add_path_item(
                "parent",
                "/parent/path",
                None,
                &Permission::default(),
                &Owner::default(),
                &CopyFile::default(),
                false,
            )
            .unwrap()
            .add_path_item(
                "child1",
                "child1/path",
                Some("parent"),
                &Permission::default(),
                &Owner::default(),
                &CopyFile::default(),
                false,
            )
            .unwrap()
            .add_path_item(
                "child2",
                "child2/path",
                Some("parent"),
                &Permission::default(),
                &Owner::default(),
                &CopyFile::default(),
                false,
            )
            .unwrap()
            .build()
            .unwrap();
    }

    #[test]
    fn test_config_builder_build_failure_invalid_parent() {
        let err = ConfigBuilder::new()
            .add_path_item(
                "key",
                "path",
                Some("invalid"),
                &Permission::default(),
                &Owner::default(),
                &CopyFile::default(),
                false,
            )
            .unwrap()
            .build()
            .unwrap_err();
        match err {
            crate::Error::MissingParentError(parent) => {
                assert_eq!(parent, "invalid".try_into().unwrap());
            }
            _ => panic!("Unexpected error type."),
        }
    }

    #[test]
    fn test_config_builder_build_failure_invalid_path() {
        let err = ConfigBuilder::new()
            .add_path_item(
                "child",
                "path",
                Some("parent"),
                &Permission::default(),
                &Owner::default(),
                &CopyFile::default(),
                false,
            )
            .unwrap()
            .add_path_item(
                "parent",
                "/{123}parent",
                None,
                &Permission::default(),
                &Owner::default(),
                &CopyFile::default(),
                false,
            )
            .unwrap()
            .build()
            .unwrap_err();

        match err {
            crate::Error::ParseError(msg) => {
                assert_eq!(msg, "Invalid variable");
            }
            _ => panic!("Unexpected error type."),
        }
    }

    #[test]
    fn test_config_builder_build_failure_infinite_recursion() {
        let err = ConfigBuilder::new()
            .add_path_item(
                "child",
                "child",
                Some("parent"),
                &Permission::default(),
                &Owner::default(),
                &CopyFile::default(),
                false,
            )
            .unwrap()
            .add_path_item(
                "parent",
                "parent",
                Some("child"),
                &Permission::default(),
                &Owner::default(),
                &CopyFile::default(),
                false,
            )
            .unwrap()
            .build()
            .unwrap_err();

        match err {
            crate::Error::InfiniteRecursionError { item, parent } => {
                let item = item.to_string();
                let parent = parent.to_string();

                if parent == "parent" {
                    assert_eq!(item.to_string(), "child");
                    assert_eq!(parent.to_string(), "parent");
                } else {
                    assert_eq!(item.to_string(), "parent");
                    assert_eq!(parent.to_string(), "child");
                }
            }
            _ => panic!("Unexpected error type."),
        }
    }

    #[test]
    fn test_config_builder_add_template_str_success() {
        ConfigBuilder::new()
            .add_template("key", "value")
            .unwrap()
            .build()
            .unwrap();
    }

    #[test]
    fn test_config_builder_add_template_str_failure_invalid_value() {
        let err = ConfigBuilder::new().add_template("key", "{{").unwrap_err();
        assert!(matches!(err, crate::Error::TemplateError(_)));
    }

    #[test]
    fn test_config_get_item_success() {
        let config = ConfigBuilder::new()
            .add_path_item(
                "parent",
                "/parent/path",
                None,
                &Permission::default(),
                &Owner::default(),
                &CopyFile::default(),
                false,
            )
            .unwrap()
            .add_path_item(
                "child",
                "child/path",
                Some("parent"),
                &Permission::default(),
                &Owner::default(),
                &CopyFile::default(),
                false,
            )
            .unwrap()
            .build()
            .unwrap();

        let item = config.get_item(&"child".try_into().unwrap()).unwrap();
        assert_eq!(item.len(), 5);
        assert_eq!(
            item.iter()
                .map(|i| i.value.to_string())
                .collect::<std::path::PathBuf>(),
            std::path::PathBuf::from("/parent/path/child/path")
        );
    }
}
