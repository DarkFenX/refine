use std::collections::HashMap;

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub(in crate::cmd) enum HMutationOnAdd {
    Short(rc::EItemId),
    Full(HItemMutationFull),
}
impl Into<rc::SolItemAddMutation> for &HMutationOnAdd {
    fn into(self) -> rc::SolItemAddMutation {
        match self {
            HMutationOnAdd::Short(mutator_id) => rc::SolItemAddMutation::new(*mutator_id),
            HMutationOnAdd::Full(full_mutation) => full_mutation.into(),
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub(in crate::cmd) enum HMutationOnChange {
    AddShort(rc::EItemId),
    AddFull(HItemMutationFull),
    ChangeAttrs(HashMap<rc::EAttrId, Option<HItemAttrMutationValue>>),
}

#[serde_with::serde_as]
#[derive(serde_tuple::Deserialize_tuple)]
pub(in crate::cmd) struct HItemMutationFull {
    pub(in crate::cmd) mutator_id: rc::EItemId,
    // Workaround for https://github.com/serde-rs/serde/issues/1183
    #[serde_as(as = "Option<std::collections::HashMap<serde_with::DisplayFromStr, _>>")]
    pub(in crate::cmd) attrs: Option<HashMap<rc::EAttrId, HItemAttrMutationValue>>,
}
impl Into<rc::SolItemAddMutation> for &HItemMutationFull {
    fn into(self) -> rc::SolItemAddMutation {
        rc::SolItemAddMutation::new_with_attrs(
            self.mutator_id,
            self.attrs
                .as_ref()
                .map(|v| {
                    v.iter()
                        .map(|(k, v)| rc::SolItemAddAttrMutation::new(*k, v.into()))
                        .collect()
                })
                .unwrap_or_else(|| Vec::new()),
        )
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::cmd) enum HItemAttrMutationValue {
    Roll(rc::MutaRoll),
    Absolute(rc::AttrVal),
}
impl Into<rc::SolItemAttrMutationValue> for &HItemAttrMutationValue {
    fn into(self) -> rc::SolItemAttrMutationValue {
        match self {
            HItemAttrMutationValue::Roll(roll) => rc::SolItemAttrMutationValue::Roll(*roll),
            HItemAttrMutationValue::Absolute(absolute) => rc::SolItemAttrMutationValue::Absolute(*absolute),
        }
    }
}
