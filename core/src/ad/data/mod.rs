use std::sync::Arc;

pub use attr::AAttr;
pub use buff::{ABuff, ABuffAttrMod, ABuffTgtFilter};
pub use effect::{
    AEffect, AEffectAttrMod, AEffectBuffDataSrc, AEffectBuffInfo, AEffectBuffScope, AEffectDomain, AEffectTgtFilter,
    AModBuildStatus, ATgtMode,
};
pub use item::{AItem, AItemEffData, AItemType};
pub use mod_shared::AModSrq;
pub use muta::{AMuta, AMutaAttrRange};

mod attr;
mod buff;
mod effect;
mod item;
mod mod_shared;
mod muta;

pub type ArcAttr = Arc<AAttr>;
pub type ArcBuff = Arc<ABuff>;
pub type ArcEffect = Arc<AEffect>;
pub type ArcItem = Arc<AItem>;
pub type ArcMuta = Arc<AMuta>;

/// Adapted data storage.
pub struct AData {
    pub items: Vec<AItem>,
    pub attrs: Vec<AAttr>,
    pub mutas: Vec<AMuta>,
    pub effects: Vec<AEffect>,
    pub buffs: Vec<ABuff>,
}
impl AData {
    pub(crate) fn new() -> Self {
        Self {
            items: Vec::new(),
            attrs: Vec::new(),
            mutas: Vec::new(),
            effects: Vec::new(),
            buffs: Vec::new(),
        }
    }
}
