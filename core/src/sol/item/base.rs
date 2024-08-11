use crate::{
    ad,
    defs::{EItemId, SolItemId},
    err::basic::ItemLoadedError,
    sol::item::SolEffectModes,
    src::Src,
};

// Item base stores all the data every item should have
#[derive(Clone)]
pub(in crate::sol) struct SolItemBase {
    id: SolItemId,
    a_item_id: EItemId,
    effect_modes: SolEffectModes,
    a_item: Option<ad::ArcItem>,
}
impl SolItemBase {
    pub(in crate::sol::item) fn new(src: &Src, id: SolItemId, a_item_id: EItemId) -> Self {
        Self {
            id,
            a_item_id,
            effect_modes: SolEffectModes::new(),
            a_item: src.get_a_item(&a_item_id).cloned(),
        }
    }
    pub(in crate::sol::item) fn get_id(&self) -> SolItemId {
        self.id
    }
    pub(in crate::sol::item) fn get_a_item_id(&self) -> EItemId {
        self.a_item_id
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
    pub(in crate::sol::item) fn get_a_item(&self) -> Result<&ad::ArcItem, ItemLoadedError> {
        self.a_item.as_ref().ok_or_else(|| ItemLoadedError::new(self.id))
    }
    pub(in crate::sol::item) fn reload_a_item(&mut self, src: &Src) {
        self.a_item = src.get_a_item(&self.a_item_id).cloned();
    }
}
