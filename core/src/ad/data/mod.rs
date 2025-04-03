use crate::util::HMap;
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
    pub items: HMap<AItemId, AItem>,
    pub attrs: HMap<AAttrId, AAttr>,
    pub mutas: HMap<AItemId, AMuta>,
    pub effects: HMap<AEffectId, AEffect>,
    pub buffs: HMap<ABuffId, ABuff>,
}
impl AData {
    pub(crate) fn new() -> Self {
        Self {
            items: HMap::new(),
            attrs: HMap::new(),
            mutas: HMap::new(),
            effects: HMap::new(),
            buffs: HMap::new(),
        }
    }
}
