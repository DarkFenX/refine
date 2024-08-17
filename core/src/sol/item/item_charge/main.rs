use crate::{
    ad,
    defs::{EItemId, SolFitId, SolItemId},
    err::basic::ItemLoadedError,
    sol::item::{state_to_bool, SolEffectModes, SolItemBase, SolItemState, SolProjs},
    src::Src,
    util::Named,
};

#[derive(Clone)]
pub(in crate::sol) struct SolCharge {
    base: SolItemBase,
    fit_id: SolFitId,
    cont_id: SolItemId,
    state_override: Option<SolItemState>,
    projs: SolProjs,
}
impl SolCharge {
    pub(in crate::sol) fn new(
        src: &Src,
        id: SolItemId,
        fit_id: SolFitId,
        type_id: EItemId,
        cont_id: SolItemId,
        cont_state: SolItemState,
        state: bool,
    ) -> Self {
        let mut charge = Self {
            base: SolItemBase::new(src, id, type_id, cont_state),
            fit_id,
            cont_id,
            state_override: None,
            projs: SolProjs::new(),
        };
        charge.set_bool_state(state);
        charge
    }
    // Item base methods
    pub(in crate::sol) fn get_id(&self) -> SolItemId {
        self.base.get_id()
    }
    pub(in crate::sol) fn get_type_id(&self) -> EItemId {
        self.base.get_type_id()
    }
    pub(in crate::sol) fn get_a_item(&self) -> Result<&ad::ArcItem, ItemLoadedError> {
        self.base.get_a_item()
    }
    pub(in crate::sol) fn get_state(&self) -> SolItemState {
        match self.state_override {
            Some(state_override) => state_override,
            None => self.base.get_state(),
        }
    }
    pub(in crate::sol) fn set_state(&mut self, state: SolItemState) {
        self.base.set_state(state)
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
    pub(in crate::sol) fn get_cont_id(&self) -> SolItemId {
        self.cont_id
    }
    pub(in crate::sol) fn get_bool_state(&self) -> bool {
        state_to_bool(self.get_state())
    }
    pub(in crate::sol) fn set_bool_state(&mut self, state: bool) {
        match state {
            true => self.state_override = None,
            false => self.state_override = Some(SolItemState::Ghost),
        }
    }
    pub(in crate::sol) fn get_projs(&self) -> &SolProjs {
        &self.projs
    }
    pub(in crate::sol) fn get_projs_mut(&mut self) -> &mut SolProjs {
        &mut self.projs
    }
}
impl Named for SolCharge {
    fn get_name() -> &'static str {
        "SolCharge"
    }
}
impl std::fmt::Display for SolCharge {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}(id={}, type_id={})",
            Self::get_name(),
            self.get_id(),
            self.get_type_id(),
        )
    }
}
