use crate::types::{FieldKey, Tokens};

/// Input path item arguments
///
/// This struct is used in the [ConfigBuilder::add_path_item](crate::ConfigBuilder::add_path_item)
/// as input arguments.
#[derive(Debug)]
pub struct PathItemArgs {
    /// The name of the field.
    pub key: FieldKey,
    /// The path part that may or may not contain placeholders. For example, `path/to/{item}`. It
    /// is recommended that all path parts are always relative, and to have the root of the path
    /// defined as a placeholder.
    pub path: std::path::PathBuf,
    /// The parent path item's field key.
    pub parent: Option<FieldKey>,
    /// The permission of the path and all child paths unless explicitly set.
    pub permission: Permission,
    /// The owner of the path and all child paths unless explicitly set.
    pub owner: Owner,
    /// The path of the path and all child paths unless explicitly set.
    pub path_type: PathType,
    /// Whether the path is deferred or not. If a path is deferred, then the
    /// [create_workspace][crate::create_workspace] will not resolve the path unless a subpath is
    /// resolvable. For example, if the path `path/to/{thing}` is marked as deferred, then even if
    /// the field `thing` is available when resolving the path, the path will not be resolved.
    /// However, if the path has a non-defferred subpath that expands the current path to
    /// `path/to/{thing}/some/{subthing}`, and both thing and subthing are valid, then the path
    /// will be resolved.
    pub deferred: bool,
    /// Extra metadata for the arguments that may be useful, such as marking a path as belonging to
    /// a specific user.
    pub metadata: std::collections::HashMap<String, crate::MetadataValue>,
}

#[derive(Debug, Clone)]
pub(crate) struct PathItem {
    pub(crate) path: Tokens,
    pub(crate) parent: Option<usize>,
    pub(crate) permission: Permission,
    pub(crate) owner: Owner,
    pub(crate) path_type: PathType,
    pub(crate) deferred: bool,
    pub(crate) metadata: std::collections::HashMap<String, crate::MetadataValue>,
}

/// The path item that has been validated and resolved in the config.
#[derive(Debug, Clone)]
pub struct ResolvedPathItem {
    pub(crate) key: Option<FieldKey>,
    pub(crate) value: std::path::PathBuf,
    pub(crate) permission: Permission,
    pub(crate) owner: Owner,
    pub(crate) path_type: PathType,
    pub(crate) deferred: bool,
    pub(crate) metadata: std::collections::HashMap<String, crate::MetadataValue>,
}

impl ResolvedPathItem {
    /// The key for the path.
    pub fn key(&self) -> Option<&FieldKey> {
        match &self.key {
            Some(key) => Some(key),
            None => None,
        }
    }

    /// The fully resolved path.
    ///
    /// This may not be a fully absolute path because the root can be defined as a variable.
    pub fn value(&self) -> &std::path::Path {
        &self.value
    }

    /// The permission for the path.
    ///
    /// There should be no inherited permissions at this point unless no permissions have been
    /// defined at all. Then it is recommended to have the workspace resolver decide what the "root" permission should be.
    pub fn permission(&self) -> &Permission {
        &self.permission
    }

    /// The owner of the path.
    ///
    /// There should be no inherited owner at this point, unless no owner has been defined at all.
    /// Then it is recommended to have the workspace resolver decide what the "root" owner should
    /// be.
    pub fn owner(&self) -> &Owner {
        &self.owner
    }

    /// The type of the path.
    ///
    /// It is assumed that all of the elements except the last will be directories. If the path is
    /// a file, then the workspace resolver should use simple logic to create a file (such as
    /// copying and pasting from another location). The file template can use a template engine
    /// defined in the create workspace's IO function.
    pub fn path_type(&self) -> &PathType {
        &self.path_type
    }

    /// Whether the path is deferred or not.
    pub fn deferred(&self) -> bool {
        self.deferred
    }

    /// Metadata for the workspace resolver.
    ///
    /// This could contain anything such as the specific user  that owns the path, or the source
    /// path to copy and paste the file from.
    pub fn metadata(&self) -> &std::collections::HashMap<String, crate::MetadataValue> {
        &self.metadata
    }
}

/// The permission for a path.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum Permission {
    /// This should inherit from the parent path.
    #[default]
    Inherit,
    /// This path should be read only.
    ///
    /// The workspace resolver's IO function is responsible for deciding what read only means for a
    /// path.
    ReadOnly,
    /// This path should be read and write.
    ///
    /// The workspace resolver's IO function is responsible for deciding what read and write means
    /// for a path.
    ReadWrite,
}

/// The owner of the path.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum Owner {
    /// This should inherit from the parent path.
    #[default]
    Inherit,
    /// This path should be owned by the "root" user.
    ///
    /// The workspace resolver's IO function is responsible for deciding who the root owner is.
    /// This may or may not be the root user of the operating system.
    Root,
    /// This path should be owned by the "project" user.
    ///
    /// The workspace resolver's IO function is responsible for deciding who the project owner is.
    Project,
    /// This path should be owned by a specific user.
    ///
    /// This is useful for workspaces where users will do their work in and should easily be able
    /// to read/write to.
    User,
}

/// The type of the path.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum PathType {
    /// The path is a directory.
    #[default]
    Directory,
    /// The path is a file.
    File,
    /// The path is a file, but the file this is sourcing is a template.
    ///
    /// The workspace resolver's IO function is responsible for how to resolve the template.
    FileTemplate,
}
