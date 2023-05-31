use std::{fmt, sync::Arc};

use crate::{
    consts::{ModRack, State},
    defs::{ReeId, ReeIdx, ReeInt},
    ert,
    src::Src,
    util::Named,
};

pub(in crate::ss) struct Module {
    pub(in crate::ss) id: ReeId,
    pub(in crate::ss) fit_id: ReeId,
    pub(in crate::ss) type_id: ReeInt,
    pub(in crate::ss) state: State,
    pub(in crate::ss) rack: ModRack,
    pub(in crate::ss) pos: ReeIdx,
    pub(in crate::ss) charge_id: Option<ReeId>,
    pub(in crate::ss) cached_item: Option<Arc<ert::Item>>,
}
impl Module {
    pub(in crate::ss) fn new(
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
