use crate::{
    ad::{AAttrId, AAttrVal, AEffectId, AItemCatId, AItemEffectData, AItemGrpId, AItemId, ASkillLevel, AState},
    def::ItemId,
    misc::EffectMode,
    rd::{REffectKey, RItemAXt},
    src::Src,
    ud::{
        UFitKey, UItemKey,
        item::{Projs, UEffectUpdates, UItemBase},
    },
    util::{Named, RMap, RSet},
};

const DISABLED_STATE: AState = AState::Ghost;

#[derive(Clone)]
pub(crate) struct UCharge {
    pub(super) base: UItemBase,
    fit_key: UFitKey,
    cont_item_key: UItemKey,
    projs: Projs,
    // Stores container state when charge is force disabled
    stored_cont_base_state: Option<AState>,
}
impl UCharge {
    pub(crate) fn new(
        item_id: ItemId,
        type_id: AItemId,
        fit_key: UFitKey,
        cont_item_key: UItemKey,
        cont_base_state: AState,
        force_disable: bool,
        src: &Src,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Self {
        let (base_state, stored_cont_base_state) = match force_disable {
            true => (DISABLED_STATE, Some(cont_base_state)),
            false => (cont_base_state, None),
        };
        Self {
            base: UItemBase::new(item_id, type_id, base_state, src, reuse_eupdates),
            fit_key,
            cont_item_key,
            projs: Projs::new(),
            stored_cont_base_state,
        }
    }
    // Item base methods
    pub(crate) fn get_item_id(&self) -> ItemId {
        self.base.get_item_id()
    }
    pub(crate) fn get_type_id(&self) -> AItemId {
        self.base.get_type_id()
    }
    pub(crate) fn set_type_id(&mut self, type_id: AItemId, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        self.base.set_type_id(type_id, reuse_eupdates, src);
    }
    pub(crate) fn get_group_id(&self) -> Option<AItemGrpId> {
        self.base.get_group_id()
    }
    pub(crate) fn get_category_id(&self) -> Option<AItemCatId> {
        self.base.get_category_id()
    }
    pub(crate) fn get_attrs(&self) -> Option<&RMap<AAttrId, AAttrVal>> {
        self.base.get_attrs()
    }
    pub(crate) fn get_effect_datas(&self) -> Option<&RMap<REffectKey, AItemEffectData>> {
        self.base.get_effect_datas()
    }
    pub(crate) fn get_defeff_key(&self) -> Option<Option<REffectKey>> {
        self.base.get_defeff_key()
    }
    pub(crate) fn get_skill_reqs(&self) -> Option<&RMap<AItemId, ASkillLevel>> {
        self.base.get_skill_reqs()
    }
    pub(crate) fn get_axt(&self) -> Option<&RItemAXt> {
        self.base.get_axt()
    }
    pub(crate) fn get_state(&self) -> AState {
        self.base.get_state()
    }
    pub(crate) fn set_state(&mut self, state: AState, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        match self.stored_cont_base_state {
            // Stored state set means charge is disabled
            Some(_) => {
                self.stored_cont_base_state = Some(state);
                reuse_eupdates.clear();
            }
            // Not disabled - proceed like with any other item
            None => self.base.set_state(state, reuse_eupdates, src),
        }
    }
    pub(crate) fn get_reffs(&self) -> Option<&RSet<REffectKey>> {
        self.base.get_reffs()
    }
    pub(in crate::ud::item) fn start_all_reffs(&self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        self.base.start_all_reffs(reuse_eupdates, src);
    }
    pub(in crate::ud::item) fn stop_all_reffs(&self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        self.base.stop_all_reffs(reuse_eupdates, src)
    }
    pub(in crate::ud::item) fn get_effect_key_mode(&self, effect_key: &REffectKey) -> EffectMode {
        self.base.get_effect_key_mode(effect_key)
    }
    pub(in crate::ud::item) fn set_effect_mode(
        &mut self,
        effect_id: AEffectId,
        effect_mode: EffectMode,
        reuse_eupdates: &mut UEffectUpdates,
        src: &Src,
    ) {
        self.base.set_effect_mode(effect_id, effect_mode, reuse_eupdates, src)
    }
    pub(in crate::ud::item) fn set_effect_modes(
        &mut self,
        effect_modes: impl Iterator<Item = (AEffectId, EffectMode)>,
        reuse_eupdates: &mut UEffectUpdates,
        src: &Src,
    ) {
        self.base.set_effect_modes(effect_modes, reuse_eupdates, src)
    }
    pub(crate) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::ud::item) fn src_changed(&mut self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        self.base.src_changed(reuse_eupdates, src);
    }
    pub(crate) fn get_force_disable(&self) -> bool {
        self.stored_cont_base_state.is_some()
    }
    pub(crate) fn set_force_disable(&mut self, force_disable: bool, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        match (force_disable, self.stored_cont_base_state) {
            // Attempt to enable when it's already enabled, or disable when it's disabled
            (true, Some(_)) | (false, None) => reuse_eupdates.clear(),
            // Turning force disable on
            (true, None) => {
                self.stored_cont_base_state = Some(self.get_state());
                self.base.set_state(DISABLED_STATE, reuse_eupdates, src);
            }
            // Turning force disable off
            (false, Some(stored_cont_base_state)) => {
                self.stored_cont_base_state = None;
                self.base.set_state(stored_cont_base_state, reuse_eupdates, src);
            }
        }
    }
    pub(crate) fn get_fit_key(&self) -> UFitKey {
        self.fit_key
    }
    pub(crate) fn get_cont_item_key(&self) -> UItemKey {
        self.cont_item_key
    }
    pub(crate) fn get_projs(&self) -> &Projs {
        &self.projs
    }
    pub(crate) fn get_projs_mut(&mut self) -> &mut Projs {
        &mut self.projs
    }
}
impl Named for UCharge {
    fn get_name() -> &'static str {
        "Charge"
    }
}
impl std::fmt::Display for UCharge {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}(item_id={}, type_id={})",
            Self::get_name(),
            self.get_item_id(),
            self.get_type_id(),
        )
    }
}
