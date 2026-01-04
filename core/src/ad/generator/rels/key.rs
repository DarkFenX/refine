use crate::{
    def::DefId,
    ed::{EAbilId, EAttrId, EBuffId, EEffectId, EItemGrpId, EItemId, EItemListId},
    util::f64_to_i32,
};

// Part of primary and foreign keys
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::ad::generator) struct KeyPart(DefId);
impl KeyPart {
    pub(in crate::ad::generator) fn new(id: DefId) -> Self {
        Self(id)
    }
    pub(in crate::ad::generator) fn new_f64(id: f64) -> Self {
        Self(f64_to_i32(id))
    }
    pub(in crate::ad::generator) fn into_inner(self) -> DefId {
        self.0
    }
}
impl From<EItemId> for KeyPart {
    fn from(id: EItemId) -> Self {
        Self(id.into_inner())
    }
}
impl From<EItemGrpId> for KeyPart {
    fn from(id: EItemGrpId) -> Self {
        Self(id.into_inner())
    }
}
impl From<EItemListId> for KeyPart {
    fn from(id: EItemListId) -> Self {
        Self(id.into_inner())
    }
}
impl From<EAttrId> for KeyPart {
    fn from(id: EAttrId) -> Self {
        Self(id.into_inner())
    }
}
impl From<EEffectId> for KeyPart {
    fn from(id: EEffectId) -> Self {
        Self(id.into_inner())
    }
}
impl From<EAbilId> for KeyPart {
    fn from(id: EAbilId) -> Self {
        Self(id.into_inner())
    }
}
impl From<EBuffId> for KeyPart {
    fn from(id: EBuffId) -> Self {
        Self(id.into_inner())
    }
}
