use crate::{
    ad,
    defs::{EItemId, SolFitId, SolItemId},
    sol::item::{SolEffectModes, SolItemState},
    src::Src,
    util::Named,
};

pub(in crate::sol) struct SolDrone {
    pub(in crate::sol) id: SolItemId,
    pub(in crate::sol) fit_id: SolFitId,
    pub(in crate::sol) a_item_id: EItemId,
    pub(in crate::sol) state: SolItemState,
    pub(in crate::sol) effect_modes: SolEffectModes,
    pub(in crate::sol) a_item: Option<ad::ArcItem>,
}
impl SolDrone {
    pub(in crate::sol) fn new(
        src: &Src,
        id: SolItemId,
        fit_id: SolFitId,
        a_item_id: EItemId,
        state: SolItemState,
    ) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            state,
            effect_modes: SolEffectModes::new(),
            a_item: src.get_a_item(&a_item_id),
        }
    }
}
impl Named for SolDrone {
    fn get_name() -> &'static str {
        "SolDrone"
    }
}
impl std::fmt::Display for SolDrone {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={}, a_item_id={})", Self::get_name(), self.id, self.a_item_id)
    }
}
