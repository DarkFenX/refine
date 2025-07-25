use crate::{
    ad,
    def::ItemId,
    misc::EffectMode,
    rd,
    src::Src,
    ud::{
        UFitKey,
        item::{UEffectUpdates, UItemBase, bool_to_state_active, state_to_bool},
    },
    util::{Named, RMap, RSet},
};

#[derive(Clone)]
pub(crate) struct UFwEffect {
    base: UItemBase,
    fit_key: UFitKey,
}
impl UFwEffect {
    pub(crate) fn new(
        item_id: ItemId,
        a_item_id: ad::AItemId,
        fit_key: UFitKey,
        state: bool,
        src: &Src,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Self {
        Self {
            base: UItemBase::new(item_id, a_item_id, bool_to_state_active(state), src, reuse_eupdates),
            fit_key,
        }
    }
    // Item base methods
    pub(crate) fn get_item_id(&self) -> ItemId {
        self.base.get_item_id()
    }
    pub(crate) fn get_a_item_id(&self) -> ad::AItemId {
        self.base.get_a_item_id()
    }
    pub(crate) fn set_a_item_id(&mut self, a_item_id: ad::AItemId, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        self.base.set_a_item_id(a_item_id, reuse_eupdates, src);
    }
    pub(crate) fn get_a_group_id(&self) -> Option<ad::AItemGrpId> {
        self.base.get_a_group_id()
    }
    pub(crate) fn get_a_category_id(&self) -> Option<ad::AItemCatId> {
        self.base.get_a_category_id()
    }
    pub(crate) fn get_a_attrs(&self) -> Option<&RMap<ad::AAttrId, ad::AAttrVal>> {
        self.base.get_a_attrs()
    }
    pub(crate) fn get_a_effect_datas(&self) -> Option<&RMap<ad::AEffectId, ad::AItemEffectData>> {
        self.base.get_a_effect_datas()
    }
    pub(crate) fn get_a_defeff_id(&self) -> Option<Option<ad::AEffectId>> {
        self.base.get_a_defeff_id()
    }
    pub(crate) fn get_a_skill_reqs(&self) -> Option<&RMap<ad::AItemId, ad::ASkillLevel>> {
        self.base.get_a_skill_reqs()
    }
    pub(crate) fn get_r_axt(&self) -> Option<&rd::RItemAXt> {
        self.base.get_r_axt()
    }
    pub(crate) fn get_a_state(&self) -> ad::AState {
        self.base.get_a_state()
    }
    pub(in crate::ud::item) fn get_reffs(&self) -> Option<&RSet<ad::AEffectId>> {
        self.base.get_reffs()
    }
    pub(in crate::ud::item) fn start_all_reffs(&self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        self.base.start_all_reffs(reuse_eupdates, src);
    }
    pub(in crate::ud::item) fn stop_all_reffs(&self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        self.base.stop_all_reffs(reuse_eupdates, src)
    }
    pub(in crate::ud::item) fn get_effect_mode(&self, effect_id: &ad::AEffectId) -> EffectMode {
        self.base.get_effect_mode(effect_id)
    }
    pub(in crate::ud::item) fn set_effect_mode(
        &mut self,
        a_effect_id: ad::AEffectId,
        effect_mode: EffectMode,
        reuse_eupdates: &mut UEffectUpdates,
        src: &Src,
    ) {
        self.base.set_effect_mode(a_effect_id, effect_mode, reuse_eupdates, src)
    }
    pub(in crate::ud::item) fn set_effect_modes(
        &mut self,
        modes: impl Iterator<Item = (ad::AEffectId, EffectMode)>,
        reuse_eupdates: &mut UEffectUpdates,
        src: &Src,
    ) {
        self.base.set_effect_modes(modes, reuse_eupdates, src)
    }
    pub(crate) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::ud::item) fn update_a_data(&mut self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        self.base.update_r_data(reuse_eupdates, src);
    }
    // Item-specific methods
    pub(crate) fn get_fw_effect_state(&self) -> bool {
        state_to_bool(self.base.get_a_state())
    }
    pub(crate) fn set_fw_effect_state(&mut self, state: bool, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        self.base.set_a_state(bool_to_state_active(state), reuse_eupdates, src)
    }
    pub(crate) fn get_fit_key(&self) -> UFitKey {
        self.fit_key
    }
}
impl Named for UFwEffect {
    fn get_name() -> &'static str {
        "FwEffect"
    }
}
impl std::fmt::Display for UFwEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}(item_id={}, a_item_id={})",
            Self::get_name(),
            self.get_item_id(),
            self.get_a_item_id(),
        )
    }
}
