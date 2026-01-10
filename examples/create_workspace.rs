//! This is an example of creating a workspace.
//!
//! The create workspace function can be called at any point in the lifecycle of a project and
//! should not destroy paths that already exist.

#[tokio::main]
async fn main() {
    let tmp_dir = tempfile::tempdir().unwrap();

    // First, the config will need to be initialized. openpathresolver intentionally does not
    // include support for a config file such as yaml, json, etc because we assume that the calling
    // code has its own config format that we can use.
    let config = openpathresolver::ConfigBuilder::new()
        // Path items are parts of the project structure. These are used both in the create
        // workspace and the get path functions.
        .add_path_item(openpathresolver::PathItemArgs {
            // The key is only allowed to be simple text such as variable identifiers in most
            // languages. The FieldKey type ensures the key is the correct format.
            key: "root".try_into().unwrap(),
            // This is the path element. Anything between { and } will be looked up in the
            // fields and use the placeholder resolvers to decide how to serialize the value.
            path: "{root}".into(),
            // This is the root most item.
            parent: None,
            // The permission model of the openpathresolver is very simple. Paths can be read only,
            // read/write, or inherit. If it is inherit, then the path will automatically pick the
            // permission of its parent. If there are no permissions, then the IO function will
            // have to decide what inherit means. Otherwise the IO function will decide what read
            // only or read/write means given the context that the calling code should know.
            permission: openpathresolver::Permission::default(),
            // The owner model is like the permission model, except the possible values are root,
            // project, user, and inherit. These values are also meaningless to openpathresolver,
            // and is up to the calling code to decide what these might mean.
            owner: openpathresolver::Owner::default(),
            // The path type is either a directory, file, or file template. It is up to the calling
            // code and the IO function to create a directory or file. If the type is file
            // template, then the IO function and calling code can use any templating engine it
            // prefers to create a file with a given template.
            path_type: openpathresolver::PathType::default(),
            // If a path is deferred, then it will not be generated unless a child path is not
            // deferred and can be resolved.
            deferred: false,
            // Extra metadata that might be useful for the IO function such as the path to copy the
            // file from.
            metadata: std::collections::HashMap::new(),
        })
        .unwrap()
        // Examples of items that depends on the root key.
        .add_path_item(openpathresolver::PathItemArgs {
            key: "key1".try_into().unwrap(),
            path: "path/to/{thing}".into(),
            parent: Some("root".try_into().unwrap()),
            permission: openpathresolver::Permission::default(),
            owner: openpathresolver::Owner::default(),
            path_type: openpathresolver::PathType::default(),
            deferred: false,
            metadata: std::collections::HashMap::new(),
        })
        .unwrap()
        .add_path_item(openpathresolver::PathItemArgs {
            key: "key2".try_into().unwrap(),
            path: "path/to/a/{thing}".into(),
            parent: Some("root".try_into().unwrap()),
            permission: openpathresolver::Permission::default(),
            owner: openpathresolver::Owner::default(),
            path_type: openpathresolver::PathType::default(),
            deferred: false,
            metadata: std::collections::HashMap::new(),
        })
        .unwrap()
        .add_path_item(openpathresolver::PathItemArgs {
            key: "key3".try_into().unwrap(),
            path: "path/to/b/{thing}".into(),
            parent: Some("root".try_into().unwrap()),
            permission: openpathresolver::Permission::default(),
            owner: openpathresolver::Owner::default(),
            path_type: openpathresolver::PathType::default(),
            deferred: false,
            metadata: std::collections::HashMap::new(),
        })
        .unwrap()
        // This item's placeholder is not specified in the fields below, so it shouldn't be
        // generated.
        .add_path_item(openpathresolver::PathItemArgs {
            key: "key4".try_into().unwrap(),
            path: "path/to/c/{not_specified}".into(),
            parent: Some("root".try_into().unwrap()),
            permission: openpathresolver::Permission::default(),
            owner: openpathresolver::Owner::default(),
            path_type: openpathresolver::PathType::default(),
            deferred: false,
            metadata: std::collections::HashMap::new(),
        })
        .unwrap()
        // This item's placeholder is an integer (specified with the add_integer_resolver). If a
        // placeholder is not defined, then it is automatically assumed to be an unstructured
        // string. Otherwise, the placeholder resolvers can define what the shape of the
        // placeholder's value should be.
        .add_path_item(openpathresolver::PathItemArgs {
            key: "key5".try_into().unwrap(),
            path: "path/to/d/{some_int}".into(),
            parent: Some("root".try_into().unwrap()),
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
    let path_fields = {
        let mut fields = std::collections::HashMap::new();
        fields.insert(
            "root".try_into().unwrap(),
            tmp_dir.path().to_string_lossy().as_ref().into(),
        );
        fields.insert("thing".try_into().unwrap(), "value".into());
        fields.insert("some_int".try_into().unwrap(), 12u8.into());

        fields
    };
    // The template fields are like the path fields, but are used in the file templates to resolve
    // those.
    let template_fields = {
        let mut fields = std::collections::HashMap::new();
        fields.insert("thing".try_into().unwrap(), "value".into());

        fields
    };

    // A simple implementation of the IO function.
    struct Func;

    #[async_trait::async_trait]
    impl openpathresolver::CreateWorkspaceIoFunction for Func {
        async fn call(
            &self,
            _config: std::sync::Arc<openpathresolver::Config>,
            _template_fields: std::sync::Arc<
                std::collections::HashMap<
                    openpathresolver::FieldKey,
                    openpathresolver::TemplateValue,
                >,
            >,
            path_item: openpathresolver::ResolvedPathItem,
        ) -> Result<(), openpathresolver::Error> {
            // In this case, we are expecting all the paths to be directories, and are ignoring
            // permissions, ownership, etc. Just create the directories.
            std::fs::create_dir_all(path_item.value())?;
            Ok(())
        }
    }

    openpathresolver::create_workspace(
        std::sync::Arc::new(config),
        &path_fields,
        std::sync::Arc::new(template_fields),
        Func,
    )
    .await
    .unwrap();

    // Print the result. This should return something like the following:
    //
    // ```
    // {root}/
    // {root}/path
    // {root}/path/to
    // {root}/path/a
    // {root}/path/a/value
    // {root}/path/b
    // {root}/path/b/value
    // {root}/path/c  # The {not_specified} is not in the fields map, so it cannot be resolved.
    // {root}/path/d
    // {root}/path/d/012
    // ```
    //
    for entry in walkdir::WalkDir::new(tmp_dir.path()) {
        let entry = entry.unwrap();
        println!("{}", entry.path().to_string_lossy());
    }
}
