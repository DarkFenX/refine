/// Cacheable data types.
use attr::Attr;
use buff::Buff;
use effect::Effect;
use item::Item;
use muta::Muta;

mod attr;
mod buff;
mod effect;
mod enums;
mod item;
mod muta;

#[derive(serde::Serialize, serde::Deserialize)]
pub(in crate::handler_json) struct Data {
    pub(in crate::handler_json) items: Vec<Item>,
    pub(in crate::handler_json) attrs: Vec<Attr>,
    pub(in crate::handler_json) mutas: Vec<Muta>,
    pub(in crate::handler_json) effects: Vec<Effect>,
    pub(in crate::handler_json) buffs: Vec<Buff>,
    pub(in crate::handler_json) fingerprint: String,
}
impl Data {
    pub(in crate::handler_json) fn from_adapted(adata: &rc::adh::Data, fingerprint: &str) -> Self {
        Self {
            items: adata.items.iter().map(|v| (*v).into()).collect(),
            attrs: adata.attrs.iter().map(|v| (*v).into()).collect(),
            mutas: adata.mutas.iter().map(|v| (*v).into()).collect(),
            effects: adata.effects.iter().map(|v| (*v).into()).collect(),
            buffs: adata.buffs.iter().map(|v| (*v).into()).collect(),
            fingerprint: fingerprint.to_string(),
        }
    }
    pub(in crate::handler_json) fn to_adapted(&self) -> (rc::adh::Data, String) {
        let adata = rc::adh::Data {
            items: self.items.iter().map(|v| (*v).into()).collect(),
            attrs: self.attrs.iter().map(|v| (*v).into()).collect(),
            mutas: self.mutas.iter().map(|v| (*v).into()).collect(),
            effects: self.effects.iter().map(|v| (*v).into()).collect(),
            buffs: self.buffs.iter().map(|v| (*v).into()).collect(),
        };
        (adata, self.fingerprint.clone())
    }
}
