use crate::{
    EffectMode, ItemId, ItemKind,
    ad::{AEffectId, AItemId},
    rd::{RData, REffectId, RItemBase, RItemFlexData, RState},
    ud::item::{
        REffs, UEffectModeOverrideIter, UEffectUpdates, UItemBase, UProjs, bool_to_state_active, state_to_bool,
    },
};

#[derive(Clone)]
pub(crate) struct UProjEffect {
    base: UItemBase,
    projs: UProjs,
}
impl UProjEffect {
    pub(crate) fn new(item_id: ItemId, type_aid: AItemId, proj_effect_state: bool, r_data: &RData) -> Self {
        Self {
            base: UItemBase::new(item_id, type_aid, bool_to_state_active(proj_effect_state), r_data),
            projs: UProjs::new(),
        }
    }
    pub(in crate::ud::item) fn get_item_kind() -> ItemKind {
        ItemKind::ProjEffect
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Item base methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UProjEffect {
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
    pub(in crate::ud::item) fn get_reffs(&self) -> Option<&REffs> {
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
    ) -> EffectMode {
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
    pub(in crate::ud::item) fn get_r_item_flex_data(&self) -> Option<&RItemFlexData> {
        self.base.get_r_item_flex_data()
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
impl UProjEffect {
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "dev")]
mod debug {
    use super::*;
    use crate::{dbg::DebugResult, ud::UData};

    impl UProjEffect {
        pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
            let Self { base, projs } = self;
            base.consistency_check(u_data)?;
            projs.consistency_check(u_data)?;
            // All projections are supposed to be without range on projected effect
            for (_projectee_uid, proj_data) in projs.iter() {
                if proj_data.is_some() {
                    return Err(Default::default());
                }
            }
            Ok(())
        }
    }
}
