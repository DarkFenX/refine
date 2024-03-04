//! Cacheable data types.

use attr::CAttr;
use buff::{CBuff, CBuffAttrMod, CModAggrMode};
use effect::{CEffect, CEffectAttrMod, CEffectBuffInfo, CModBuildStatus, CState, CTgtMode};
use item::{CItem, CItemEffData, CItemType};
use mod_shared::{CModDomain, CModOp, CModSrq, CModTgtFilter};
use muta::{CMuta, CMutaAttrRange};

mod attr;
mod buff;
mod effect;
mod item;
mod mod_shared;
mod muta;

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
            items: a_data.items.iter().map(|v| v.into()).collect(),
            attrs: a_data.attrs.iter().map(|v| v.into()).collect(),
            mutas: a_data.mutas.iter().map(|v| v.into()).collect(),
            effects: a_data.effects.iter().map(|v| v.into()).collect(),
            buffs: a_data.buffs.iter().map(|v| v.into()).collect(),
            fingerprint: fingerprint.to_string(),
        }
    }
    pub(in crate::handler_json) fn to_adapted(&self) -> (rc::ad::AData, String) {
        let a_data = rc::ad::AData {
            items: self.items.iter().map(|v| v.into()).collect(),
            attrs: self.attrs.iter().map(|v| v.into()).collect(),
            mutas: self.mutas.iter().map(|v| v.into()).collect(),
            effects: self.effects.iter().map(|v| v.into()).collect(),
            buffs: self.buffs.iter().map(|v| v.into()).collect(),
        };
        (a_data, self.fingerprint.clone())
    }
}
