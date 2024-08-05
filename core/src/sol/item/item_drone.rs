use crate::{
    ad,
    defs::{EItemId, SolFitId, SolItemId},
    sol::item::{SolEffectModes, SolItemBase, SolItemState},
    src::Src,
    util::{Named, Result},
};

#[derive(Clone)]
pub(in crate::sol) struct SolDrone {
    base: SolItemBase,
    fit_id: SolFitId,
    pub(in crate::sol) state: SolItemState,
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
            base: SolItemBase::new(src, id, a_item_id),
            fit_id,
            state,
        }
    }
    // Item base methods
    pub(in crate::sol) fn get_id(&self) -> SolItemId {
        self.base.get_id()
    }
    pub(in crate::sol) fn get_a_item_id(&self) -> EItemId {
        self.base.get_a_item_id()
    }
    pub(in crate::sol) fn get_a_item(&self) -> Result<&ad::ArcItem> {
        self.base.get_a_item()
    }
    pub(in crate::sol) fn get_effect_modes(&self) -> &SolEffectModes {
        self.base.get_effect_modes()
    }
    pub(in crate::sol) fn get_effect_modes_mut(&mut self) -> &mut SolEffectModes {
        self.base.get_effect_modes_mut()
    }
    pub(in crate::sol) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::sol::item) fn reload_a_item(&mut self, src: &Src) {
        self.base.reload_a_item(src);
    }
    // Item-specific methods
    pub(in crate::sol) fn get_fit_id(&self) -> SolFitId {
        self.fit_id
    }
}
impl Named for SolDrone {
    fn get_name() -> &'static str {
        "SolDrone"
    }
}
impl std::fmt::Display for SolDrone {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}(id={}, a_item_id={})",
            Self::get_name(),
            self.get_id(),
            self.get_a_item_id(),
        )
    }
}
