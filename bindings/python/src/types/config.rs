use pyo3::{exceptions::PyNotImplementedError, prelude::*};

use crate::errors::to_py_error;

#[derive(Debug, Clone)]
#[pyclass]
pub struct Config {
    pub(crate) inner: base_openpathresolver::Config,
}

#[pymethods]
impl Config {
    #[new]
    fn new<'py>(
        resolvers: std::collections::HashMap<String, Bound<'py, PyAny>>,
        path_items: Bound<'py, PyAny>,
        templates: std::collections::HashMap<String, String>,
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
            } else if resolver.is_instance_of::<crate::EntityResolver>() {
                let resolver = resolver.extract::<crate::EntityResolver>()?;
                builder = builder
                    .add_entity_resolver(key, &resolver.key)
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

            builder = builder
                .add_path_item(
                    path_item.key,
                    path_item.path,
                    path_item.parent.as_ref().map(|v| v.inner.as_str()),
                    &path_item.permission.inner,
                    &path_item.owner.inner,
                    &path_item.copy_file.inner,
                    path_item.deferred,
                )
                .map_err(|err| to_py_error(&err))?;
        }

        for (key, template) in templates {
            builder = builder
                .add_template(&key, &template)
                .map_err(|err| to_py_error(&err))?;
        }

        let config = builder.build().map_err(|err| to_py_error(&err))?;

        Ok(Self { inner: config })
    }

    fn write_template(
        &self,
        key: &str,
        template_fields: std::collections::HashMap<String, crate::TemplateValue>,
    ) -> PyResult<String> {
        let mut converted_template_fields =
            std::collections::HashMap::with_capacity(template_fields.len());

        for (key, value) in template_fields {
            let key: base_openpathresolver::FieldKey =
                key.try_into().map_err(|err| to_py_error(&err))?;
            let value = value.inner;

            converted_template_fields.insert(key, value);
        }

        self.inner
            .write_template_to_string(key, &converted_template_fields)
            .map_err(|err| to_py_error(&err))
    }
}
