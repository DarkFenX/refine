use crate::{
    ad::{AEffectId, AItemCatId, AItemGrpId, AItemId},
    misc::EffectMode,
    num::{SkillLevel, Value},
    rd::{RAttrId, REffectId, RItemAXt, RItemEffectData, RState, Src},
    ud::{
        ItemId, UFitId,
        item::{UEffectUpdates, UItemBase, bool_to_state_offline, state_to_bool},
    },
    util::{LibNamed, RMap, RSet},
};

#[derive(Clone)]
pub(crate) struct URig {
    pub(super) base: UItemBase,
    fit_uid: UFitId,
}
impl URig {
    pub(crate) fn new(item_id: ItemId, type_aid: AItemId, fit_uid: UFitId, rig_state: bool, src: &Src) -> Self {
        Self {
            base: UItemBase::new(item_id, type_aid, bool_to_state_offline(rig_state), src),
            fit_uid,
        }
    }
    // Item base methods
    pub(crate) fn get_item_id(&self) -> ItemId {
        self.base.get_item_id()
    }
    pub(crate) fn get_type_aid(&self) -> AItemId {
        self.base.get_type_aid()
    }
    pub(crate) fn set_type_aid(&mut self, type_aid: AItemId, src: &Src) {
        self.base.set_type_aid(type_aid, src);
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
    pub(crate) fn get_val_fitted_group_id(&self) -> Option<AItemGrpId> {
        self.base.get_val_fitted_group_id()
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
    pub(crate) fn update_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        self.base.update_reffs(reuse_eupdates, src);
    }
    pub(in crate::ud::item) fn stop_all_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        self.base.stop_all_reffs(reuse_eupdates, src)
    }
    pub(in crate::ud::item) fn get_effect_mode(&self, effect_rid: &REffectId) -> EffectMode {
        self.base.get_effect_mode(effect_rid)
    }
    pub(in crate::ud::item) fn set_effect_mode(&mut self, effect_aid: AEffectId, effect_mode: EffectMode, src: &Src) {
        self.base.set_effect_mode(effect_aid, effect_mode, src)
    }
    pub(in crate::ud::item) fn set_effect_modes(
        &mut self,
        effect_modes: impl Iterator<Item = (AEffectId, EffectMode)>,
        src: &Src,
    ) {
        self.base.set_effect_modes(effect_modes, src)
    }
    pub(crate) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::ud::item) fn src_changed(&mut self, src: &Src) {
        self.base.src_changed(src);
    }
    // Item-specific methods
    pub(crate) fn get_rig_state(&self) -> bool {
        state_to_bool(self.base.get_state())
    }
    pub(crate) fn set_rig_state(&mut self, state: bool) {
        self.base.set_state(bool_to_state_offline(state))
    }
    pub(crate) fn get_fit_uid(&self) -> UFitId {
        self.fit_uid
    }
}
impl LibNamed for URig {
    fn lib_get_name() -> &'static str {
        "URig"
    }
}
impl std::fmt::Display for URig {
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
