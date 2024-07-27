use crate::{
    ad,
    defs::{EItemId, SolItemId},
    sol::item::{bool_to_state, state_to_bool, SolItemBase, SolItemState, SolProjs},
    src::Src,
    util::{Named, Result},
};

pub(in crate::sol) struct SolProjEffect {
    pub(in crate::sol) base: SolItemBase,
    pub(in crate::sol) state: SolItemState,
    pub(in crate::sol) projs: SolProjs,
}
impl SolProjEffect {
    pub(in crate::sol) fn new(src: &Src, id: SolItemId, a_item_id: EItemId, state: bool) -> Self {
        Self {
            base: SolItemBase::new(src, id, a_item_id),
            state: bool_to_state(state),
            projs: SolProjs::new(),
        }
    }
    pub(in crate::sol::item) fn get_id(&self) -> SolItemId {
        self.base.get_id()
    }
    pub(in crate::sol::item) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::sol) fn get_a_item(&self) -> Result<&ad::ArcItem> {
        self.base.get_a_item()
    }
    pub(in crate::sol::item) fn reload_a_item(&mut self, src: &Src) {
        self.base.reload_a_item(src);
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
        write!(
            f,
            "{}(id={}, a_item_id={})",
            Self::get_name(),
            self.base.id,
            self.base.a_item_id
        )
    }
}
