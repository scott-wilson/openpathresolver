use pyo3::prelude::*;

mod errors;
mod path_resolver;
mod types;
mod workspace_resolver;

pub(crate) use errors::to_py_result;
pub use errors::Error;
pub use path_resolver::{find_paths, get_fields, get_key, get_path};
pub use types::{
    Config, FieldKey, IntegerResolver, MetadataValue, Owner, PathItem, PathType, PathValue,
    Permission, ResolvedPathItem, StringResolver, TemplateValue,
};
pub use workspace_resolver::{create_workspace, get_workspace};

#[pymodule(gil_used = false)]
pub mod openpathresolver {
    // Errors
    #[pymodule_export]
    use super::Error;

    // Types
    #[pymodule_export]
    use super::{
        Config, FieldKey, IntegerResolver, Owner, PathItem, PathType, Permission, ResolvedPathItem,
        StringResolver,
    };

    // Functions
    #[pymodule_export]
    use super::{create_workspace, find_paths, get_fields, get_key, get_path, get_workspace};
}
