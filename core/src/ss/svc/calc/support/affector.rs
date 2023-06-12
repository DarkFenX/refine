use std::hash::{Hash, Hasher};

use crate::{ad, defs::ReeId};

#[derive(Clone)]
pub(in crate::ss::svc::calc) struct AffectorSpec {
    pub(in crate::ss::svc::calc) item_id: ReeId,
    pub(in crate::ss::svc::calc) effect: ad::ArcEffect,
    pub(in crate::ss::svc::calc) modifier: ad::AAttrMod,
}
impl AffectorSpec {
    pub(in crate::ss::svc::calc) fn new(item_id: ReeId, effect: ad::ArcEffect, modifier: ad::AAttrMod) -> Self {
        Self {
            item_id,
            effect,
            modifier,
        }
    }
}
impl PartialEq for AffectorSpec {
    fn eq(&self, other: &Self) -> bool {
        self.item_id == other.item_id && self.effect.id == other.effect.id && self.modifier == other.modifier
    }
}
impl Eq for AffectorSpec {}
impl Hash for AffectorSpec {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.item_id.hash(state);
        self.effect.id.hash(state);
        self.modifier.hash(state);
    }
}
