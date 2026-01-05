use pyo3::prelude::*;

/// The type of the path.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[pyclass(eq, eq_int, frozen, hash)]
pub enum PathType {
    /// The path is a directory.
    Directory,
    /// The path is a file.
    File,
    /// The path is a file, but the file this is sourcing is a template.
    ///
    /// The workspace resolver's IO function is responsible for how to resolve the template.
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

/// The owner of the path.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[pyclass(eq, eq_int, frozen, hash)]
pub enum Owner {
    /// This should inherit from the parent path.
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

/// The permission for a path.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[pyclass(eq, eq_int, frozen, hash)]
pub enum Permission {
    /// This should inherit from the parent path.
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

/// The path item that has been validated and resolved in the config.
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
    #[allow(missing_docs)]
    pub fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    /// The key for the path.
    pub fn key(&self) -> Option<&str> {
        match self.inner.key() {
            Some(key) => Some(key.as_str()),
            None => None,
        }
    }

    /// The fully resolved path.
    ///
    /// This may not be a fully absolute path because the root can be defined as a variable.
    pub fn value(&self) -> &std::path::Path {
        self.inner.value()
    }

    /// The permission for the path.
    ///
    /// There should be no inherited permissions at this point unless no permissions have been
    /// defined at all. Then it is recommended to have the workspace resolver decide what the "root" permission should be.
    pub fn permission(&self) -> Permission {
        Permission::from(self.inner.permission().to_owned())
    }

    /// The owner of the path.
    ///
    /// There should be no inherited owner at this point, unless no owner has been defined at all.
    /// Then it is recommended to have the workspace resolver decide what the "root" owner should
    /// be.
    pub fn owner(&self) -> Owner {
        Owner::from(self.inner.owner().to_owned())
    }

    /// The type of the path.
    ///
    /// It is assumed that all of the elements except the last will be directories. If the path is
    /// a file, then the workspace resolver should use simple logic to create a file (such as
    /// copying and pasting from another location). The file template can use a template engine
    /// defined in the create workspace's IO function.
    pub fn path_type(&self) -> PathType {
        PathType::from(self.inner.path_type().to_owned())
    }

    /// Whether the path is deferred or not.
    pub fn deferred(&self) -> bool {
        self.inner.deferred()
    }

    /// Metadata for the workspace resolver.
    ///
    /// This could contain anything such as the specific user  that owns the path, or the source
    /// path to copy and paste the file from.
    pub fn metadata(&self) -> std::collections::HashMap<String, crate::MetadataValue> {
        self.inner
            .metadata()
            .iter()
            .map(|(k, v)| (k.clone(), crate::MetadataValue::from(v.clone())))
            .collect()
    }
}

/// Input path item arguments.
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
    /// Create a new path item to add to the config.
    ///
    /// Args:
    ///     key: The name of the field.
    ///     path: The path part that may or may not contain placeholders. For example, `path/to/{item}`.
    ///         It is recommended that all path parts are always relative, and to have the root of the
    ///         path defined as a placeholder.
    ///     parent: The parent path item's field key.
    ///     permission: The permission of the path and all child paths unless explicitly set.
    ///     owner: The owner of the path and all child paths unless explicitly set.
    ///     path_type: The path_type of the path and all child paths unless explicitly set.
    ///     deferred: Whether the path is deferred or not. If the path is deferred, then the
    ///         :code:`create_workspace` function will not resolve the path unless a subpath is
    ///         resolvable. For example, if the path `path/to/{thing}` is marked as deferred, then
    ///         even if the field `thing` is available when resolving the path, the path will not
    ///         be resolved. However, if the path has a non-defferred subpath that expands the
    ///         current path to `path/to/{thing}/some/{subthing}`, and both thing and subthing are
    ///         valid, then the path will be resolved.
    ///     metadata: Extra metadata for the arguments that may be useful, as as marking a path as
    ///     belonging to a specific user.
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
