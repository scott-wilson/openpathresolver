use pyo3::prelude::*;

// TODO: Replace this with something to include extra useful information such as
// the field key that raised the error. There might be a nice way to do this in
// a future pyo3. See https://github.com/PyO3/pyo3/issues/295 for more info.

pyo3::create_exception!(
    path_resolver,
    Error,
    pyo3::exceptions::PyException,
    "Error for the workspace or path resolvers."
);

pub(crate) fn to_py_error(err: &base_openpathresolver::Error) -> PyErr {
    Error::new_err(err.to_string())
}

pub(crate) fn to_py_result<T>(result: Result<T, base_openpathresolver::Error>) -> PyResult<T> {
    match result {
        Ok(value) => Ok(value),
        Err(err) => Err(to_py_error(&err)),
    }
}
