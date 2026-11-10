use super::{aar_paste, missile_flight_time};
use crate::{
    svc::SvcCtx,
    ud::{UItem, UItemId},
};

#[derive(Copy, Clone)]
pub(in crate::svc::calc) enum ItemAddRemoveReviser {
    AarPaste,
    MissileFlightTime,
}
impl ItemAddRemoveReviser {
    pub(in crate::svc::calc) const ALL: [Self; 2] = [Self::AarPaste, Self::MissileFlightTime];
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Service methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemAddRemoveReviser {
    pub(in crate::svc::calc) fn reacts_to(self, changed_item: &UItem) -> bool {
        match self {
            Self::AarPaste => aar_paste::reacts_to_item_add_removal(changed_item),
            Self::MissileFlightTime => missile_flight_time::reacts_to_item_add_removal(changed_item),
        }
    }
    pub(in crate::svc::calc) fn revise(
        self,
        ctx: &SvcCtx,
        affector_uid: UItemId,
        changed_uid: UItemId,
        changed_item: &UItem,
    ) -> bool {
        match self {
            Self::AarPaste => aar_paste::revise_on_item_add_removal(ctx, affector_uid, changed_uid, changed_item),
            Self::MissileFlightTime => missile_flight_time::revise_on_item_add_removal(ctx, affector_uid, changed_item),
        }
    }
}
