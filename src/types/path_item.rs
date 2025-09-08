use crate::types::{FieldKey, Tokens};

#[derive(Debug)]
pub(crate) struct PathItemBuilder {
    pub(crate) key: FieldKey,
    pub(crate) value: std::path::PathBuf,
    pub(crate) parent: Option<FieldKey>,
    pub(crate) permission: Permission,
    pub(crate) owner: Owner,
    pub(crate) copy_file: CopyFile,
    pub(crate) deferred: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct PathItem {
    pub(crate) value: Tokens,
    pub(crate) parent: Option<usize>,
    pub(crate) permission: Permission,
    pub(crate) owner: Owner,
    pub(crate) copy_file: CopyFile,
    pub(crate) deferred: bool,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ResolvedPathItem {
    pub(crate) key: Option<FieldKey>,
    pub(crate) value: std::path::PathBuf,
    pub(crate) permission: Permission,
    pub(crate) owner: Owner,
    pub(crate) copy_file: CopyFile,
    pub(crate) deferred: bool,
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

    pub fn copy_file(&self) -> &CopyFile {
        &self.copy_file
    }

    pub fn deferred(&self) -> bool {
        self.deferred
    }
}

#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize)]
pub enum Permission {
    #[default]
    Inherit,
    ReadOnly,
    ReadWrite,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum Owner {
    #[default]
    Inherit,
    Root,
    Project,
    User,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum CopyFile {
    #[default]
    None,
    Path(std::path::PathBuf),
    Template(crate::FieldKey),
}
