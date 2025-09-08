mod config;
mod entity;
mod field_key;
mod path_item;
mod resolver;
mod token;
mod value;

pub(crate) type PathAttributes = std::collections::HashMap<FieldKey, PathValue>;
pub(crate) type TemplateAttributes = std::collections::HashMap<FieldKey, TemplateValue>;
pub(crate) type Resolvers = std::collections::HashMap<FieldKey, Resolver>;

pub use config::{Config, ConfigBuilder};
pub use entity::{PathEntity, TemplateEntity};
pub use field_key::FieldKey;
pub use path_item::{CopyFile, Owner, Permission, ResolvedPathItem};
pub(crate) use path_item::{PathItem, PathItemBuilder};
pub use resolver::Resolver;
pub(crate) use token::{Token, Tokens};
pub use value::{PathValue, TemplateValue};
