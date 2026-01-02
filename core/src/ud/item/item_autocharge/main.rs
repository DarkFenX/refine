use crate::{
    ad::{AAttrVal, AEffectId, AItemCatId, AItemGrpId, AItemId, ASkillLevel, AState},
    def::ItemId,
    misc::EffectMode,
    rd::{RAttrId, REffectId, RItemAXt, RItemEffectData, Src},
    ud::{
        UFitId, UItemId,
        item::{UEffectUpdates, UItemBase, UProjs},
    },
    util::{Named, RMap, RSet},
};

#[derive(Clone)]
pub(crate) struct UAutocharge {
    pub(super) base: UItemBase,
    fit_key: UFitId,
    cont_item_key: UItemId,
    cont_effect_key: REffectId,
    projs: UProjs,
    activated: bool,
    force_disabled: bool,
}
impl UAutocharge {
    pub(crate) fn new(
        item_id: ItemId,
        type_id: AItemId,
        fit_key: UFitId,
        cont_item_key: UItemId,
        cont_effect_key: REffectId,
        activated: bool,
        force_disabled: bool,
        src: &Src,
    ) -> Self {
        Self {
            base: UItemBase::new(item_id, type_id, get_state(activated, force_disabled), src),
            fit_key,
            cont_item_key,
            cont_effect_key,
            projs: UProjs::new(),
            activated,
            force_disabled,
        }
    }
    // Item base methods
    pub(crate) fn get_item_id(&self) -> ItemId {
        self.base.get_item_id()
    }
    pub(crate) fn get_type_id(&self) -> AItemId {
        self.base.get_type_id()
    }
    pub(crate) fn get_group_id(&self) -> Option<AItemGrpId> {
        self.base.get_group_id()
    }
    pub(crate) fn get_category_id(&self) -> Option<AItemCatId> {
        self.base.get_category_id()
    }
    pub(crate) fn get_attrs(&self) -> Option<&RMap<RAttrId, AAttrVal>> {
        self.base.get_attrs()
    }
    pub(crate) fn get_effect_datas(&self) -> Option<&RMap<REffectId, RItemEffectData>> {
        self.base.get_effect_datas()
    }
    pub(crate) fn get_defeff_key(&self) -> Option<Option<REffectId>> {
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
    pub(in crate::ud::item) fn is_ice_harvester(&self) -> bool {
        self.base.is_ice_harvester()
    }
    pub(crate) fn get_reffs(&self) -> Option<&RSet<REffectId>> {
        self.base.get_reffs()
    }
    pub(crate) fn update_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        self.base.update_reffs(reuse_eupdates, src);
    }
    pub(in crate::ud::item) fn stop_all_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, src: &Src) {
        self.base.stop_all_reffs(reuse_eupdates, src)
    }
    pub(in crate::ud::item) fn get_effect_key_mode(&self, effect_key: &REffectId) -> EffectMode {
        self.base.get_effect_key_mode(effect_key)
    }
    pub(in crate::ud::item) fn set_effect_mode(&mut self, effect_id: AEffectId, effect_mode: EffectMode, src: &Src) {
        self.base.set_effect_mode(effect_id, effect_mode, src)
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
    pub(in crate::ud::item) fn src_changed(&mut self, _src: &Src) {
        // Just panic to expose attempts to reload it, since autocharges should never be reloaded.
        // Instead, they are removed and re-added when parent item changes.
        unreachable!("autocharges should be removed/added outside of autocharge item handler");
    }
    // Item-specific methods
    pub(crate) fn set_activated(&mut self, activated: bool) {
        // No changes to state - nothing to do
        if self.activated == activated {
            return;
        }
        self.activated = activated;
        self.base.set_state(get_state(self.activated, self.force_disabled));
    }
    pub(crate) fn get_force_disabled(&self) -> bool {
        self.force_disabled
    }
    pub(crate) fn set_force_disabled(&mut self, force_disabled: bool) {
        // No changes to state - nothing to do
        if self.force_disabled == force_disabled {
            return;
        }
        self.force_disabled = force_disabled;
        self.base.set_state(get_state(self.activated, self.force_disabled));
    }
    pub(crate) fn get_fit_key(&self) -> UFitId {
        self.fit_key
    }
    pub(crate) fn get_cont_item_key(&self) -> UItemId {
        self.cont_item_key
    }
    pub(crate) fn get_cont_effect_key(&self) -> REffectId {
        self.cont_effect_key
    }
    pub(crate) fn get_projs(&self) -> &UProjs {
        &self.projs
    }
    pub(crate) fn get_projs_mut(&mut self) -> &mut UProjs {
        &mut self.projs
    }
}
impl Named for UAutocharge {
    fn get_name() -> &'static str {
        "UAutocharge"
    }
}
impl std::fmt::Display for UAutocharge {
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

fn get_state(activated: bool, force_disabled: bool) -> AState {
    match force_disabled {
        true => AState::Disabled,
        false => match activated {
            true => AState::Active,
            false => AState::Offline,
        },
    }
}
