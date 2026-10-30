use crate::{
    EffectMode,
    ad::{AEffectId, AItemId},
    rd::{RData, REffectId, RItemBase, RItemFlexData, RState, RcItem},
    ud::{
        ItemId,
        item::{
            REffs, UEffectModeOverrideIter,
            base::{UEffectUpdates, process_effects},
            misc::UEffectModes,
        },
    },
};

/// Item base stores all the data every item should have
#[derive(Clone)]
pub(in crate::ud::item) struct UItemBase {
    // User-defined data
    item_id: ItemId,
    type_aid: AItemId,
    state: RState,
    effect_modes: UEffectModes,
    // Source-dependent data
    cache: Option<ItemBaseCache>,
}

#[derive(Clone)]
struct ItemBaseCache {
    r_item: RcItem,
    /// Running effects, are available only when adapted item is set
    reffs: REffs,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Constructors
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UItemBase {
    pub(in crate::ud::item) fn new(item_id: ItemId, type_aid: AItemId, state: RState, r_data: &RData) -> Self {
        Self {
            item_id,
            type_aid,
            state,
            effect_modes: UEffectModes::new(),
            cache: r_data.get_item_by_aid(&type_aid).map(|r_item| ItemBaseCache {
                r_item: r_item.clone(),
                reffs: REffs::new(),
            }),
        }
    }
    pub(in crate::ud::item::base) fn base_with_type_aid_not_loaded(
        item_id: ItemId,
        type_aid: AItemId,
        state: RState,
    ) -> Self {
        Self {
            item_id,
            type_aid,
            state,
            effect_modes: UEffectModes::new(),
            cache: None,
        }
    }
    pub(in crate::ud::item::base) fn base_with_r_item(item_id: ItemId, r_item: RcItem, state: RState) -> Self {
        Self {
            item_id,
            type_aid: r_item.base.aid,
            state,
            effect_modes: UEffectModes::new(),
            cache: Some(ItemBaseCache {
                r_item,
                reffs: REffs::new(),
            }),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Runtime data methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UItemBase {
    pub(in crate::ud::item) fn get_r_item_base(&self) -> Option<&RItemBase> {
        self.base_get_r_item().map(|v| &v.base)
    }
    pub(in crate::ud::item) fn get_r_item_flex_data(&self) -> Option<&RItemFlexData> {
        self.base_get_r_item().map(|v| &v.attr_data)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// User data methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UItemBase {
    pub(in crate::ud::item) fn get_item_id(&self) -> ItemId {
        self.item_id
    }
    pub(in crate::ud::item) fn get_type_aid(&self) -> AItemId {
        self.type_aid
    }
    pub(in crate::ud::item) fn set_type_aid(&mut self, type_aid: AItemId, r_data: &RData) {
        self.type_aid = type_aid;
        self.base_update_r_data(r_data);
    }
    pub(in crate::ud::item) fn get_state(&self) -> RState {
        self.state
    }
    pub(in crate::ud::item) fn set_state(&mut self, state: RState) {
        self.state = state;
    }
    pub(in crate::ud::item) fn get_effect_mode_by_rid(&self, effect_rid: &REffectId) -> EffectMode {
        self.effect_modes.get_by_rid(effect_rid)
    }
    pub(in crate::ud::item) fn get_effect_mode_by_aid(&self, effect_aid: &AEffectId) -> EffectMode {
        self.effect_modes.get_by_aid(effect_aid)
    }
    pub(in crate::ud::item) fn iter_effect_mode_overrides(&self) -> UEffectModeOverrideIter<'_> {
        self.effect_modes.iter_overrides_with_aids()
    }
    pub(in crate::ud::item) fn set_effect_mode(
        &mut self,
        effect_aid: AEffectId,
        effect_mode: EffectMode,
        r_data: &RData,
    ) -> EffectMode {
        self.effect_modes.set_by_aid(effect_aid, effect_mode, r_data)
    }
    pub(in crate::ud::item) fn set_effect_modes(
        &mut self,
        effect_modes: impl Iterator<Item = (AEffectId, EffectMode)>,
        r_data: &RData,
    ) {
        for (effect_aid, effect_mode) in effect_modes {
            self.effect_modes.set_by_aid(effect_aid, effect_mode, r_data);
        }
    }
    pub(in crate::ud::item::base) fn base_update_effect_modes(&mut self, r_data: &RData) {
        self.effect_modes.update_rids(r_data);
    }
    pub(in crate::ud::item) fn is_loaded(&self) -> bool {
        self.cache.is_some()
    }
    pub(in crate::ud::item) fn r_data_changed(&mut self, r_data: &RData) {
        self.base_update_effect_modes(r_data);
        self.base_update_r_data(r_data);
    }
    pub(in crate::ud::item::base) fn base_update_r_data(&mut self, r_data: &RData) {
        match r_data.get_item_by_aid(&self.type_aid) {
            Some(r_item) => self.base_set_r_item(r_item.clone()),
            None => self.cache = None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Running effect-specific methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UItemBase {
    pub(in crate::ud::item) fn get_reffs(&self) -> Option<&REffs> {
        self.cache.as_ref().map(|v| &v.reffs)
    }
    pub(in crate::ud::item) fn update_reffs(
        &mut self,
        reuse_eupdates: &mut UEffectUpdates,
        r_data: &RData,
        require_disabled_defeff: bool,
        force_active_nondefeff: bool,
    ) {
        // Always clear, regardless of item being loaded or not
        reuse_eupdates.clear();
        if let Some(cache) = &mut self.cache {
            process_effects(
                reuse_eupdates,
                &mut cache.reffs,
                r_data,
                &cache.r_item,
                self.state,
                &self.effect_modes,
                require_disabled_defeff,
                force_active_nondefeff,
            )
        }
    }
    pub(in crate::ud::item) fn stop_all_reffs(
        &mut self,
        reuse_eupdates: &mut UEffectUpdates,
        r_data: &RData,
        require_disabled_defeff: bool,
        force_active_nondefeff: bool,
    ) {
        reuse_eupdates.clear();
        if let Some(cache) = &mut self.cache {
            process_effects(
                reuse_eupdates,
                &mut cache.reffs,
                r_data,
                &cache.r_item,
                RState::Ghost,
                &self.effect_modes,
                require_disabled_defeff,
                force_active_nondefeff,
            )
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helpers for uitem bases
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UItemBase {
    pub(in crate::ud::item::base) fn base_set_type_aid_primitive(&mut self, type_aid: AItemId) {
        self.type_aid = type_aid;
    }
    pub(in crate::ud::item::base) fn base_set_type_aid_not_loaded(&mut self, type_aid: AItemId) {
        self.type_aid = type_aid;
        self.cache = None;
    }
    pub(in crate::ud::item::base) fn base_set_r_item(&mut self, r_item: RcItem) {
        self.type_aid = r_item.base.aid;
        match &mut self.cache {
            Some(cache) => {
                cache.r_item = r_item;
            }
            None => {
                self.cache = Some(ItemBaseCache {
                    r_item,
                    reffs: REffs::new(),
                })
            }
        }
    }
    pub(in crate::ud::item::base) fn base_get_r_item(&self) -> Option<&RcItem> {
        self.cache.as_ref().map(|v| &v.r_item)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "dev")]
mod debug {
    use super::*;
    use crate::{dbg::DebugResult, ud::UData};

    impl UItemBase {
        pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
            let Self {
                effect_modes,
                cache,
                // Not checked
                item_id: _,
                type_aid: _,
                state: _,
            } = self;
            if let Some(ItemBaseCache { r_item, reffs }) = cache {
                for cap_consumer in r_item.base.cap_consumers.iter() {
                    cap_consumer.consistency_check(u_data)?;
                }
                r_item.attr_data.consistency_check(u_data)?;
                for attr_rid in r_item.attr_data.attrs.keys() {
                    attr_rid.consistency_check(u_data)?;
                }
                for effect_rid in reffs.iter() {
                    effect_rid.consistency_check(u_data)?;
                }
            }
            effect_modes.consistency_check(u_data)?;
            Ok(())
        }
    }
}
