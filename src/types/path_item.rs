use crate::types::{FieldKey, Tokens};

#[derive(Debug)]
pub struct PathItemArgs {
    pub key: FieldKey,
    pub path: std::path::PathBuf,
    pub parent: Option<FieldKey>,
    pub permission: Permission,
    pub owner: Owner,
    pub path_type: PathType,
    pub deferred: bool,
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
    pub fn key(&self) -> Option<&FieldKey> {
        match &self.key {
            Some(key) => Some(key),
            None => None,
        }
    }

    pub fn value(&self) -> &std::path::Path {
        &self.value
    }

    pub fn permission(&self) -> &Permission {
        &self.permission
    }

    pub fn owner(&self) -> &Owner {
        &self.owner
    }

    pub fn path_type(&self) -> &PathType {
        &self.path_type
    }

    pub fn deferred(&self) -> bool {
        self.deferred
    }

    pub fn metadata(&self) -> &std::collections::HashMap<String, crate::MetadataValue> {
        &self.metadata
    }
}

#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum Permission {
    #[default]
    Inherit,
    ReadOnly,
    ReadWrite,
}

#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum Owner {
    #[default]
    Inherit,
    Root,
    Project,
    User,
}

#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum PathType {
    #[default]
    Directory,
    File,
    FileTemplate,
}
