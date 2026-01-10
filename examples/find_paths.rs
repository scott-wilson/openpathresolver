//! This is an example of finding paths.
//!
//! The find_paths function is extremely similar to get_path, but while get_path requires all of
//! the fields to be specified, leaving a field out of the map tells find_paths to return all paths
//! that match the placeholder's shape. For example, getting all of the versions of a specific
//! asset would be as simple as filling the fields map with all of the fields except the version.

fn main() {
    let tmp_dir = tempfile::tempdir().unwrap();
    let root_dir = tmp_dir.path();
    let mut expected_paths = Vec::new();

    {
        let test_dir = root_dir.join("path/to");
        std::fs::create_dir_all(&test_dir).unwrap();

        for index in 0..5 {
            let task_test_dir = test_dir.clone();

            let path = task_test_dir.join(format!("value_{:03}.txt", index));
            std::fs::write(&path, "test").unwrap();
            expected_paths.push(path);
        }

        expected_paths.sort();
    }

    // First, the config will need to be initialized. openpathresolver intentionally does not
    // include support for a config file such as yaml, json, etc because we assume that the calling
    // code has its own config format that we can use.
    let config = openpathresolver::ConfigBuilder::new()
        .add_path_item(openpathresolver::PathItemArgs {
            // The key is only allowed to be simple text such as variable identifiers in most
            // languages. The FieldKey type ensures the key is the correct format.
            key: "root".try_into().unwrap(),
            path: root_dir.to_path_buf(),
            // This is the root most item.
            parent: None,
            // The following fields can be ignored, since they are not useful for the find_paths
            // function.
            permission: openpathresolver::Permission::default(),
            owner: openpathresolver::Owner::default(),
            path_type: openpathresolver::PathType::default(),
            deferred: false,
            metadata: std::collections::HashMap::new(),
        })
        .unwrap()
        .add_path_item(openpathresolver::PathItemArgs {
            key: "key".try_into().unwrap(),
            // This is the path element. Anything between { and } will be looked up in the
            // fields and use the placeholder resolvers to decide how to serialize the value. If a
            // placeholder is not defined, then it is automatically assumed to be an unstructured
            // string. Otherwise, the placeholder resolvers can define what the shape of the
            // placeholder's value should be.
            path: "path/to/{thing}_{frame}.txt".into(),
            // This is a child of the root item.
            parent: Some("root".try_into().unwrap()),
            permission: openpathresolver::Permission::default(),
            owner: openpathresolver::Owner::default(),
            path_type: openpathresolver::PathType::File,
            deferred: false,
            metadata: std::collections::HashMap::new(),
        })
        .unwrap()
        // This marks values such as "001", "012", "123", and "1234" as valid. But "01" and "1"
        // would not be valid.
        .add_integer_resolver("frame", 3)
        .unwrap()
        // Build the config. At this point, the config cannot be modified and would have to be
        // rebuilt.
        .build()
        .unwrap();

    // The path fields is a mapping between placeholders and values for the placeholders.
    let fields = {
        let mut fields = std::collections::HashMap::new();
        fields.insert("thing".try_into().unwrap(), "value".into());
        // The frame field is intentionally left out to tell find_paths to find all of the
        // frames.

        fields
    };

    let mut result_paths = openpathresolver::find_paths(&config, "key", &fields).unwrap();
    result_paths.sort();

    // This should return the paths:
    // - {root}/path/to/value_000.txt
    // - {root}/path/to/value_001.txt
    // - {root}/path/to/value_002.txt
    // - {root}/path/to/value_003.txt
    // - {root}/path/to/value_004.txt
    // - {root}/path/to/value_005.txt

    assert_eq!(expected_paths, result_paths);
}
