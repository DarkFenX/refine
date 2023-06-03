use std::{fmt, sync::Arc};

use crate::{
    ad,
    consts::{ModRack, State},
    defs::{ReeId, ReeIdx, ReeInt},
    src::Src,
    util::Named,
};

pub(crate) struct Module {
    pub(crate) id: ReeId,
    pub(crate) fit_id: ReeId,
    pub(crate) type_id: ReeInt,
    pub(crate) state: State,
    pub(crate) rack: ModRack,
    pub(crate) pos: ReeIdx,
    pub(crate) charge_id: Option<ReeId>,
    pub(crate) cached_item: Option<Arc<ad::AItem>>,
}
impl Module {
    pub(crate) fn new(
        src: &Arc<Src>,
        id: ReeId,
        fit_id: ReeId,
        type_id: ReeInt,
        state: State,
        rack: ModRack,
        pos: ReeIdx,
        charge_id: Option<ReeId>,
    ) -> Self {
        Self {
            id,
            fit_id,
            type_id,
            state,
            rack,
            pos,
            charge_id,
            cached_item: src.cache_handler.get_item(&type_id),
        }
    }
}
impl Named for Module {
    fn get_name() -> &'static str {
        "ssi:Module"
    }
}
impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(id={}, type_id={})", Self::get_name(), self.id, self.type_id)
    }
}
