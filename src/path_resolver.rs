/// Resolve a path from a key and fields.
///
/// This will get a path to find in the filesystem or save to based on the input key and fields.
///
/// # Errors
///
/// - The key needs to be in the input config struct.
/// - The path variables need to be a subset of the fields. For example, `"{one}/{two}/{three}"` with
///   the fields `{"one": 1, "two": 2, "three": 3}` is valid, but `"{one}/{two}/{three}"` with the
///   fields `{"one": 1}` is invalid.
///
/// # Example
///
/// ```rust
/// # use openpathresolver::{ConfigBuilder, get_path, Owner, PathItemArgs, PathType, Permission};
/// let config = ConfigBuilder::new()
///     .add_path_item(PathItemArgs {
///         key: "key".try_into().unwrap(),
///         path: "/path/to/{thing}".into(),
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
/// let path = get_path(&config, "key", &fields).unwrap();
///
/// assert_eq!(path, std::path::PathBuf::from("/path/to/value"));
/// ```
pub fn get_path(
    config: &crate::Config,
    key: impl TryInto<crate::FieldKey, Error = crate::Error>,
    fields: &crate::types::PathAttributes,
) -> Result<std::path::PathBuf, crate::Error> {
    let key = key.try_into()?;
    let item = match config.get_item(&key) {
        Some(item) => item,
        None => {
            return Err(crate::Error::new(format!(
                "Could not find path from key: {key}"
            )));
        }
    };

    let mut path = std::path::PathBuf::new();
    let mut path_part = String::new();

    for part in item.iter() {
        part.path.draw(&mut path_part, fields, &config.resolvers)?;
        path.push(path_part.as_str());
        path_part.clear();
    }

    Ok(path)
}

/// Try to extract the fields from a key and path.
///
/// # Errors
///
/// - The key needs to be in the input config struct.
///
/// # Example
///
/// ```rust
/// # use openpathresolver::{ConfigBuilder, get_fields, Owner, PathItemArgs, PathType, Permission};
/// let config = ConfigBuilder::new()
///     .add_path_item(PathItemArgs {
///         key: "key".try_into().unwrap(),
///         path: "/path/to/{thing}".into(),
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
/// let path = std::path::PathBuf::from("/path/to/value");
/// let fields = get_fields(&config, "key", &path).unwrap();
///
/// let expected_fields = {
///     let mut fields = std::collections::HashMap::new();
///     fields.insert("thing".try_into().unwrap(), "value".into());
///
///     Some(fields)
/// };
///
/// assert_eq!(fields, expected_fields);
/// ```
pub fn get_fields(
    config: &crate::Config,
    key: impl TryInto<crate::FieldKey, Error = crate::Error>,
    path: impl AsRef<std::path::Path>,
) -> Result<Option<crate::types::PathAttributes>, crate::Error> {
    let key = key.try_into()?;
    let path = std::path::PathBuf::from(
        path.as_ref()
            .to_string_lossy()
            .replace("\\", "/")
            .replace("/", std::path::MAIN_SEPARATOR_STR),
    );
    let item = match config.get_item(&key) {
        Some(item) => item,
        None => {
            return Err(crate::Error::new(format!(
                "Could not find fields from key: {key}"
            )));
        }
    };
    let mut part_pattern = String::new();
    let mut fields = crate::types::PathAttributes::new();

    for (part, path_part) in item.iter().zip(path.iter()) {
        part_pattern.clear();
        part_pattern.push('^');
        part.path
            .draw_regex_pattern(&mut part_pattern, &config.resolvers)?;
        part_pattern.push('$');
        // TODO: cache this line - building regexes are expensive.
        let regex_pattern = regex::Regex::new(&part_pattern)?;
        let path_part_str = path_part.to_string_lossy();
        let captures = match regex_pattern.captures(&path_part_str) {
            Some(captures) => captures,
            None => return Ok(None),
        };

        let mut counter = 1;

        for token in part.path.tokens.iter() {
            if let crate::types::Token::Variable(key) = token {
                let captured = &captures[counter];
                let resolver = match config.resolvers.get(key) {
                    Some(resolver) => resolver,
                    None => &crate::Resolver::Default,
                };
                let value = resolver.to_path_value(captured)?;
                fields.insert(key.to_owned(), value);

                counter += 1;
            }
        }
    }

    Ok(Some(fields))
}

