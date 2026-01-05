use pyo3::prelude::*;

use crate::errors::to_py_error;

type PathAttributes = std::collections::HashMap<String, crate::PathValue>;
type TemplateAttributes = std::collections::HashMap<String, crate::TemplateValue>;

/// Build a workspace by creating the files and folders for the given fields.
///
/// The create workspace function will use the `path_fields` to decide if a path should be built or
/// not. In other words, this will create paths that can be resolved with the path fields, but
/// other paths will not be created.
///
/// Args:
///     config: The config to build the workspace from.
///     path_fields: The fields to fill the path placeholders with. If a path includes a
///         placeholder that does not include a field, then the path will not be built.
///     template_fields: The fields used to fill file templates with.
///     io_function: This function is responsible for actually creating the workspace and defining
///         the rules of the workspace based on the config.
#[pyfunction]
pub fn create_workspace<'py>(
    py: Python<'py>,
    config: crate::Config,
    path_fields: PathAttributes,
    template_fields: TemplateAttributes,
    io_function: Py<PyAny>,
) -> PyResult<Bound<'py, PyAny>> {
    let path_fields = crate::path_resolver::convert_fields_from_wrapper(path_fields)?;
    let template_fields = convert_fields_from_wrapper(template_fields)?;

    struct CreateWorkspaceIoFunctionWrapper(Py<PyAny>, pyo3_async_runtimes::TaskLocals);

    #[async_trait::async_trait]
    impl base_openpathresolver::CreateWorkspaceIoFunction for CreateWorkspaceIoFunctionWrapper {
        async fn call(
            &self,
            config: std::sync::Arc<base_openpathresolver::Config>,
            template_fields: std::sync::Arc<
                std::collections::HashMap<
                    base_openpathresolver::FieldKey,
                    base_openpathresolver::TemplateValue,
                >,
            >,
            path_item: base_openpathresolver::ResolvedPathItem,
        ) -> Result<(), base_openpathresolver::Error> {
            // TODO: This is probably not efficient. Could probably get the values straight from
            // the create_workspace args.
            let template_fields = std::sync::Arc::unwrap_or_clone(template_fields);
            let template_fields = convert_fields_from_base(template_fields)
                .map_err(|err| base_openpathresolver::Error::new(err.to_string()))?;
            Python::attach(|py| -> PyResult<_> {
                let awaitable = self.0.call(
                    py,
                    (
                        crate::Config { inner: config },
                        template_fields,
                        crate::ResolvedPathItem { inner: path_item },
                    ),
                    None,
                )?;
                pyo3_async_runtimes::into_future_with_locals(&self.1, awaitable.bind(py).clone())
            })
            .map_err(|err| base_openpathresolver::Error::new(err.to_string()))?
            .await
            .map_err(|err| base_openpathresolver::Error::new(err.to_string()))?;

            Ok(())
        }
    }

    let task_locals = pyo3_async_runtimes::tokio::get_current_locals(py)?;
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        base_openpathresolver::create_workspace(
            config.inner.clone(),
            &path_fields,
            std::sync::Arc::new(template_fields),
            CreateWorkspaceIoFunctionWrapper(io_function, task_locals),
        )
        .await
        .map_err(|err| to_py_error(&err))
    })
}

/// Get all of the path items that would be created with the :code:`create_workspace` function.
///
/// The only paths that will be returned are paths that can be fully resolved with the given path
/// fields.
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
        let value = base_openpathresolver::TemplateValue::from(value);

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
        let value = crate::TemplateValue::from(value);

        converted_fields.insert(key, value);
    }

    Ok(converted_fields)
}
