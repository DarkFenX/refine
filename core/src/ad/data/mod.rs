pub use attr::AAttr;
pub use buff::{ABuff, ABuffAffecteeFilter, ABuffAggrMode, ABuffModifier};
pub use effect::{
    AEffect, AEffectAffecteeFilter, AEffectBuffInfo, AEffectBuffScope, AEffectBuffSrc, AEffectBuffSrcCustom,
    AEffectChargeInfo, AEffectId, AEffectLocation, AEffectModBuildStatus, AEffectModifier,
};
pub use item::{
    AItem, AItemChargeLimit, AItemEffectData, AItemExtras, AItemKind, AItemShipLimit, AShipDroneLimit, AShipKind,
};
pub use muta::{AMuta, AMutaAttrRange};
pub use primitives::{
    AAttrId, AAttrVal, ABuffId, ACount, ACustomEffectId, ADogmaEffectId, AEffectCatId, AItemCatId, AItemGrpId, AItemId,
    ASkillLevel, ASlotIndex, ArcAttr, ArcBuff, ArcEffect, ArcItem, ArcMuta,
};
pub use shared::{AModifierSrq, AOp, AState};

mod attr;
mod buff;
mod effect;
mod item;
mod muta;
mod primitives;
mod shared;

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
