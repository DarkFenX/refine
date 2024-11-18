use crate::{
    ad,
    defs::{AttrVal, EAttrId, EEffectId, EItemGrpId, EItemId, SkillLevel, SolItemId},
    err::basic::ItemLoadedError,
    sol::item::{SolEffectModes, SolItemState},
    src::Src,
    util::StMap,
};

// Item base stores all the data every item should have
#[derive(Clone)]
pub(in crate::sol) struct SolItemBase {
    // Following fields are part of item skeleton
    id: SolItemId,
    type_id: EItemId,
    state: SolItemState,
    effect_modes: SolEffectModes,
    // Following fields are stored for fast access / optimization
    a_item: Option<ad::ArcItem>,
}
impl SolItemBase {
    pub(in crate::sol::item) fn new(src: &Src, id: SolItemId, type_id: EItemId, state: SolItemState) -> Self {
        Self {
            id,
            type_id,
            state,
            effect_modes: SolEffectModes::new(),
            a_item: src.get_a_item(&type_id).cloned(),
        }
    }
    pub(in crate::sol::item) fn get_id(&self) -> SolItemId {
        self.id
    }
    pub(in crate::sol::item) fn get_type_id(&self) -> EItemId {
        self.type_id
    }
    pub(in crate::sol::item) fn get_group_id(&self) -> Result<EItemGrpId, ItemLoadedError> {
        self.get_a_item().map(|v| v.grp_id)
    }
    pub(in crate::sol::item) fn get_category_id(&self) -> Result<EItemGrpId, ItemLoadedError> {
        self.get_a_item().map(|v| v.cat_id)
    }
    pub(in crate::sol::item) fn get_attrs(&self) -> Result<&StMap<EAttrId, AttrVal>, ItemLoadedError> {
        self.get_a_item().map(|v| &v.attr_vals)
    }
    pub(in crate::sol::item) fn get_effect_datas(
        &self,
    ) -> Result<&StMap<EEffectId, ad::AItemEffectData>, ItemLoadedError> {
        self.get_a_item().map(|v| &v.effect_datas)
    }
    pub(in crate::sol::item) fn get_defeff_id(&self) -> Result<Option<EEffectId>, ItemLoadedError> {
        self.get_a_item().map(|v| v.defeff_id)
    }
    pub(in crate::sol::item) fn get_skill_reqs(&self) -> Result<&StMap<EItemId, SkillLevel>, ItemLoadedError> {
        self.get_a_item().map(|v| &v.srqs)
    }
    pub(in crate::sol::item) fn get_state(&self) -> SolItemState {
        self.state
    }
    pub(in crate::sol::item) fn set_state(&mut self, state: SolItemState) {
        self.state = state
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
    pub(in crate::sol::item) fn reload_a_data(&mut self, src: &Src) {
        self.a_item = src.get_a_item(&self.type_id).cloned();
    }
    fn get_a_item(&self) -> Result<&ad::ArcItem, ItemLoadedError> {
        self.a_item.as_ref().ok_or_else(|| ItemLoadedError::new(self.id))
    }
}
