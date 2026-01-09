use rc::ItemMutCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, Map, serde_as};

use super::{HAttrVals, HEffect, HModification};

#[serde_as]
#[derive(Serialize)]
pub(in crate::info::item) struct HItemExtendedInfo {
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    attrs: Vec<(rc::AttrId, HAttrVals)>,
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    effects: Vec<(rc::EffectId, HEffect)>,
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    mods: Vec<(rc::AttrId, Vec<HModification>)>,
}
impl<T> From<&mut T> for HItemExtendedInfo
where
    T: ItemMutCommon,
{
    fn from(core_item: &mut T) -> Self {
        let attrs = match core_item.iter_attrs() {
            Ok(iter_attrs) => iter_attrs.map(|(k, v)| (k, HAttrVals::from_core(v))).collect(),
            Err(_) => Vec::new(),
        };
        let effects = match core_item.iter_effects() {
            Ok(iter_effects) => iter_effects.map(|(k, v)| (k, HEffect::from_core(v))).collect(),
            Err(_) => Vec::new(),
        };
        let mods = match core_item.iter_modifiers() {
            Ok(iter_mods) => iter_mods
                .map(|(k, v)| (k, v.into_iter().map(|m| HModification::from_core(m)).collect()))
                .collect(),
            Err(_) => Vec::new(),
        };
        Self { attrs, effects, mods }
    }
}
