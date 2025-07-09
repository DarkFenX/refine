pub use attr::AAttr;
pub use buff::{ABuff, ABuffAffecteeFilter, ABuffAggrMode, ABuffModifier};
pub(crate) use effect::AEffectXt;
pub use effect::{
    AEffect, AEffectAffecteeFilter, AEffectBuffInfo, AEffectBuffScope, AEffectBuffSrc, AEffectBuffSrcCustom, AEffectId,
    AEffectLocation, AEffectModBuildStatus, AEffectModifier, AEffectRt,
};
pub use item::{AItem, AItemEffectData, AItemKind, AItemRt, ASkillLevel};
pub(crate) use item::{AItemChargeLimit, AItemContLimit, AItemShipLimit, AItemXt, AShipDroneLimit, AShipKind};
pub use muta::{AMuta, AMutaAttrRange};
pub(crate) use primitives::ASlotIndex;
pub use primitives::{
    AAttrId, AAttrVal, ABuffId, ACount, ACustomEffectId, ADogmaEffectId, AEffectCatId, AItemCatId, AItemGrpId, AItemId,
    ArcAttr, ArcBuff, ArcEffectRt, ArcItemRt, ArcMuta,
};
pub use shared::{AModifierSrq, AOp, AState};

use crate::util::RMap;

mod attr;
mod buff;
mod effect;
mod item;
mod muta;
mod primitives;
mod shared;

/// Adapted data storage.
pub struct AData {
    pub items: RMap<AItemId, AItem>,
    pub attrs: RMap<AAttrId, AAttr>,
    pub mutas: RMap<AItemId, AMuta>,
    pub effects: RMap<AEffectId, AEffect>,
    pub buffs: RMap<ABuffId, ABuff>,
}
impl AData {
    pub(crate) fn new() -> Self {
        Self {
            items: RMap::new(),
            attrs: RMap::new(),
            mutas: RMap::new(),
            effects: RMap::new(),
            buffs: RMap::new(),
        }
    }
}
