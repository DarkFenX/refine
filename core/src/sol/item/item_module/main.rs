use crate::{
    ad,
    defs::{EItemId, Idx, SolFitId, SolItemId},
    err::basic::ItemLoadedError,
    sol::{
        item::{SolEffectModes, SolItemBase, SolItemMutation, SolItemState, SolProjs},
        SolModRack,
    },
    src::Src,
    util::Named,
};

#[derive(Clone)]
pub(in crate::sol) struct SolModule {
    base: SolItemBase,
    mutation: Option<SolItemMutation>,
    fit_id: SolFitId,
    rack: SolModRack,
    pos: Idx,
    charge_id: Option<SolItemId>,
    projs: SolProjs,
}
impl SolModule {
    pub(in crate::sol) fn new(
        src: &Src,
        id: SolItemId,
        type_id: EItemId,
        fit_id: SolFitId,
        state: SolItemState,
        rack: SolModRack,
        pos: Idx,
        charge_id: Option<SolItemId>,
    ) -> Self {
        Self {
            base: SolItemBase::new(src, id, type_id, state),
            mutation: None,
            fit_id,
            rack,
            pos,
            charge_id,
            projs: SolProjs::new(),
        }
    }
    // Item base methods
    pub(in crate::sol) fn get_id(&self) -> SolItemId {
        self.base.get_id()
    }
    pub(in crate::sol) fn get_type_id(&self) -> EItemId {
        if let Some(mutation) = self.mutation.as_ref() {
            if let Some(type_id) = mutation.get_item_type_id() {
                return type_id;
            }
        }
        self.base.get_type_id()
    }
    pub(in crate::sol) fn get_a_item(&self) -> Result<&ad::ArcItem, ItemLoadedError> {
        self.base.get_a_item()
    }
    pub(in crate::sol) fn get_state(&self) -> SolItemState {
        self.base.get_state()
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
    pub(in crate::sol) fn get_rack(&self) -> SolModRack {
        self.rack
    }
    pub(in crate::sol) fn get_pos(&self) -> Idx {
        self.pos
    }
    pub(in crate::sol) fn set_pos(&mut self, pos: Idx) {
        self.pos = pos
    }
    pub(in crate::sol) fn get_charge_id(&self) -> Option<SolItemId> {
        self.charge_id
    }
    pub(in crate::sol) fn set_charge_id(&mut self, charge_item_id: Option<SolItemId>) {
        self.charge_id = charge_item_id
    }
    pub(in crate::sol) fn get_projs(&self) -> &SolProjs {
        &self.projs
    }
    pub(in crate::sol) fn get_projs_mut(&mut self) -> &mut SolProjs {
        &mut self.projs
    }
}
impl Named for SolModule {
    fn get_name() -> &'static str {
        "SolModule"
    }
}
impl std::fmt::Display for SolModule {
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
