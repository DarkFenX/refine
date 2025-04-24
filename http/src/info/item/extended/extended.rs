use std::collections::HashMap;

use rc::ItemMutCommon;

use crate::shared::HEffectId;

use super::{HAttrVal, HEffect, HModificationInfo};

#[derive(serde::Serialize)]
pub(crate) struct HItemExtendedInfo {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub(crate) attrs: HashMap<rc::AttrId, HAttrVal>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub(crate) effects: HashMap<HEffectId, HEffect>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub(crate) mods: HashMap<rc::AttrId, Vec<HModificationInfo>>,
}
impl<T> From<&mut T> for HItemExtendedInfo
where
    T: ItemMutCommon,
{
    fn from(core_item: &mut T) -> Self {
        let attrs = match core_item.iter_attrs() {
            Ok(iter_attrs) => iter_attrs.map(|(k, v)| (k, HAttrVal::from(&v))).collect(),
            Err(_) => HashMap::new(),
        };
        let effects = match core_item.iter_effects() {
            Ok(iter_effects) => iter_effects.map(|(k, v)| (k.into(), HEffect::from(&v))).collect(),
            Err(_) => HashMap::new(),
        };
        let mods = match core_item.iter_modifiers() {
            Ok(iter_mods) => iter_mods
                .map(|(k, v)| (k, v.into_iter().map(|m| HModificationInfo::from(&m)).collect()))
                .collect(),
            Err(_) => HashMap::new(),
        };
        Self { attrs, effects, mods }
    }
}
