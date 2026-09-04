use crate::{
    EffectMode, ItemId, ItemKind,
    ad::{AEffectId, AItemId},
    rd::{RData, REffectId, RItemAttrData, RItemBase, RState},
    ud::{
        UFitId,
        item::{UEffectUpdates, UItemBase, bool_to_state_offline, state_to_bool},
    },
    util::RSet,
};

#[derive(Clone)]
pub(crate) struct UCharacter {
    pub(super) base: UItemBase,
    fit_uid: UFitId,
}
impl UCharacter {
    pub(crate) fn new(
        item_id: ItemId,
        type_aid: AItemId,
        fit_uid: UFitId,
        character_state: bool,
        r_data: &RData,
    ) -> Self {
        Self {
            base: UItemBase::new(item_id, type_aid, bool_to_state_offline(character_state), r_data),
            fit_uid,
        }
    }
    pub(in crate::ud::item) fn get_item_kind() -> ItemKind {
        ItemKind::Character
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Item base methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UCharacter {
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
    pub(in crate::ud::item) fn get_reffs(&self) -> Option<&RSet<REffectId>> {
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
    pub(in crate::ud::item) fn iter_effect_mode_overrides(
        &self,
    ) -> impl ExactSizeIterator<Item = (AEffectId, EffectMode)> {
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
    pub(in crate::ud::item) fn get_r_item_base(&self) -> Option<&RItemBase> {
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
impl UCharacter {
    pub(crate) fn get_character_state(&self) -> bool {
        state_to_bool(self.base.get_state())
    }
    pub(crate) fn set_character_state(&mut self, state: bool) {
        self.base.set_state(bool_to_state_offline(state))
    }
    pub(crate) fn get_fit_uid(&self) -> UFitId {
        self.fit_uid
    }
}
