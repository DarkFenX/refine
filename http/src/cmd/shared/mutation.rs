use std::collections::HashMap;

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub(in crate::cmd) enum HMutationOnAdd {
    Short(rc::EItemId),
    Full(HItemMutationFull),
}
impl From<&HMutationOnAdd> for rc::SolItemAddMutation {
    fn from(h_mutation: &HMutationOnAdd) -> Self {
        match h_mutation {
            HMutationOnAdd::Short(mutator_id) => Self::new(*mutator_id),
            HMutationOnAdd::Full(full_mutation) => full_mutation.into(),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
#[serde(untagged)]
pub(in crate::cmd) enum HMutationOnChange {
    AddShort(rc::EItemId),
    AddFull(HItemMutationFull),
    ChangeAttrs(
        #[serde_as(as = "std::collections::HashMap<serde_with::DisplayFromStr, _>")]
        HashMap<rc::EAttrId, Option<HItemAttrMutationValue>>,
    ),
}

#[serde_with::serde_as]
#[derive(serde_tuple::Deserialize_tuple)]
pub(in crate::cmd) struct HItemMutationFull {
    pub(in crate::cmd) mutator_id: rc::EItemId,
    // Workaround for https://github.com/serde-rs/serde/issues/1183
    #[serde_as(as = "Option<std::collections::HashMap<serde_with::DisplayFromStr, _>>")]
    pub(in crate::cmd) attrs: Option<HashMap<rc::EAttrId, HItemAttrMutationValue>>,
}
impl From<&HItemMutationFull> for rc::SolItemAddMutation {
    fn from(h_item_mutation: &HItemMutationFull) -> Self {
        Self::new_with_attrs(
            h_item_mutation.mutator_id,
            h_item_mutation
                .attrs
                .as_ref()
                .map(|v| {
                    v.iter()
                        .map(|(k, v)| rc::SolItemAddAttrMutation::new(*k, v.into()))
                        .collect()
                })
                .unwrap_or_default(),
        )
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::cmd) enum HItemAttrMutationValue {
    Roll(rc::MutaRoll),
    Absolute(rc::AttrVal),
}
impl From<&HItemAttrMutationValue> for rc::SolItemAttrMutationValue {
    fn from(h_mutation_value: &HItemAttrMutationValue) -> Self {
        match h_mutation_value {
            HItemAttrMutationValue::Roll(roll) => Self::Roll(*roll),
            HItemAttrMutationValue::Absolute(absolute) => Self::Absolute(*absolute),
        }
    }
}
