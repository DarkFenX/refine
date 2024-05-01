use std::sync::Arc;

pub use attr::AAttr;
pub use buff::{ABuff, ABuffAggrMode, ABuffAttrMod, ABuffTgtFilter};
pub use effect::{
    AEffect, AEffectAffecteeFilter, AEffectBuffDataSrc, AEffectBuffInfo, AEffectBuffScope, AEffectDomain,
    AEffectModifier, AModBuildStatus, ATgtMode,
};
pub use item::{AItem, AItemEffData, AItemType};
pub use muta::{AMuta, AMutaAttrRange};
pub use shared::{AModOp, AModSrq, AState};

mod attr;
mod buff;
mod effect;
mod item;
mod muta;
mod shared;

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
