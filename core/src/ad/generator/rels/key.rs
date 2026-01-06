use crate::{
    ed::{EAbilId, EAttrId, EBuffId, EEffectId, EItemGrpId, EItemId, EItemListId},
    util::round_f64_to_i32,
};

// Part of primary and foreign keys
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::ad::generator) struct KeyPart(i32);
impl KeyPart {
    pub(in crate::ad::generator) fn from_i32(id: i32) -> Self {
        Self(id)
    }
    pub(in crate::ad::generator) fn into_i32(self) -> i32 {
        self.0
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl KeyPart {
    pub(in crate::ad::generator) fn from_f64_rounded(id: f64) -> Self {
        Self(round_f64_to_i32(id))
    }
    pub(in crate::ad::generator) fn from_item_eid(item_eid: EItemId) -> Self {
        Self(item_eid.into_i32())
    }
    pub(in crate::ad::generator) fn from_item_grp_eid(item_grp_eid: EItemGrpId) -> Self {
        Self(item_grp_eid.into_i32())
    }
    pub(in crate::ad::generator) fn from_item_list_eid(item_list_eid: EItemListId) -> Self {
        Self(item_list_eid.into_i32())
    }
    pub(in crate::ad::generator) fn from_attr_eid(attr_eid: EAttrId) -> Self {
        Self(attr_eid.into_i32())
    }
    pub(in crate::ad::generator) fn from_effect_eid(effect_eid: EEffectId) -> Self {
        Self(effect_eid.into_i32())
    }
    pub(in crate::ad::generator) fn from_abil_eid(abil_eid: EAbilId) -> Self {
        Self(abil_eid.into_i32())
    }
    pub(in crate::ad::generator) fn from_buff_eid(buff_eid: EBuffId) -> Self {
        Self(buff_eid.into_i32())
    }
}
