use crate::{
    ad::{AEffectId, AItemCatId, AItemGrpId, AItemId},
    misc::EffectMode,
    num::{SkillLevel, Value},
    rd::{RAttrId, RData, REffectId, RItemAXt, RItemEffectData, RState},
    ud::{
        ItemId,
        item::{UEffectUpdates, UItemBase, UProjs, bool_to_state_active, state_to_bool},
    },
    util::{LibNamed, RMap, RSet},
};

#[derive(Clone)]
pub(crate) struct UProjEffect {
    pub(super) base: UItemBase,
    projs: UProjs,
}
impl UProjEffect {
    pub(crate) fn new(item_id: ItemId, type_aid: AItemId, proj_effect_state: bool, r_data: &RData) -> Self {
        Self {
            base: UItemBase::new(item_id, type_aid, bool_to_state_active(proj_effect_state), r_data),
            projs: UProjs::new(),
        }
    }
    // Item base methods
    pub(crate) fn get_item_id(&self) -> ItemId {
        self.base.get_item_id()
    }
    pub(crate) fn get_type_aid(&self) -> AItemId {
        self.base.get_type_aid()
    }
    pub(crate) fn set_type_aid(&mut self, type_aid: AItemId, r_data: &RData) {
        self.base.set_type_aid(type_aid, r_data);
    }
    pub(crate) fn get_group_id(&self) -> Option<AItemGrpId> {
        self.base.get_group_id()
    }
    pub(crate) fn get_category_id(&self) -> Option<AItemCatId> {
        self.base.get_category_id()
    }
    pub(crate) fn get_attrs(&self) -> Option<&RMap<RAttrId, Value>> {
        self.base.get_attrs()
    }
    pub(crate) fn get_effects(&self) -> Option<&RMap<REffectId, RItemEffectData>> {
        self.base.get_effects()
    }
    pub(crate) fn get_defeff_rid(&self) -> Option<Option<REffectId>> {
        self.base.get_defeff_rid()
    }
    pub(crate) fn get_skill_reqs(&self) -> Option<&RMap<AItemId, SkillLevel>> {
        self.base.get_skill_reqs()
    }
    pub(crate) fn get_axt(&self) -> Option<&RItemAXt> {
        self.base.get_axt()
    }
    pub(crate) fn get_state(&self) -> RState {
        self.base.get_state()
    }
    pub(in crate::ud::item) fn is_ice_harvester(&self) -> bool {
        self.base.is_ice_harvester()
    }
    pub(in crate::ud::item) fn get_reffs(&self) -> Option<&RSet<REffectId>> {
        self.base.get_reffs()
    }
    pub(crate) fn update_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, r_data: &RData) {
        self.base.update_reffs(reuse_eupdates, r_data, false, false);
    }
    pub(in crate::ud::item) fn stop_all_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, r_data: &RData) {
        self.base.stop_all_reffs(reuse_eupdates, r_data, false, false)
    }
    pub(in crate::ud::item) fn get_effect_mode(&self, effect_rid: &REffectId) -> EffectMode {
        self.base.get_effect_mode(effect_rid)
    }
    pub(in crate::ud::item) fn set_effect_mode(
        &mut self,
        effect_aid: AEffectId,
        effect_mode: EffectMode,
        r_data: &RData,
    ) {
        self.base.set_effect_mode(effect_aid, effect_mode, r_data)
    }
    pub(in crate::ud::item) fn set_effect_modes(
        &mut self,
        effect_modes: impl Iterator<Item = (AEffectId, EffectMode)>,
        r_data: &RData,
    ) {
        self.base.set_effect_modes(effect_modes, r_data)
    }
    pub(crate) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::ud::item) fn r_data_changed(&mut self, r_data: &RData) {
        self.base.r_data_changed(r_data);
    }
    // Item-specific methods
    pub(crate) fn get_proj_effect_state(&self) -> bool {
        state_to_bool(self.base.get_state())
    }
    pub(crate) fn set_proj_effect_state(&mut self, state: bool) {
        self.base.set_state(bool_to_state_active(state))
    }
    pub(crate) fn get_projs(&self) -> &UProjs {
        &self.projs
    }
    pub(crate) fn get_projs_mut(&mut self) -> &mut UProjs {
        &mut self.projs
    }
}
impl LibNamed for UProjEffect {
    fn lib_get_name() -> &'static str {
        "UProjEffect"
    }
}
impl std::fmt::Display for UProjEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}(item_id={}, type_id={})",
            Self::lib_get_name(),
            self.get_item_id(),
            self.get_type_aid(),
        )
    }
}
