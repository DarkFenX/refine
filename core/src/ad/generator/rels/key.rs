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
    pub(in crate::ad::generator) fn from_f64_rounded(id: f64) -> Self {
        Self(round_f64_to_i32(id))
    }
}
impl From<EItemId> for KeyPart {
    fn from(id: EItemId) -> Self {
        Self(id.into_i32())
    }
}
impl From<EItemGrpId> for KeyPart {
    fn from(id: EItemGrpId) -> Self {
        Self(id.into_i32())
    }
}
impl From<EItemListId> for KeyPart {
    fn from(id: EItemListId) -> Self {
        Self(id.into_i32())
    }
}
impl From<EAttrId> for KeyPart {
    fn from(id: EAttrId) -> Self {
        Self(id.into_i32())
    }
}
impl From<EEffectId> for KeyPart {
    fn from(id: EEffectId) -> Self {
        Self(id.into_i32())
    }
}
impl From<EAbilId> for KeyPart {
    fn from(id: EAbilId) -> Self {
        Self(id.into_i32())
    }
}
impl From<EBuffId> for KeyPart {
    fn from(id: EBuffId) -> Self {
        Self(id.into_i32())
    }
}
