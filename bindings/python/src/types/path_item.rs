use pyo3::prelude::*;

#[derive(Clone, Debug)]
#[pyclass]
pub struct CopyFile {
    pub(crate) inner: base_openpathresolver::CopyFile,
}

#[pymethods]
impl CopyFile {
    #[staticmethod]
    fn new_none() -> Self {
        Self {
            inner: base_openpathresolver::CopyFile::None,
        }
    }

    #[staticmethod]
    fn new_path(path: &str) -> Self {
        Self {
            inner: base_openpathresolver::CopyFile::Path(std::path::PathBuf::from(path)),
        }
    }

    #[staticmethod]
    fn new_template(value: &str) -> PyResult<Self> {
        Ok(Self {
            inner: base_openpathresolver::CopyFile::Template(crate::to_py_result(
                base_openpathresolver::FieldKey::try_from(value),
            )?),
        })
    }
}

#[derive(Clone, Debug)]
#[pyclass]
pub struct Owner {
    pub(crate) inner: base_openpathresolver::Owner,
}

#[pymethods]
impl Owner {
    #[staticmethod]
    fn new_inherit() -> Self {
        Self {
            inner: base_openpathresolver::Owner::Inherit,
        }
    }

    #[staticmethod]
    fn new_root() -> Self {
        Self {
            inner: base_openpathresolver::Owner::Root,
        }
    }

    #[staticmethod]
    fn new_project() -> Self {
        Self {
            inner: base_openpathresolver::Owner::Project,
        }
    }

    #[staticmethod]
    fn new_user() -> Self {
        Self {
            inner: base_openpathresolver::Owner::User,
        }
    }
}

#[derive(Clone, Debug)]
#[pyclass]
pub struct Permission {
    pub(crate) inner: base_openpathresolver::Permission,
}

#[pymethods]
impl Permission {
    #[staticmethod]
    fn new_inherit() -> Self {
        Self {
            inner: base_openpathresolver::Permission::Inherit,
        }
    }

    #[staticmethod]
    fn new_read_only() -> Self {
        Self {
            inner: base_openpathresolver::Permission::ReadOnly,
        }
    }

    #[staticmethod]
    fn new_read_write() -> Self {
        Self {
            inner: base_openpathresolver::Permission::ReadWrite,
        }
    }
}

#[derive(Clone, Debug)]
#[pyclass]
pub struct ResolvedPathItem {
    pub(crate) inner: base_openpathresolver::ResolvedPathItem,
}

#[pymethods]
impl ResolvedPathItem {
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
        Permission {
            inner: self.inner.permission().to_owned(),
        }
    }

    pub fn owner(&self) -> Owner {
        Owner {
            inner: self.inner.owner().to_owned(),
        }
    }

    pub fn copy_file(&self) -> CopyFile {
        CopyFile {
            inner: self.inner.copy_file().to_owned(),
        }
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
    pub(crate) copy_file: CopyFile,
    pub(crate) deferred: bool,
}

#[pymethods]
impl PathItem {
    #[new]
    fn new(
        key: String,
        path: std::path::PathBuf,
        parent: Option<String>,
        permission: Permission,
        owner: Owner,
        copy_file: CopyFile,
        deferred: bool,
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
            copy_file,
            deferred,
        })
    }
}
