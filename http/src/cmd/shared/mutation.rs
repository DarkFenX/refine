use std::collections::HashMap;

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub(in crate::cmd) enum HItemMutation {
    Short(rc::EItemId),
    Full(HItemMutationFull),
}
impl Into<rc::SolItemMutation> for &HItemMutation {
    fn into(self) -> rc::SolItemMutation {
        match self {
            HItemMutation::Short(mutator_id) => rc::SolItemMutation::new(*mutator_id),
            HItemMutation::Full(full_mutation) => full_mutation.into(),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde_tuple::Deserialize_tuple)]
pub(in crate::cmd) struct HItemMutationFull {
    pub(in crate::cmd) mutator_id: rc::EItemId,
    // Workaround for https://github.com/serde-rs/serde/issues/1183
    #[serde_as(as = "Option<std::collections::HashMap<serde_with::DisplayFromStr, _>>")]
    pub(in crate::cmd) attrs: Option<HashMap<rc::EAttrId, HItemAttrMutation>>,
}
impl Into<rc::SolItemMutation> for &HItemMutationFull {
    fn into(self) -> rc::SolItemMutation {
        rc::SolItemMutation::new_with_attrs(
            self.mutator_id,
            self.attrs
                .as_ref()
                .map(|v| v.iter().map(|(k, v)| (*k, v.into())).collect())
                .unwrap_or_else(|| rc::util::StMap::new()),
        )
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::cmd) enum HItemAttrMutation {
    Roll(rc::MutaRoll),
    Absolute(rc::AttrVal),
}
impl Into<rc::SolItemAttrMutation> for &HItemAttrMutation {
    fn into(self) -> rc::SolItemAttrMutation {
        match self {
            HItemAttrMutation::Roll(roll) => rc::SolItemAttrMutation::Roll(*roll),
            HItemAttrMutation::Absolute(absolute) => rc::SolItemAttrMutation::Absolute(*absolute),
        }
    }
}
