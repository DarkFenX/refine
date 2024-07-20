use crate::{
    defs::{EItemId, SlotNumber, SolFitId, SolItemId},
    ec,
    sol::item::{bool_to_state, state_to_bool, SolItemBase, SolItemState},
    src::Src,
    util::Named,
};

pub(in crate::sol) struct SolImplant {
    pub(in crate::sol) base: SolItemBase,
    pub(in crate::sol) fit_id: SolFitId,
    pub(in crate::sol) state: SolItemState,
}
impl SolImplant {
    pub(in crate::sol) fn new(src: &Src, id: SolItemId, fit_id: SolFitId, a_item_id: EItemId, state: bool) -> Self {
        Self {
            base: SolItemBase::new(src, id, a_item_id),
            fit_id,
            state: bool_to_state(state),
        }
    }
    pub(in crate::sol) fn get_bool_state(&self) -> bool {
        state_to_bool(self.state)
    }
    pub(in crate::sol) fn set_bool_state(&mut self, state: bool) {
        self.state = bool_to_state(state);
    }
    pub(in crate::sol) fn get_slot(&self) -> Option<SlotNumber> {
        match &self.base.a_item {
            None => None,
            Some(a_item) => match a_item.attr_vals.get(&ec::attrs::IMPLANTNESS) {
                None => None,
                Some(value) => Some(value.round() as SlotNumber),
            },
        }
    }
}
impl Named for SolImplant {
    fn get_name() -> &'static str {
        "SolImplant"
    }
}
impl std::fmt::Display for SolImplant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}(id={}, a_item_id={})",
            Self::get_name(),
            self.base.id,
            self.base.a_item_id
        )
    }
}
