use crate::types::PathItem;

pub fn get_path(
    config: &crate::Config,
    key: impl TryInto<crate::FieldKey, Error = crate::Error>,
    fields: &crate::types::PathAttributes,
) -> Result<std::path::PathBuf, crate::Error> {
    let key = key.try_into()?;
    let item = match config.get_item(&key) {
        Some(item) => item,
        None => return Err(crate::Error::MissingItemError(key.clone())),
    };

    let mut path = std::path::PathBuf::new();
    let mut path_part = String::new();

    for part in item.iter() {
        part.value.draw(&mut path_part, fields, &config.resolvers)?;
        path.push(path_part.as_str());
        path_part.clear();
    }

    Ok(path)
}

pub fn get_fields(
    config: &crate::Config,
    key: impl TryInto<crate::FieldKey, Error = crate::Error>,
    path: impl AsRef<std::path::Path>,
) -> Result<Option<crate::types::PathAttributes>, crate::Error> {
    let key = key.try_into()?;
    let path = path.as_ref();
    let item = match config.get_item(&key) {
        Some(item) => item,
        None => return Err(crate::Error::MissingItemError(key.clone())),
    };

    let mut path_pattern = Vec::new();
    let mut counter = 0usize;
    let mut id_field_map = std::collections::HashMap::new();

    for part in item.iter() {
        let mut path_part = String::new();
        part.value
            .draw_regex_pattern(&mut path_part, &config.resolvers)?;
        path_pattern.push(path_part);

        for token in &part.value.tokens {
            if let crate::types::Token::Variable(key) = token {
                id_field_map.insert(counter, key);
                counter += 1;
            }
        }
    }

    let mut fields = crate::types::PathAttributes::new();

    for (path_part, pattern_part) in path.iter().zip(path_pattern.iter()) {
        let path_part = path_part.to_string_lossy();
        // TODO: cache this line - building regexes are expensive.
        let pattern_part = regex::Regex::new(&format!("^{}$", pattern_part))?;
        let captures = match pattern_part.captures(&path_part) {
            Some(captures) => captures,
            None => return Ok(None),
        };

        for (index, matching_pattern) in captures.iter().skip(1).enumerate() {
            let matching_pattern = match matching_pattern {
                Some(matching_pattern) => matching_pattern,
                None => continue,
            };
            let field_key = *id_field_map.get(&index).unwrap();
            let resolver = match config.resolvers.get(field_key) {
                Some(resolver) => resolver,
                None => &crate::Resolver::Default,
            };
            let value = resolver.to_path_value(matching_pattern.as_str())?;

            if let Some(other_value) = fields.get(field_key) {
                if &value != other_value {
                    return Err(crate::Error::MismatchedFieldError {
                        key: field_key.clone(),
                        value: value.clone(),
                        other_value: other_value.clone(),
                    });
                }
            }

            fields.insert(field_key.clone(), value);
        }
    }

    Ok(Some(fields))
}

pub fn get_key<'a>(
    config: &'a crate::Config,
    path: impl AsRef<std::path::Path>,
    fields: &crate::types::PathAttributes,
) -> Result<Option<&'a crate::FieldKey>, crate::Error> {
    let path = path.as_ref();

    for (key, _) in config.item_map.iter() {
        let other_path = get_path(config, key, fields)?;

        if path == other_path {
            return Ok(Some(key));
        }
    }

    Ok(None)
}

pub fn find_paths(
    config: &crate::Config,
    key: impl TryInto<crate::FieldKey, Error = crate::Error>,
    fields: &crate::types::PathAttributes,
) -> Result<Vec<std::path::PathBuf>, crate::Error> {
    let key = key.try_into()?;
    let item = match config.get_item(&key) {
        Some(item) => item,
        None => return Err(crate::Error::MissingItemError(key.clone())),
    };

    let mut path_pattern = Vec::new();

    for part in item.iter() {
        let mut path_part = String::new();
        part.value
            .draw_regex_pattern(&mut path_part, &config.resolvers)?;
        path_pattern.push(path_part);
    }

    let mut paths = Vec::new();

    fn recursive_find_paths(
        config: &crate::Config,
        fields: &crate::types::PathAttributes,
        root: &std::path::Path,
        elements: &[&PathItem],
        paths: &mut Vec<std::path::PathBuf>,
    ) -> Result<(), crate::Error> {
        let mut root = root.to_path_buf();

        for (index, element) in elements.iter().enumerate() {
            let mut value = element.value.clone();

            if value.has_variable_tokens() {
                value = value.try_to_literal_token(fields, &config.resolvers)?;
            }

            if value.has_variable_tokens() {
                if root.as_os_str().is_empty() {
                    return Err(crate::Error::VariableRootPathError);
                }

                let mut pattern = String::new();
                value.draw_regex_pattern(&mut pattern, &config.resolvers)?;
                // TODO: Cache this line - building regexes are expensive.
                let pattern = regex::Regex::new(&format!("^{}$", pattern))?;
                let sub_elements = elements.get(index + 1..).unwrap_or(&[]);

                for dir_entry in std::fs::read_dir(&root)? {
                    let dir_entry = dir_entry?;
                    let path = dir_entry.path();
                    let name = match path.file_name() {
                        Some(name) => name.to_string_lossy(),
                        None => continue,
                    };

                    if !pattern.is_match(&name) {
                        continue;
                    }

                    if sub_elements.is_empty() {
                        paths.push(path);
                        continue;
                    } else {
                        recursive_find_paths(config, fields, &root, sub_elements, paths)?;
                    }
                }

                break;
            } else {
                root.push(value.to_string());
            }
        }

        Ok(())
    }

    recursive_find_paths(
        config,
        fields,
        &std::path::PathBuf::new(),
        &item,
        &mut paths,
    )?;

    Ok(paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_path_success() {
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

        let path = get_path(&config, "key", &fields).unwrap();

        assert_eq!(path, std::path::PathBuf::from("/path/to/value"));
    }

    #[test]
    fn test_get_fields_success() {
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

        let expected_fields = {
            let mut fields = crate::types::PathAttributes::new();
            fields.insert("thing".try_into().unwrap(), "value".into());

            fields
        };

        let fields = get_fields(&config, "key", "/path/to/value")
            .unwrap()
            .unwrap();

        assert_eq!(fields, expected_fields);
    }

    #[test]
    fn test_get_key_success() {
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

        let result = get_key(&config, "/path/to/value", &fields)
            .unwrap()
            .unwrap();

        assert_eq!(result.to_string(), "key");
    }

    #[test]
    fn test_find_paths_success() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let root_dir = tmp_dir.path();
        let mut expected_paths = Vec::new();

        {
            let test_dir = root_dir.join("path/to");
            std::fs::create_dir_all(&test_dir).unwrap();

            for index in 0..5 {
                let task_test_dir = test_dir.clone();

                let path = task_test_dir.join(format!("value_{}.txt", index));
                std::fs::write(&path, "test").unwrap();
                expected_paths.push(path);
            }

            expected_paths.sort();
        }

        let config = crate::ConfigBuilder::new()
            .add_path_item(
                "root",
                root_dir,
                None,
                &crate::Permission::default(),
                &crate::Owner::default(),
                &crate::CopyFile::default(),
                false,
            )
            .unwrap()
            .add_path_item(
                "key",
                "path/to/{thing}_{frame}.txt",
                Some("root"),
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

        let mut result_paths = find_paths(&config, "key", &fields).unwrap();
        result_paths.sort();

        assert_eq!(expected_paths, result_paths);
    }
}
