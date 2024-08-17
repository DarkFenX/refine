use crate::{
    ad,
    defs::{EItemId, SolItemId},
    err::basic::ItemLoadedError,
    sol::item::{SolEffectModes, SolItemState},
    src::Src,
};

// Item base stores all the data every item should have
#[derive(Clone)]
pub(in crate::sol) struct SolItemBase {
    id: SolItemId,
    type_id: EItemId,
    state: SolItemState,
    effect_modes: SolEffectModes,
    a_item: Option<ad::ArcItem>,
}
impl SolItemBase {
    pub(in crate::sol::item) fn new(src: &Src, id: SolItemId, type_id: EItemId, state: SolItemState) -> Self {
        Self {
            id,
            type_id,
            state,
            effect_modes: SolEffectModes::new(),
            a_item: src.get_a_item(&type_id).cloned(),
        }
    }
    pub(in crate::sol::item) fn get_id(&self) -> SolItemId {
        self.id
    }
    pub(in crate::sol::item) fn get_type_id(&self) -> EItemId {
        self.type_id
    }
    pub(in crate::sol::item) fn get_a_item(&self) -> Result<&ad::ArcItem, ItemLoadedError> {
        self.a_item.as_ref().ok_or_else(|| ItemLoadedError::new(self.id))
    }
    pub(in crate::sol::item) fn get_state(&self) -> SolItemState {
        self.state
    }
    pub(in crate::sol::item) fn set_state(&mut self, state: SolItemState) {
        self.state = state
    }
    pub(in crate::sol::item) fn get_effect_modes(&self) -> &SolEffectModes {
        &self.effect_modes
    }
    pub(in crate::sol::item) fn get_effect_modes_mut(&mut self) -> &mut SolEffectModes {
        &mut self.effect_modes
    }
    pub(in crate::sol::item) fn is_loaded(&self) -> bool {
        self.a_item.is_some()
    }
    pub(in crate::sol::item) fn reload_a_item(&mut self, src: &Src) {
        self.a_item = src.get_a_item(&self.type_id).cloned();
    }
}
