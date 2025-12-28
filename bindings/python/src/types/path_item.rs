use pyo3::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[pyclass(eq, eq_int, frozen, hash)]
pub enum PathType {
    Directory,
    File,
    FileTemplate,
}

impl From<base_openpathresolver::PathType> for PathType {
    fn from(value: base_openpathresolver::PathType) -> Self {
        match value {
            base_openpathresolver::PathType::Directory => Self::Directory,
            base_openpathresolver::PathType::File => Self::File,
            base_openpathresolver::PathType::FileTemplate => Self::FileTemplate,
        }
    }
}

impl From<PathType> for base_openpathresolver::PathType {
    fn from(value: PathType) -> Self {
        match value {
            PathType::Directory => Self::Directory,
            PathType::File => Self::File,
            PathType::FileTemplate => Self::FileTemplate,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[pyclass(eq, eq_int, frozen, hash)]
pub enum Owner {
    Inherit,
    Root,
    Project,
    User,
}

impl From<base_openpathresolver::Owner> for Owner {
    fn from(value: base_openpathresolver::Owner) -> Self {
        match value {
            base_openpathresolver::Owner::Inherit => Self::Inherit,
            base_openpathresolver::Owner::Root => Self::Root,
            base_openpathresolver::Owner::Project => Self::Project,
            base_openpathresolver::Owner::User => Self::User,
        }
    }
}

impl From<Owner> for base_openpathresolver::Owner {
    fn from(value: Owner) -> Self {
        match value {
            Owner::Inherit => Self::Inherit,
            Owner::Root => Self::Root,
            Owner::Project => Self::Project,
            Owner::User => Self::User,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[pyclass(eq, eq_int, frozen, hash)]
pub enum Permission {
    Inherit,
    ReadOnly,
    ReadWrite,
}

impl From<base_openpathresolver::Permission> for Permission {
    fn from(value: base_openpathresolver::Permission) -> Self {
        match value {
            base_openpathresolver::Permission::Inherit => Self::Inherit,
            base_openpathresolver::Permission::ReadOnly => Self::ReadOnly,
            base_openpathresolver::Permission::ReadWrite => Self::ReadWrite,
        }
    }
}

impl From<Permission> for base_openpathresolver::Permission {
    fn from(value: Permission) -> Self {
        match value {
            Permission::Inherit => Self::Inherit,
            Permission::ReadOnly => Self::ReadOnly,
            Permission::ReadWrite => Self::ReadWrite,
        }
    }
}

#[derive(Clone)]
#[pyclass]
pub struct ResolvedPathItem {
    pub(crate) inner: base_openpathresolver::ResolvedPathItem,
}

impl std::fmt::Debug for ResolvedPathItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

#[pymethods]
impl ResolvedPathItem {
    pub fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    pub fn key(&self) -> Option<&str> {
        match self.inner.key() {
            Some(key) => Some(key.as_str()),
            None => None,
        }
    }

    pub fn value(&self) -> &std::path::Path {
        self.inner.value()
    }

    pub fn permission(&self) -> Permission {
        Permission::from(self.inner.permission().to_owned())
    }

    pub fn owner(&self) -> Owner {
        Owner::from(self.inner.owner().to_owned())
    }

    pub fn path_type(&self) -> PathType {
        PathType::from(self.inner.path_type().to_owned())
    }

    pub fn deferred(&self) -> bool {
        self.inner.deferred()
    }
}

#[derive(Debug, Clone)]
#[pyclass]
pub struct PathItem {
    pub(crate) key: crate::FieldKey,
    pub(crate) path: std::path::PathBuf,
    pub(crate) parent: Option<crate::FieldKey>,
    pub(crate) permission: Permission,
    pub(crate) owner: Owner,
    pub(crate) path_type: PathType,
    pub(crate) deferred: bool,
    pub(crate) metadata: std::collections::HashMap<String, crate::MetadataValue>,
}

#[pymethods]
impl PathItem {
    #[allow(clippy::too_many_arguments)]
    #[new]
    fn new(
        key: String,
        path: std::path::PathBuf,
        parent: Option<String>,
        permission: Permission,
        owner: Owner,
        path_type: PathType,
        deferred: bool,
        metadata: std::collections::HashMap<String, crate::MetadataValue>,
    ) -> PyResult<Self> {
        let key = crate::FieldKey::try_from(key)?;

        let parent = match parent {
            Some(parent) => Some(crate::FieldKey::try_from(parent)?),
            None => None,
        };

        Ok(Self {
            key,
            path,
            parent,
            permission,
            owner,
            path_type,
            deferred,
            metadata,
        })
    }
}
