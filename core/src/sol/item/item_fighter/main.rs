use crate::{
    ad,
    defs::{Amount, EItemId, SolFitId, SolItemId},
    sol::item::{SolAutocharges, SolEffectModes, SolItemBase, SolItemState},
    src::Src,
    util::{Named, Result},
};

pub(in crate::sol) struct SolFighter {
    base: SolItemBase,
    fit_id: SolFitId,
    pub(in crate::sol) state: SolItemState,
    pub(in crate::sol) amt_override: Option<Amount>,
    pub(in crate::sol) autocharges: SolAutocharges,
}
impl SolFighter {
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
            amt_override: None,
            autocharges: SolAutocharges::new(),
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
        self.autocharges.clear();
    }
    // Item-specific methods
    pub(in crate::sol) fn get_fit_id(&self) -> SolFitId {
        self.fit_id
    }
}
impl Named for SolFighter {
    fn get_name() -> &'static str {
        "SolFighter"
    }
}
impl std::fmt::Display for SolFighter {
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
