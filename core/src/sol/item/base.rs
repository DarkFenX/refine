use crate::{
    ad,
    defs::{EItemId, SolItemId},
    sol::item::SolEffectModes,
    src::Src,
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
}
