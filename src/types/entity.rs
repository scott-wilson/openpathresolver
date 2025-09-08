use crate::types::{FieldKey, PathAttributes, PathValue, TemplateAttributes, TemplateValue};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PathEntity {
    entity_id: String,
    entity_type: String,
    parent: Option<std::sync::Arc<PathEntity>>,
    attributes: PathAttributes,
}

impl std::hash::Hash for PathEntity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.entity_type.hash(state);
        self.entity_id.hash(state);
    }
}

impl std::cmp::PartialEq for PathEntity {
    fn eq(&self, other: &Self) -> bool {
        self.entity_id == other.entity_id
    }
}

impl std::cmp::Eq for PathEntity {}

impl PathEntity {
    pub fn new(
        entity_id: impl AsRef<str>,
        entity_type: impl AsRef<str>,
        attributes: impl std::iter::IntoIterator<Item = (FieldKey, PathValue)>,
        parent: Option<std::sync::Arc<PathEntity>>,
    ) -> Self {
        Self {
            entity_id: entity_id.as_ref().to_string(),
            entity_type: entity_type.as_ref().to_string(),
            attributes: attributes.into_iter().collect(),
            parent,
        }
    }

    pub fn entity_id(&self) -> &str {
        &self.entity_id
    }

    pub fn entity_type(&self) -> &str {
        &self.entity_type
    }

    pub fn parent(&self) -> Option<std::sync::Arc<PathEntity>> {
        self.parent.clone()
    }

    pub fn attributes(&self) -> &PathAttributes {
        &self.attributes
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TemplateEntity {
    entity_id: String,
    entity_type: String,
    parent: Option<std::sync::Arc<TemplateEntity>>,
    attributes: TemplateAttributes,
}

impl std::cmp::PartialEq for TemplateEntity {
    fn eq(&self, other: &Self) -> bool {
        self.entity_id == other.entity_id
    }
}

impl std::cmp::Eq for TemplateEntity {}

impl TemplateEntity {
    pub fn new(
        entity_id: impl AsRef<str>,
        entity_type: impl AsRef<str>,
        attributes: impl std::iter::IntoIterator<Item = (FieldKey, TemplateValue)>,
        parent: Option<std::sync::Arc<TemplateEntity>>,
    ) -> Self {
        Self {
            entity_id: entity_id.as_ref().to_string(),
            entity_type: entity_type.as_ref().to_string(),
            attributes: attributes.into_iter().collect(),
            parent,
        }
    }

    pub fn entity_id(&self) -> &str {
        &self.entity_id
    }

    pub fn entity_type(&self) -> &str {
        &self.entity_type
    }

    pub fn parent(&self) -> Option<std::sync::Arc<TemplateEntity>> {
        self.parent.clone()
    }

    pub fn attributes(&self) -> &TemplateAttributes {
        &self.attributes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn arb_path_values() -> impl Strategy<Value = PathValue> {
        prop_oneof![
            any::<u16>().prop_map(PathValue::from),
            ".*".prop_map(PathValue::from),
        ]
    }

    fn arb_field_keys() -> impl Strategy<Value = FieldKey> {
        "[[:alpha:]]+".prop_map(|s| s.try_into().unwrap())
    }

    fn arb_template_values() -> impl Strategy<Value = TemplateValue> {
        let leaf = prop_oneof![
            Just(TemplateValue::None),
            any::<bool>().prop_map(TemplateValue::Bool),
            any::<i64>().prop_map(TemplateValue::Integer),
            any::<f64>().prop_map(TemplateValue::Float),
            ".*".prop_map(TemplateValue::String),
        ];
        leaf.prop_recursive(8, 256, 10, |inner| {
            prop_oneof![
                prop::collection::vec(inner.clone(), 0..10).prop_map(TemplateValue::Array),
                prop::collection::hash_map(arb_field_keys(), inner.clone(), 0..10)
                    .prop_map(TemplateValue::Object),
                (
                    ".*",
                    ".*",
                    prop::collection::hash_map(arb_field_keys(), inner.clone(), 0..10)
                )
                    .prop_map(|(id, type_, attributes)| {
                        TemplateValue::Entity(TemplateEntity::new(id, type_, attributes, None))
                    }),
            ]
        })
    }

    fn arb_path_attributes() -> impl Strategy<Value = PathAttributes> {
        prop::collection::hash_map(arb_field_keys(), arb_path_values(), 0..10)
    }

    fn arb_template_attributes() -> impl Strategy<Value = TemplateAttributes> {
        prop::collection::hash_map(arb_field_keys(), arb_template_values(), 0..10)
    }

    fn arb_path_entity() -> impl Strategy<Value = PathEntity> {
        let leaf = (".*", ".*", arb_path_attributes())
            .prop_map(|(id, type_, attributes)| PathEntity::new(id, type_, attributes, None));

        leaf.prop_recursive(8, 256, 10, |inner| {
            (".*", ".*", arb_path_attributes(), inner).prop_map(
                |(id, type_, attributes, parent)| {
                    PathEntity::new(id, type_, attributes, Some(std::sync::Arc::new(parent)))
                },
            )
        })
    }

    fn arb_template_entity() -> impl Strategy<Value = TemplateEntity> {
        let leaf = (".*", ".*", arb_template_attributes())
            .prop_map(|(id, type_, attributes)| TemplateEntity::new(id, type_, attributes, None));

        leaf.prop_recursive(8, 256, 10, |inner| {
            (".*", ".*", arb_template_attributes(), inner).prop_map(
                |(id, type_, attributes, parent)| {
                    TemplateEntity::new(id, type_, attributes, Some(std::sync::Arc::new(parent)))
                },
            )
        })
    }

    proptest! {
        #[test]
        fn test_path_entity_new_success(
            entity_id in ".*",
            entity_type in ".*",
            attributes in arb_path_attributes(),
            parent in prop_oneof![
                Just(None),
                arb_path_entity().prop_map(Some)
            ]
        ) {
            let parent = parent.map(std::sync::Arc::new);
            let entity = PathEntity::new(&entity_id, &entity_type, attributes.clone(), parent.clone());

            prop_assert_eq!(entity.entity_id(), entity_id);
            prop_assert_eq!(entity.entity_type(), entity_type);
            prop_assert_eq!(entity.parent(), parent);
            prop_assert_eq!(entity.attributes(), &attributes);
        }
    }

    proptest! {
        #[test]
        fn test_template_entity_new_success(
            entity_id in ".*",
            entity_type in ".*",
            attributes in arb_template_attributes(),
            parent in prop_oneof![
                Just(None),
                arb_template_entity().prop_map(Some)
                ]
            ) {
            let parent = parent.map(std::sync::Arc::new);
            let entity = TemplateEntity::new(&entity_id, &entity_type, attributes.clone(), parent.clone());

            prop_assert_eq!(entity.entity_id(), entity_id);
            prop_assert_eq!(entity.entity_type(), entity_type);
            prop_assert_eq!(entity.parent(), parent);
            prop_assert_eq!(entity.attributes(), &attributes);
        }
    }
}