/// Find a key from a path and fields.
///
/// # Example
///
/// ```rust
/// # use openpathresolver::{ConfigBuilder, get_key, Owner, PathItemArgs, PathType, Permission};
/// let config = ConfigBuilder::new()
///     .add_path_item(PathItemArgs {
///         key: "key".try_into().unwrap(),
///         path: "/path/to/{thing}".into(),
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
/// let path = std::path::PathBuf::from("/path/to/value");
/// let key = get_key(&config, &path, &fields).unwrap();
///
/// assert_eq!(key.map(|k| k.as_str()), Some("key"));
/// ```
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

/// Find paths from a given key and fields.
///
/// This differs from the [get_path] because it will search the filesystem for the paths and the
/// fields do not need to be a superset of the path variables. If a path variable is missing, then
/// that will signify to the system to return all paths that match that variable's shape. For
/// example, if a user needs to find all of the versions of the "Widget" publish, and the structure
/// of the path looks like `"{root}/publishes/{entity}/{version}"`, then the only required fields
/// will be `root` and `entity`.
///
/// # Example
///
/// ```rust
/// # use openpathresolver::{ConfigBuilder, find_paths, Owner, PathItemArgs, PathType, Permission};
/// let config = ConfigBuilder::new()
///     .add_path_item(PathItemArgs {
///         key: "key".try_into().unwrap(),
///         path: "/path/to/{thing}".into(),
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
/// find_paths(&config, "key", &fields);
/// ```
pub fn find_paths(
    config: &crate::Config,
    key: impl TryInto<crate::FieldKey, Error = crate::Error>,
    fields: &crate::types::PathAttributes,
) -> Result<Vec<std::path::PathBuf>, crate::Error> {
    let key = key.try_into()?;
    let item = match config.get_item(&key) {
        Some(item) => item,
        None => {
            return Err(crate::Error::new(format!(
                "Could not find paths from key: {key}"
            )));
        }
    };

    let mut regex_pattern = String::new();
    let mut glob_path = std::path::PathBuf::new();

    regex_pattern.push('^');

    for (index, part) in item.iter().enumerate() {
        let mut regex_part = String::new();
        part.path
            .draw_regex_pattern(&mut regex_part, &config.resolvers)?;

        let value = if part.path.has_variable_tokens() {
            part.path.try_to_literal_token(fields, &config.resolvers)?
        } else {
            part.path.clone()
        };

        let mut glob_part = String::new();
        value.draw_glob_pattern(&mut glob_part)?;

        regex_pattern.push_str(&regex_part);

        if index != item.len() - 1 && !regex_pattern.ends_with(r"[\\/]") {
            regex_pattern.push_str(r"[\\/]");
        }

        glob_path.push(glob_part);
    }

    regex_pattern.push('$');

    let compiled_regex = regex::Regex::new(&regex_pattern)?;
    let mut out_paths = Vec::new();

    for result in glob::glob(glob_path.to_string_lossy().as_ref())? {
        let path = result?;

        if compiled_regex.is_match(path.to_string_lossy().as_ref()) {
            out_paths.push(path);
        }
    }

    Ok(out_paths)
}

#[cfg(test)]
mod tests {
    use crate::{Owner, PathItemArgs, PathType, Permission};

    use super::*;

    #[test]
    fn test_get_path_success() {
        let config = crate::ConfigBuilder::new()
            .add_path_item(PathItemArgs {
                key: "key".try_into().unwrap(),
                path: "/path/to/{thing}".into(),
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

        let path = get_path(&config, "key", &fields).unwrap();

        assert_eq!(path, std::path::PathBuf::from("/path/to/value"));
    }

    #[test]
    fn test_get_fields_success() {
        let config = crate::ConfigBuilder::new()
            .add_path_item(PathItemArgs {
                key: "key".try_into().unwrap(),
                path: "/path/to/{thing}".into(),
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
            .add_path_item(PathItemArgs {
                key: "key".try_into().unwrap(),
                path: "/path/to/{thing}".into(),
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
            .add_path_item(PathItemArgs {
                key: "root".try_into().unwrap(),
                path: root_dir.to_path_buf(),
                parent: None,
                permission: Permission::default(),
                owner: Owner::default(),
                path_type: PathType::default(),
                deferred: false,
                metadata: std::collections::HashMap::new(),
            })
            .unwrap()
            .add_path_item(PathItemArgs {
                key: "key".try_into().unwrap(),
                path: "path/to/{thing}_{frame}.txt".into(),
                parent: Some("root".try_into().unwrap()),
                permission: Permission::default(),
                owner: Owner::default(),
                path_type: PathType::File,
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

        let mut result_paths = find_paths(&config, "key", &fields).unwrap();
        result_paths.sort();

        assert_eq!(expected_paths, result_paths);
    }
}
