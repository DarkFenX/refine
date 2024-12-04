use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(in crate::cmd) struct HItemMutation {
    pub(in crate::cmd) mutator_id: rc::EItemId,
    // Workaround for https://github.com/serde-rs/serde/issues/1183
    #[serde_as(as = "std::collections::HashMap<serde_with::DisplayFromStr, _>")]
    pub(in crate::cmd) attrs: HashMap<rc::EAttrId, HItemAttrMutation>,
}
impl Into<rc::SolItemMutation> for &HItemMutation {
    fn into(self) -> rc::SolItemMutation {
        rc::SolItemMutation::new_with_attrs(
            self.mutator_id,
            self.attrs.iter().map(|(k, v)| (*k, v.into())).collect(),
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
