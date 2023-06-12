use std::hash::{Hash, Hasher};

use crate::{
    ad,
    defs::{ReeId, ReeIdx},
};

#[derive(Clone)]
pub(in crate::ss::svc::calc) struct AffectorSpec {
    pub(in crate::ss::svc::calc) item_id: ReeId,
    pub(in crate::ss::svc::calc) effect: ad::ArcEffect,
    pub(in crate::ss::svc::calc) modifier_idx: ReeIdx,
}
impl AffectorSpec {
    pub(in crate::ss::svc::calc) fn new(item_id: ReeId, effect: ad::ArcEffect, modifier_idx: ReeIdx) -> Self {
        Self {
            item_id,
            effect,
            modifier_idx,
        }
    }
    pub(in crate::ss::svc::calc) fn get_modifier(&self) -> Option<&ad::AAttrMod> {
        self.effect.mods.get(self.modifier_idx)
    }
}
impl PartialEq for AffectorSpec {
    fn eq(&self, other: &Self) -> bool {
        self.item_id == other.item_id && self.effect.id == other.effect.id && self.modifier_idx == other.modifier_idx
    }
}
impl Eq for AffectorSpec {}
impl Hash for AffectorSpec {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.item_id.hash(state);
        self.effect.id.hash(state);
        self.modifier_idx.hash(state);
    }
}
