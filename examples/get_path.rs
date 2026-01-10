//! This is an example of getting a path.
//!
//! The get_path function is extremely useful when trying to save a file to a location with a
//! specific naming structure, or getting a path to a file with that naming structure.

fn main() {
    // First, the config will need to be initialized. openpathresolver intentionally does not
    // include support for a config file such as yaml, json, etc because we assume that the calling
    // code has its own config format that we can use.
    let config = openpathresolver::ConfigBuilder::new()
        .add_path_item(openpathresolver::PathItemArgs {
            // The key is only allowed to be simple text such as variable identifiers in most
            // languages. The FieldKey type ensures the key is the correct format.
            key: "key".try_into().unwrap(),
            // This is the path element. Anything between { and } will be looked up in the
            // fields and use the placeholder resolvers to decide how to serialize the value. If a
            // placeholder is not defined, then it is automatically assumed to be an unstructured
            // string. Otherwise, the placeholder resolvers can define what the shape of the
            // placeholder's value should be.
            path: "/path/to/{thing}/{some_int}".into(),
            // This is the root most item.
            parent: None,
            // The following fields can be ignored, since they are not useful for the get_path
            // function.
            permission: openpathresolver::Permission::default(),
            owner: openpathresolver::Owner::default(),
            path_type: openpathresolver::PathType::default(),
            deferred: false,
            metadata: std::collections::HashMap::new(),
        })
        .unwrap()
        // This marks values such as "001", "012", "123", and "1234" as valid. But "01" and "1"
        // would not be valid.
        .add_integer_resolver("some_int", 3)
        .unwrap()
        // Build the config. At this point, the config cannot be modified and would have to be
        // rebuilt.
        .build()
        .unwrap();

    // The path fields is a mapping between placeholders and values for the placeholders.
    let fields = {
        let mut fields = std::collections::HashMap::new();
        fields.insert("thing".try_into().unwrap(), "value".into());
        fields.insert("some_int".try_into().unwrap(), 12u8.into());

        fields
    };

    let path = openpathresolver::get_path(&config, "key", &fields).unwrap();

    assert_eq!(path, std::path::PathBuf::from("/path/to/value/012"));
}
