use crate::util::StMap;
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
    pub items: StMap<AItemId, AItem>,
    pub attrs: StMap<AAttrId, AAttr>,
    pub mutas: StMap<AItemId, AMuta>,
    pub effects: StMap<AEffectId, AEffect>,
    pub buffs: StMap<ABuffId, ABuff>,
}
impl AData {
    pub(crate) fn new() -> Self {
        Self {
            items: StMap::new(),
            attrs: StMap::new(),
            mutas: StMap::new(),
            effects: StMap::new(),
            buffs: StMap::new(),
        }
    }
}
