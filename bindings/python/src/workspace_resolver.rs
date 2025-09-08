use pyo3::prelude::*;

use crate::errors::to_py_error;

type PathAttributes = std::collections::HashMap<String, crate::PathValue>;
type TemplateAttributes = std::collections::HashMap<String, crate::TemplateValue>;

#[pyfunction]
pub fn create_workspace<'py>(
    config: &crate::Config,
    path_fields: PathAttributes,
    template_fields: TemplateAttributes,
    io_function: Bound<'py, PyAny>,
) -> PyResult<()> {
    let io_function_wrapper = |_c: &base_openpathresolver::Config,
                               i: &base_openpathresolver::ResolvedPathItem,
                               a: &std::collections::HashMap<
        base_openpathresolver::FieldKey,
        base_openpathresolver::TemplateValue,
    >|
     -> Result<(), base_openpathresolver::Error> {
        let i = crate::ResolvedPathItem { inner: i.clone() };
        let a = convert_fields_from_base(a.clone())
            .map_err(|err| base_openpathresolver::Error::RuntimeError(err.to_string()))?;

        // TODO: I want to avoid having to copy the config every time the wrapper function is
        // called, which could be a lot.
        io_function
            .call((config.clone(), i, a), None)
            .map_err(|err| base_openpathresolver::Error::RuntimeError(err.to_string()))?;

        Ok(())
    };

    base_openpathresolver::create_workspace(
        &config.inner,
        &crate::path_resolver::convert_fields_from_wrapper(path_fields)?,
        &convert_fields_from_wrapper(template_fields)?,
        io_function_wrapper,
    )
    .map_err(|err| to_py_error(&err))
}

#[pyfunction]
pub fn get_workspace(
    config: &crate::Config,
    path_fields: PathAttributes,
) -> PyResult<Vec<crate::ResolvedPathItem>> {
    let result = base_openpathresolver::get_workspace(
        &config.inner,
        &crate::path_resolver::convert_fields_from_wrapper(path_fields)?,
    )
    .map_err(|err| to_py_error(&err))?;

    let mut converted_result = Vec::with_capacity(result.len());

    for path_item in result {
        converted_result.push(crate::ResolvedPathItem { inner: path_item });
    }

    Ok(converted_result)
}

pub(crate) fn convert_fields_from_wrapper(
    fields: TemplateAttributes,
) -> PyResult<
    std::collections::HashMap<
        base_openpathresolver::FieldKey,
        base_openpathresolver::TemplateValue,
    >,
> {
    let mut converted_fields = std::collections::HashMap::with_capacity(fields.len());

    for (key, value) in fields {
        let key =
            base_openpathresolver::FieldKey::try_from(key).map_err(|err| to_py_error(&err))?;
        let value = base_openpathresolver::TemplateValue::try_from(value)
            .map_err(|err| to_py_error(&err))?;

        converted_fields.insert(key, value);
    }

    Ok(converted_fields)
}

pub(crate) fn convert_fields_from_base(
    fields: std::collections::HashMap<
        base_openpathresolver::FieldKey,
        base_openpathresolver::TemplateValue,
    >,
) -> PyResult<std::collections::HashMap<crate::FieldKey, crate::TemplateValue>> {
    let mut converted_fields = std::collections::HashMap::with_capacity(fields.len());

    for (key, value) in fields {
        let key = crate::FieldKey::try_from(key)?;
        let value = crate::TemplateValue::try_from(value)?;

        converted_fields.insert(key, value);
    }

    Ok(converted_fields)
}
