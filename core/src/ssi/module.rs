use std::{fmt, sync::Arc};

use crate::{
    ad,
    consts::{ModRack, State},
    defs::{ReeId, ReeIdx, ReeInt},
    src::Src,
    util::Named,
};

pub(crate) struct SsModule {
    pub(crate) id: ReeId,
    pub(crate) fit_id: ReeId,
    pub(crate) a_item_id: ReeInt,
    pub(crate) state: State,
    pub(crate) rack: ModRack,
    pub(crate) pos: ReeIdx,
    pub(crate) charge_a_item_id: Option<ReeId>,
    pub(crate) a_item: Option<Arc<ad::AItem>>,
}
impl SsModule {
    pub(crate) fn new(
        src: &Src,
        id: ReeId,
        fit_id: ReeId,
        a_item_id: ReeInt,
        state: State,
        rack: ModRack,
        pos: ReeIdx,
        charge_a_item_id: Option<ReeId>,
    ) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            state,
            rack,
            pos,
            charge_a_item_id: charge_a_item_id,
            a_item: src.get_a_item(&a_item_id),
        }
    }
}
impl Named for SsModule {
    fn get_name() -> &'static str {
        "SsModule"
    }
}
impl fmt::Display for SsModule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(id={}, a_item_id={})", Self::get_name(), self.id, self.a_item_id)
    }
}
