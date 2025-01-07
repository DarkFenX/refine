use crate::{
    ad,
    defs::{AttrVal, EAttrId, EEffectId, EItemGrpId, EItemId, SkillLevel, SolFitId, SolItemId},
    err::basic::ItemLoadedError,
    sol::uad::item::{SolEffectModes, SolItemBase, SolItemState, SolProjs},
    src::Src,
    util::{Named, StMap},
};

#[derive(Clone)]
pub(in crate::sol) struct SolAutocharge {
    base: SolItemBase,
    fit_id: SolFitId,
    cont_id: SolItemId,
    projs: SolProjs,
    force_disable: bool,
}
impl SolAutocharge {
    pub(in crate::sol) fn new(
        src: &Src,
        id: SolItemId,
        type_id: EItemId,
        fit_id: SolFitId,
        cont_id: SolItemId,
        cont_state: SolItemState,
        force_disable: bool,
    ) -> Self {
        Self {
            base: SolItemBase::new(src, id, type_id, cont_state),
            fit_id,
            cont_id,
            projs: SolProjs::new(),
            force_disable,
        }
    }
    // Item base methods
    pub(in crate::sol) fn get_id(&self) -> SolItemId {
        self.base.get_id()
    }
    pub(in crate::sol) fn get_type_id(&self) -> EItemId {
        self.base.get_type_id()
    }
    pub(in crate::sol) fn get_group_id(&self) -> Result<EItemGrpId, ItemLoadedError> {
        self.base.get_group_id()
    }
    pub(in crate::sol) fn get_category_id(&self) -> Result<EItemGrpId, ItemLoadedError> {
        self.base.get_category_id()
    }
    pub(in crate::sol) fn get_attrs(&self) -> Result<&StMap<EAttrId, AttrVal>, ItemLoadedError> {
        self.base.get_attrs()
    }
    pub(in crate::sol) fn get_effect_datas(&self) -> Result<&StMap<EEffectId, ad::AItemEffectData>, ItemLoadedError> {
        self.base.get_effect_datas()
    }
    pub(in crate::sol) fn get_defeff_id(&self) -> Result<Option<EEffectId>, ItemLoadedError> {
        self.base.get_defeff_id()
    }
    pub(in crate::sol) fn get_skill_reqs(&self) -> Result<&StMap<EItemId, SkillLevel>, ItemLoadedError> {
        self.base.get_skill_reqs()
    }
    pub(in crate::sol) fn get_state(&self) -> SolItemState {
        match self.force_disable {
            true => SolItemState::Ghost,
            false => self.base.get_state(),
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
    pub(in crate::sol::uad::item) fn update_a_data(&mut self, _: &Src) {
        // Just panic to expose attempts to reload it, since autocharges should never be reloaded.
        // Instead, they are removed and re-added when source changes.
        panic!("autocharges shouldn't be reloaded");
    }
    // Item-specific methods
    pub(in crate::sol) fn get_fit_id(&self) -> SolFitId {
        self.fit_id
    }
    pub(in crate::sol) fn get_cont_id(&self) -> SolItemId {
        self.cont_id
    }
    pub(in crate::sol) fn get_force_disable(&self) -> bool {
        self.force_disable
    }
    pub(in crate::sol) fn set_force_disable(&mut self, force_disable: bool) {
        self.force_disable = force_disable
    }
    pub(in crate::sol) fn get_projs(&self) -> &SolProjs {
        &self.projs
    }
    pub(in crate::sol) fn get_projs_mut(&mut self) -> &mut SolProjs {
        &mut self.projs
    }
}
impl Named for SolAutocharge {
    fn get_name() -> &'static str {
        "SolAutocharge"
    }
}
impl std::fmt::Display for SolAutocharge {
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
