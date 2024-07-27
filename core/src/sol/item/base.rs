use crate::{
    ad,
    defs::{EItemId, SolItemId},
    sol::item::SolEffectModes,
    src::Src,
    util::{Error, ErrorKind, Result},
};

// Item base stores all the data every item should have
pub(in crate::sol) struct SolItemBase {
    pub(in crate::sol) id: SolItemId,
    pub(in crate::sol) a_item_id: EItemId,
    pub(in crate::sol) effect_modes: SolEffectModes,
    pub(in crate::sol) a_item: Option<ad::ArcItem>,
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
    pub(in crate::sol::item) fn is_loaded(&self) -> bool {
        self.a_item.is_some()
    }
    pub(in crate::sol) fn get_a_item(&self) -> Result<&ad::ArcItem> {
        self.a_item
            .as_ref()
            .ok_or_else(|| Error::new(ErrorKind::AItemNotLoaded(self.a_item_id)))
    }
    pub(in crate::sol::item) fn reload_a_item(&mut self, src: &Src) {
        self.a_item = src.get_a_item(&self.a_item_id).cloned();
    }
}
