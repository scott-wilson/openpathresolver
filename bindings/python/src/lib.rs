//! Find paths based on a structured query or build out a tree in the filesystem.
//!
//! The open path resolver contains two parts. The path resolver is responsible for finding paths
//! or the elements used to find a path. This can be used either to get where to save a file or
//! where to try to find a file to load.
//!
//! The workspace resolver is responsible for building out a tree for a given query. For example,
//! if there is a location to save published elements and workspaces for building out the elements,
//! and a user needs to build out the "Widget" element, then the system can automatically build out
//! the file and folder structure needed for the user to do their work.

#![deny(missing_docs)]
#![deny(rustdoc::missing_crate_level_docs)]
#![deny(unsafe_code)]

use pyo3::prelude::*;

mod errors;
mod path_resolver;
mod types;
mod workspace_resolver;

pub use errors::Error;
pub(crate) use errors::to_py_result;
pub use path_resolver::{find_paths, get_fields, get_key, get_path};
pub use types::{
    Config, FieldKey, IntegerResolver, MetadataValue, Owner, PathItem, PathType, PathValue,
    Permission, ResolvedPathItem, StringResolver, TemplateValue,
};
pub use workspace_resolver::{create_workspace, get_workspace};

/// Find paths based on a structured query or build out a tree in the filesystem.
///
/// The open path resolver contains two parts. The path resolver is responsible for finding paths
/// or the elements used to find a path. This can be used either to get where to save a file or
/// where to try to find a file to load.
///
/// The workspace resolver is responsible for building out a tree for a given query. For example,
/// if there is a location to save published elements and workspaces for building out the elements,
/// and a user needs to build out the "Widget" element, then the system can automatically build out
/// the file and folder structure needed for the user to do their work.
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
