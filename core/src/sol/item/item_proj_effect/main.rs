use crate::{
    ad,
    defs::{EItemId, SolItemId},
    sol::item::{bool_to_state, state_to_bool, SolEffectModes, SolItemState, SolProjs},
    src::Src,
    util::Named,
};

pub(in crate::sol) struct SolProjEffect {
    pub(in crate::sol) id: SolItemId,
    pub(in crate::sol) a_item_id: EItemId,
    pub(in crate::sol) state: SolItemState,
    pub(in crate::sol) projs: SolProjs,
    pub(in crate::sol) effect_modes: SolEffectModes,
    pub(in crate::sol) a_item: Option<ad::ArcItem>,
}
impl SolProjEffect {
    pub(in crate::sol) fn new(src: &Src, id: SolItemId, a_item_id: EItemId, state: bool) -> Self {
        Self {
            id,
            a_item_id,
            state: bool_to_state(state),
            projs: SolProjs::new(),
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
}
impl Named for SolProjEffect {
    fn get_name() -> &'static str {
        "SolProjEffect"
    }
}
impl std::fmt::Display for SolProjEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={}, a_item_id={})", Self::get_name(), self.id, self.a_item_id)
    }
}
