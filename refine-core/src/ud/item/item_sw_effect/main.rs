use crate::{
    EffectMode, ItemId, ItemKind,
    ad::{AEffectId, AItemId},
    rd::{RData, REffectId, RItemAttrData, RItemBase, RState},
    ud::item::{UEffectUpdates, UItemBase, bool_to_state_active, state_to_bool},
    util::{LibNamed, RSet},
};

#[derive(Clone)]
pub(crate) struct USwEffect {
    pub(super) base: UItemBase,
}
impl USwEffect {
    pub(crate) fn new(item_id: ItemId, type_aid: AItemId, state: bool, r_data: &RData) -> Self {
        Self {
            base: UItemBase::new(item_id, type_aid, bool_to_state_active(state), r_data),
        }
    }
    pub(in crate::ud::item) fn get_item_kind() -> ItemKind {
        ItemKind::SwEffect
    }
}
impl LibNamed for USwEffect {
    fn lib_get_name() -> &'static str {
        "USwEffect"
    }
}
impl std::fmt::Display for USwEffect {
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Item base methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl USwEffect {
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
    // Runtime data
    pub(in crate::ud::item) fn get_r_item_base(&self) -> Option<&RItemBase> {
        self.base.get_r_item_base()
    }
    pub(in crate::ud::item) fn get_r_item_attr_data(&self) -> Option<&RItemAttrData> {
        self.base.get_r_item_attr_data()
    }
    pub(in crate::ud::item) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::ud::item) fn r_data_changed(&mut self, r_data: &RData) {
        self.base.r_data_changed(r_data);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Item-specific methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl USwEffect {
    pub(crate) fn get_sw_effect_state(&self) -> bool {
        state_to_bool(self.base.get_state())
    }
    pub(crate) fn set_sw_effect_state(&mut self, state: bool) {
        self.base.set_state(bool_to_state_active(state))
    }
}
