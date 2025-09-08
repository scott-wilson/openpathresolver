use pyo3::prelude::*;

#[derive(Clone, Debug)]
#[pyclass]
pub struct PathEntity {
    pub(crate) inner: std::sync::Arc<base_openpathresolver::PathEntity>,
}

#[pymethods]
impl PathEntity {
    #[new]
    fn new<'py>(
        entity_id: &str,
        entity_type: &str,
        attributes: std::collections::HashMap<String, Bound<'py, PyAny>>,
        parent: Option<PathEntity>,
    ) -> PyResult<Self> {
        let attributes = {
            let mut new_attributes = std::collections::HashMap::new();

            for (key, value) in attributes {
                new_attributes.insert(
                    crate::errors::to_py_result(base_openpathresolver::FieldKey::try_from(key))?,
                    crate::PathValue::try_from(value)?.inner,
                );
            }

            new_attributes
        };
        let parent = match parent {
            Some(parent) => Some(parent.inner.clone()),
            None => None,
        };
        let inner = std::sync::Arc::new(base_openpathresolver::PathEntity::new(
            entity_id,
            entity_type,
            attributes,
            parent,
        ));

        Ok(Self { inner })
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn entity_id(&self) -> &str {
        self.inner.entity_id()
    }

    fn entity_type(&self) -> &str {
        self.inner.entity_type()
    }

    fn parent(&self) -> Option<PathEntity> {
        self.inner.parent().map(|parent| Self {
            inner: parent.clone(),
        })
    }

    fn attributes<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<std::collections::HashMap<String, Py<PyAny>>> {
        let mut attributes = std::collections::HashMap::new();

        for (key, value) in self.inner.attributes() {
            attributes.insert(key.to_string(), crate::path_value_to_py_object(py, value)?);
        }

        Ok(attributes)
    }
}

#[derive(Clone, Debug)]
#[pyclass]
pub struct TemplateEntity {
    pub(crate) inner: std::sync::Arc<base_openpathresolver::TemplateEntity>,
}

#[pymethods]
impl TemplateEntity {
    #[new]
    fn new<'py>(
        entity_id: &str,
        entity_type: &str,
        attributes: std::collections::HashMap<String, Bound<'py, PyAny>>,
        parent: Option<TemplateEntity>,
    ) -> PyResult<Self> {
        let attributes = {
            let mut new_attributes = std::collections::HashMap::new();

            for (key, value) in attributes {
                new_attributes.insert(
                    crate::errors::to_py_result(base_openpathresolver::FieldKey::try_from(key))?,
                    crate::TemplateValue::try_from(value)?.inner,
                );
            }

            new_attributes
        };
        let parent = match parent {
            Some(parent) => Some(parent.inner.clone()),
            None => None,
        };
        let inner = std::sync::Arc::new(base_openpathresolver::TemplateEntity::new(
            entity_id,
            entity_type,
            attributes,
            parent,
        ));

        Ok(Self { inner })
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn entity_id(&self) -> &str {
        self.inner.entity_id()
    }

    fn entity_type(&self) -> &str {
        self.inner.entity_type()
    }

    fn parent(&self) -> Option<TemplateEntity> {
        self.inner.parent().map(|parent| Self {
            inner: parent.clone(),
        })
    }

    fn attributes<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<std::collections::HashMap<&str, Py<PyAny>>> {
        let mut attributes = std::collections::HashMap::new();

        for (key, value) in self.inner.attributes() {
            attributes.insert(key.as_str(), crate::template_value_to_py_object(py, value)?);
        }

        Ok(attributes)
    }
}
