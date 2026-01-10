use crate::{
    ad::{AEffectId, AItemCatId, AItemGrpId, AItemId},
    misc::EffectMode,
    num::{SkillLevel, Value},
    rd::{RAttrId, REffectId, RItemAXt, RItemEffectData, RState, Src},
    ud::{
        ItemId, UFitId, UItemId,
        item::{UEffectUpdates, UItemBase, UProjs},
    },
    util::{LibNamed, RMap, RSet},
};

#[derive(Clone)]
pub(crate) struct UCharge {
    pub(super) base: UItemBase,
    fit_uid: UFitId,
    cont_item_uid: UItemId,
    projs: UProjs,
    activated: bool,
    force_disabled: bool,
}
impl UCharge {
    pub(crate) fn new(
        item_id: ItemId,
        type_id: AItemId,
        fit_uid: UFitId,
        cont_item_uid: UItemId,
        activated: bool,
        force_disabled: bool,
        src: &Src,
    ) -> Self {
        Self {
            base: UItemBase::new(item_id, type_id, get_state(activated, force_disabled), src),
            fit_uid,
            cont_item_uid,
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
    pub(crate) fn set_type_id(&mut self, type_id: AItemId, src: &Src) {
        self.base.set_type_id(type_id, src);
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
    pub(crate) fn get_effect_datas(&self) -> Option<&RMap<REffectId, RItemEffectData>> {
        self.base.get_effect_datas()
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
    pub(crate) fn get_reffs(&self) -> Option<&RSet<REffectId>> {
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
    pub(crate) fn get_activated(&self) -> bool {
        self.activated
    }
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
        // No changes to state - nothing to do but clear reusable data
        if self.force_disabled == force_disabled {
            return;
        }
        self.force_disabled = force_disabled;
        self.base.set_state(get_state(self.activated, self.force_disabled));
    }
    pub(crate) fn get_fit_uid(&self) -> UFitId {
        self.fit_uid
    }
    pub(crate) fn get_cont_item_uid(&self) -> UItemId {
        self.cont_item_uid
    }
    pub(crate) fn get_projs(&self) -> &UProjs {
        &self.projs
    }
    pub(crate) fn get_projs_mut(&mut self) -> &mut UProjs {
        &mut self.projs
    }
}
impl LibNamed for UCharge {
    fn lib_get_name() -> &'static str {
        "UCharge"
    }
}
impl std::fmt::Display for UCharge {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}(item_id={}, type_id={})",
            Self::lib_get_name(),
            self.get_item_id(),
            self.get_type_id(),
        )
    }
}

fn get_state(activated: bool, force_disabled: bool) -> RState {
    match force_disabled {
        true => RState::Disabled,
        false => match activated {
            true => RState::Active,
            false => RState::Offline,
        },
    }
}
