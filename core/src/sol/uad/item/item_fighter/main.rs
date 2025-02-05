use crate::{
    ad,
    defs::{AttrVal, Count, EAttrId, EEffectId, EItemGrpId, EItemId, SkillLevel, SolFitId, SolItemId},
    sol::uad::item::{SolAutocharges, SolEffectModes, SolItemBase, SolItemState, SolMinionState, SolProjs},
    src::Src,
    util::{Named, StMap},
};

#[derive(Clone)]
pub(in crate::sol) struct SolFighter {
    base: SolItemBase,
    fit_id: SolFitId,
    count_override: Option<Count>,
    autocharges: SolAutocharges,
    projs: SolProjs,
}
impl SolFighter {
    pub(in crate::sol) fn new(
        src: &Src,
        id: SolItemId,
        type_id: EItemId,
        fit_id: SolFitId,
        state: SolMinionState,
    ) -> Self {
        Self {
            base: SolItemBase::new(src, id, type_id, state.into()),
            fit_id,
            count_override: None,
            autocharges: SolAutocharges::new(),
            projs: SolProjs::new(),
        }
    }
    // Item base methods
    pub(in crate::sol) fn get_id(&self) -> SolItemId {
        self.base.get_id()
    }
    pub(in crate::sol) fn get_type_id(&self) -> EItemId {
        self.base.get_type_id()
    }
    pub(in crate::sol) fn get_group_id(&self) -> Option<EItemGrpId> {
        self.base.get_group_id()
    }
    pub(in crate::sol) fn get_category_id(&self) -> Option<EItemGrpId> {
        self.base.get_category_id()
    }
    pub(in crate::sol) fn get_attrs(&self) -> Option<&StMap<EAttrId, AttrVal>> {
        self.base.get_attrs()
    }
    pub(in crate::sol) fn get_effect_datas(&self) -> Option<&StMap<EEffectId, ad::AItemEffectData>> {
        self.base.get_effect_datas()
    }
    pub(in crate::sol) fn get_defeff_id(&self) -> Option<Option<EEffectId>> {
        self.base.get_defeff_id()
    }
    pub(in crate::sol) fn get_skill_reqs(&self) -> Option<&StMap<EItemId, SkillLevel>> {
        self.base.get_skill_reqs()
    }
    pub(in crate::sol) fn get_a_extras(&self) -> Option<&ad::AItemExtras> {
        self.base.get_a_extras()
    }
    pub(in crate::sol) fn get_state(&self) -> SolItemState {
        self.base.get_state()
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
    pub(in crate::sol::uad::item) fn update_a_data(&mut self, src: &Src) {
        self.base.update_a_data(src);
        self.autocharges.clear()
    }
    // Item-specific methods
    pub(in crate::sol) fn get_fighter_state(&self) -> SolMinionState {
        self.base.get_state().into()
    }
    pub(in crate::sol) fn set_fighter_state(&mut self, state: SolMinionState) {
        self.base.set_state(state.into())
    }
    pub(in crate::sol) fn get_fit_id(&self) -> SolFitId {
        self.fit_id
    }
    pub(in crate::sol) fn get_count_override(&self) -> Option<Count> {
        self.count_override
    }
    pub(in crate::sol) fn get_autocharges(&self) -> &SolAutocharges {
        &self.autocharges
    }
    pub(in crate::sol) fn get_autocharges_mut(&mut self) -> &mut SolAutocharges {
        &mut self.autocharges
    }
    pub(in crate::sol) fn get_projs(&self) -> &SolProjs {
        &self.projs
    }
    pub(in crate::sol) fn get_projs_mut(&mut self) -> &mut SolProjs {
        &mut self.projs
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
            "{}(id={}, type_id={})",
            Self::get_name(),
            self.get_id(),
            self.get_type_id(),
        )
    }
}
