use pyo3::{exceptions::PyNotImplementedError, prelude::*};

use crate::errors::to_py_error;

/// Store the resolver configs.
///
/// The config stores two major components. The resolvers, which are responsible for resolving the
/// placholder values, and the items, which are all of the path parts that are used to find paths
/// or used to create paths.
#[derive(Debug, Clone)]
#[pyclass]
pub struct Config {
    pub(crate) inner: std::sync::Arc<base_openpathresolver::Config>,
}

#[pymethods]
impl Config {
    /// Create a new config.
    ///
    /// Args:
    ///     resolvers: The resolvers for the placeholders. If a resolver is not supplied, then the
    ///         placholder will default to a simple string resolver.
    ///     path_items: The path item arguments to resolve.
    #[new]
    fn new<'py>(
        resolvers: std::collections::HashMap<String, Bound<'py, PyAny>>,
        path_items: Bound<'py, PyAny>,
    ) -> PyResult<Self> {
        let mut builder = base_openpathresolver::ConfigBuilder::new();

        for (key, resolver) in resolvers {
            if resolver.is_instance_of::<crate::StringResolver>() {
                let resolver = resolver.extract::<crate::StringResolver>()?;
                builder = builder
                    .add_string_resolver(key, resolver.pattern.as_deref())
                    .map_err(|err| to_py_error(&err))?;
            } else if resolver.is_instance_of::<crate::IntegerResolver>() {
                let resolver = resolver.extract::<crate::IntegerResolver>()?;
                builder = builder
                    .add_integer_resolver(key, resolver.padding)
                    .map_err(|err| to_py_error(&err))?;
            } else {
                return Err(PyNotImplementedError::new_err(format!(
                    "{} is not implemented.",
                    resolver.str()?
                )));
            }
        }

        for path_item in path_items.try_iter()? {
            let path_item = path_item?.extract::<crate::PathItem>()?;
            let parent = match path_item.parent {
                Some(p) => Some(p.try_into().map_err(|err| to_py_error(&err))?),
                None => None,
            };

            builder = builder
                .add_path_item(base_openpathresolver::PathItemArgs {
                    key: path_item.key.inner,
                    path: path_item.path,
                    parent,
                    permission: path_item.permission.into(),
                    owner: path_item.owner.into(),
                    path_type: path_item.path_type.into(),
                    deferred: path_item.deferred,
                    metadata: path_item
                        .metadata
                        .into_iter()
                        .map(|(k, v)| (k, v.inner))
                        .collect(),
                })
                .map_err(|err| to_py_error(&err))?;
        }

        let config = builder.build().map_err(|err| to_py_error(&err))?;

        Ok(Self {
            inner: std::sync::Arc::new(config),
        })
    }
}
