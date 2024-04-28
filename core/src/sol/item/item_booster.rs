use crate::{
    ad,
    defs::{EItemId, SlotNumber, SolFitId, SolItemId},
    ec,
    sol::item::{bool_to_state, state_to_bool, SolEffectModes, SolItemState},
    src::Src,
    util::Named,
};

pub(in crate::sol) struct SolBooster {
    pub(in crate::sol) id: SolItemId,
    pub(in crate::sol) fit_id: SolFitId,
    pub(in crate::sol) a_item_id: EItemId,
    pub(in crate::sol) state: SolItemState,
    pub(in crate::sol) effect_modes: SolEffectModes,
    pub(in crate::sol) a_item: Option<ad::ArcItem>,
}
impl SolBooster {
    pub(in crate::sol) fn new(src: &Src, id: SolItemId, fit_id: SolFitId, a_item_id: EItemId, state: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            state: bool_to_state(state),
            effect_modes: SolEffectModes::new(),
            a_item: src.get_a_item(&a_item_id).cloned(),
        }
    }
    pub(in crate::sol) fn get_bool_state(&self) -> bool {
        state_to_bool(self.state)
    }
    pub(in crate::sol) fn set_bool_state(&mut self, state: bool) {
        self.state = bool_to_state(state);
    }
    pub(in crate::sol) fn get_slot(&self) -> Option<SlotNumber> {
        match &self.a_item {
            None => None,
            Some(a_item) => match a_item.attr_vals.get(&ec::attrs::BOOSTERNESS) {
                None => None,
                Some(value) => Some(value.round() as SlotNumber),
            },
        }
    }
}
impl Named for SolBooster {
    fn get_name() -> &'static str {
        "SolBooster"
    }
}
impl std::fmt::Display for SolBooster {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={}, a_item_id={})", Self::get_name(), self.id, self.a_item_id)
    }
}
