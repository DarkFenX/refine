use crate::{
    ad,
    defs::{EItemId, SlotNumber, SsFitId, SsItemId},
    ec,
    src::Src,
    ss::item::{bool_to_state, state_to_bool, SsEffectModes, SsItemState},
    util::Named,
};

pub(in crate::ss) struct SsBooster {
    pub(in crate::ss) id: SsItemId,
    pub(in crate::ss) fit_id: SsFitId,
    pub(in crate::ss) a_item_id: EItemId,
    pub(in crate::ss) state: SsItemState,
    pub(in crate::ss) effect_modes: SsEffectModes,
    pub(in crate::ss) a_item: Option<ad::ArcItem>,
}
impl SsBooster {
    pub(in crate::ss) fn new(src: &Src, id: SsItemId, fit_id: SsFitId, a_item_id: EItemId, state: bool) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            state: bool_to_state(state),
            effect_modes: SsEffectModes::new(),
            a_item: src.get_a_item(&a_item_id),
        }
    }
    pub(in crate::ss) fn get_bool_state(&self) -> bool {
        state_to_bool(self.state)
    }
    pub(in crate::ss) fn set_bool_state(&mut self, state: bool) {
        self.state = bool_to_state(state);
    }
    pub(in crate::ss) fn get_slot(&self) -> Option<SlotNumber> {
        match &self.a_item {
            None => None,
            Some(a_item) => match a_item.attr_vals.get(&ec::attrs::BOOSTERNESS) {
                None => None,
                Some(value) => Some(value.round() as SlotNumber),
            },
        }
    }
}
impl Named for SsBooster {
    fn get_name() -> &'static str {
        "SsBooster"
    }
}
impl std::fmt::Display for SsBooster {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={}, a_item_id={})", Self::get_name(), self.id, self.a_item_id)
    }
}
