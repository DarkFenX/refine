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
pub(in crate::handler_json) struct CData {
    pub(in crate::handler_json) items: Vec<CItem>,
    pub(in crate::handler_json) attrs: Vec<CAttr>,
    pub(in crate::handler_json) mutas: Vec<CMuta>,
    pub(in crate::handler_json) effects: Vec<CEffect>,
    pub(in crate::handler_json) buffs: Vec<CBuff>,
    pub(in crate::handler_json) fingerprint: String,
}
impl CData {
    pub(in crate::handler_json) fn from_adapted(a_data: &rc::ad::AData, fingerprint: &str) -> Self {
        Self {
            items: a_data.items.values().map(|v| v.into()).collect(),
            attrs: a_data.attrs.values().map(|v| v.into()).collect(),
            mutas: a_data.mutas.values().map(|v| v.into()).collect(),
            effects: a_data.effects.values().map(|v| v.into()).collect(),
            buffs: a_data.buffs.values().map(|v| v.into()).collect(),
            fingerprint: fingerprint.to_string(),
        }
    }
    pub(in crate::handler_json) fn to_adapted(&self) -> (rc::ad::AData, String) {
        let a_data = rc::ad::AData {
            items: self
                .items
                .iter()
                .map(|v| {
                    let item = rc::ad::AItem::from(v);
                    (item.id, item)
                })
                .collect(),
            attrs: self
                .attrs
                .iter()
                .map(|v| {
                    let item = rc::ad::AAttr::from(v);
                    (item.id, item)
                })
                .collect(),
            mutas: self
                .mutas
                .iter()
                .map(|v| {
                    let item = rc::ad::AMuta::from(v);
                    (item.id, item)
                })
                .collect(),
            effects: self
                .effects
                .iter()
                .map(|v| {
                    let item = rc::ad::AEffect::from(v);
                    (item.id, item)
                })
                .collect(),
            buffs: self
                .buffs
                .iter()
                .map(|v| {
                    let item = rc::ad::ABuff::from(v);
                    (item.id, item)
                })
                .collect(),
        };
        (a_data, self.fingerprint.clone())
    }
}
