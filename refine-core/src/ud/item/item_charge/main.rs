use crate::{
    EffectMode, ItemId, ItemKind,
    ad::{AEffectId, AItemId},
    rd::{RData, REffectId, RItemAttrData, RItemBase, RState},
    ud::{
        UFitId, UItemId,
        item::{UEffectModeOverrideIter, UEffectUpdates, UItemBase, UProjs},
    },
    util::RSet,
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
        type_aid: AItemId,
        fit_uid: UFitId,
        cont_item_uid: UItemId,
        activated: bool,
        force_disabled: bool,
        r_data: &RData,
    ) -> Self {
        Self {
            base: UItemBase::new(item_id, type_aid, get_state(activated, force_disabled), r_data),
            fit_uid,
            cont_item_uid,
            projs: UProjs::new(),
            activated,
            force_disabled,
        }
    }
    pub(in crate::ud::item) fn get_item_kind() -> ItemKind {
        ItemKind::Charge
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Item base methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UCharge {
    // User data
    pub(crate) fn get_item_id(&self) -> ItemId {
        self.base.get_item_id()
    }
    pub(crate) fn get_type_aid(&self) -> AItemId {
        self.base.get_type_aid()
    }
    pub(crate) fn set_type_aid(&mut self, type_aid: AItemId, r_data: &RData) {
        self.base.set_type_aid(type_aid, r_data);
    }
    pub(crate) fn get_state(&self) -> RState {
        self.base.get_state()
    }
    pub(crate) fn get_reffs(&self) -> Option<&RSet<REffectId>> {
        self.base.get_reffs()
    }
    pub(crate) fn update_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, r_data: &RData) {
        self.base.update_reffs(reuse_eupdates, r_data, false, false);
    }
    pub(in crate::ud::item) fn stop_all_reffs(&mut self, reuse_eupdates: &mut UEffectUpdates, r_data: &RData) {
        self.base.stop_all_reffs(reuse_eupdates, r_data, false, false)
    }
    pub(in crate::ud::item) fn get_effect_mode_by_rid(&self, effect_rid: &REffectId) -> EffectMode {
        self.base.get_effect_mode_by_rid(effect_rid)
    }
    pub(in crate::ud::item) fn iter_effect_mode_overrides(&self) -> UEffectModeOverrideIter<'_> {
        self.base.iter_effect_mode_overrides()
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
    // Runtime data
    pub(crate) fn get_r_item_base(&self) -> Option<&RItemBase> {
        self.base.get_r_item_base()
    }
    pub(crate) fn get_r_item_attr_data(&self) -> Option<&RItemAttrData> {
        self.base.get_r_item_attr_data()
    }
    pub(crate) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::ud::item) fn r_data_changed(&mut self, r_data: &RData) {
        self.base.r_data_changed(r_data);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Item-specific methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UCharge {
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

fn get_state(activated: bool, force_disabled: bool) -> RState {
    match force_disabled {
        true => RState::Disabled,
        false => match activated {
            true => RState::Active,
            false => RState::Offline,
        },
    }
}
