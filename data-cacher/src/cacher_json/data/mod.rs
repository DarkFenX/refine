//! Cacheable data types.

use attr::CAttr;
use buff::{CBuff, CBuffAffecteeFilter, CBuffAggrMode, CBuffModifier};
use effect::{CEffect, CEffectAffecteeFilter, CEffectBuffInfo, CEffectId, CEffectLocation, CEffectModifier};
use item::{CItem, CItemEffectData};
use mod_shared::{CModifierSrq, COp};
use muta::{CMuta, CMutaAttrRange};
use primitives::{
    CAttrId, CAttrVal, CBuffId, CCount, CCustomEffectId, CDogmaEffectId, CEffectCatId, CItemCatId, CItemGrpId, CItemId,
    CSkillLevel,
};
use shared::CState;

mod attr;
mod buff;
mod effect;
mod item;
mod mod_shared;
mod muta;
mod primitives;
mod shared;

#[derive(serde::Serialize, serde::Deserialize)]
pub(in crate::cacher_json) struct CData {
    pub(in crate::cacher_json) items: Vec<CItem>,
    pub(in crate::cacher_json) attrs: Vec<CAttr>,
    pub(in crate::cacher_json) mutas: Vec<CMuta>,
    pub(in crate::cacher_json) effects: Vec<CEffect>,
    pub(in crate::cacher_json) buffs: Vec<CBuff>,
}
impl From<&rc::ad::AData> for CData {
    fn from(a_data: &rc::ad::AData) -> Self {
        Self {
            items: a_data.items.values().map(|v| v.into()).collect(),
            attrs: a_data.attrs.values().map(|v| v.into()).collect(),
            mutas: a_data.mutas.values().map(|v| v.into()).collect(),
            effects: a_data.effects.values().map(|v| v.into()).collect(),
            buffs: a_data.buffs.values().map(|v| v.into()).collect(),
        }
    }
}
impl From<&CData> for rc::ad::AData {
    fn from(c_data: &CData) -> Self {
        Self {
            items: c_data
                .items
                .iter()
                .map(|v| {
                    let item = rc::ad::AItem::from(v);
                    (item.id, item)
                })
                .collect(),
            attrs: c_data
                .attrs
                .iter()
                .map(|v| {
                    let item = rc::ad::AAttr::from(v);
                    (item.id, item)
                })
                .collect(),
            mutas: c_data
                .mutas
                .iter()
                .map(|v| {
                    let item = rc::ad::AMuta::from(v);
                    (item.id, item)
                })
                .collect(),
            effects: c_data
                .effects
                .iter()
                .map(|v| {
                    let item = rc::ad::AEffect::from(v);
                    (item.id, item)
                })
                .collect(),
            buffs: c_data
                .buffs
                .iter()
                .map(|v| {
                    let item = rc::ad::ABuff::from(v);
                    (item.id, item)
                })
                .collect(),
        }
    }
}
