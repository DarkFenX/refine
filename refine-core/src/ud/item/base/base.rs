use crate::{
    EffectMode, ItemId,
    ad::{AEffectId, AItemId},
    rd::{RData, REffectId, RItemBase, RItemEffectOptions, RItemFlexData, RState, RcItem},
    ud::item::{
        REffs, UEffectModeOverrideIter,
        base::{UEffectUpdates, UItemEffectsResolver},
        misc::UEffectModes,
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
    // TODO: try moving into the cache, which owns running effects. The cache has no padding left,
    // TODO: so it would grow from 40 to 48 bytes (here the field fills padding after state),
    // TODO: growing the item and shifting inlining in unrelated code (up to 5% slower)
    // Promise about running effects: when set, they are exactly pregenerated effects of the current
    // runtime item up to this state, so a state change only starts or stops pregenerated effects
    // between the two states. None makes no promise (e.g. after resolution with effect mode
    // overrides), and the next resolution evaluates every effect
    reffs_pregen_up_to: Option<RState>,
}

#[derive(Clone)]
struct ItemBaseCache {
    r_item: RcItem,
    /// Running effects, are available only when runtime item is set
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
            reffs_pregen_up_to: Some(RState::Ghost),
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
            reffs_pregen_up_to: None,
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
            reffs_pregen_up_to: Some(RState::Ghost),
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
        self.base_get_r_item().map(|v| &v.flex_data)
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
    pub(in crate::ud::item) fn get_effect_mode_by_rid(&self, effect_rid: REffectId) -> EffectMode {
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
    ) -> bool {
        let mut changed = false;
        for (effect_aid, effect_mode) in effect_modes {
            changed |= self.effect_modes.set_by_aid(effect_aid, effect_mode, r_data) != effect_mode;
        }
        changed
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
        effect_options: RItemEffectOptions,
    ) {
        // Always clear, regardless of item being loaded or not
        reuse_eupdates.clear();
        if let Some(cache) = &mut self.cache {
            let has_mode_overrides = self.effect_modes.has_overrides_for_current_src();
            let reffs_pregen_up_to = match has_mode_overrides {
                true => None,
                false => self.reffs_pregen_up_to,
            };
            UItemEffectsResolver::process_effects(
                reuse_eupdates,
                &mut cache.reffs,
                r_data,
                &cache.r_item,
                reffs_pregen_up_to,
                self.state,
                &self.effect_modes,
                effect_options,
            );
            self.reffs_pregen_up_to = match has_mode_overrides {
                true => None,
                false => Some(self.state),
            };
        }
    }
    pub(in crate::ud::item) fn stop_all_reffs(
        &mut self,
        reuse_eupdates: &mut UEffectUpdates,
        r_data: &RData,
        effect_options: RItemEffectOptions,
    ) {
        reuse_eupdates.clear();
        if let Some(cache) = &mut self.cache {
            UItemEffectsResolver::process_effects(
                reuse_eupdates,
                &mut cache.reffs,
                r_data,
                &cache.r_item,
                None,
                RState::Ghost,
                &self.effect_modes,
                effect_options,
            );
            // No effects run in ghost state, whatever effect modes are
            self.reffs_pregen_up_to = Some(RState::Ghost);
        }
    }
    // Effects stopped by update_reffs or stop_all_reffs stay running until services have processed
    // their stop, so that services see their modifiers as belonging to running effects throughout
    pub(in crate::ud::item) fn remove_stopped_reffs(&mut self, eupdates: &UEffectUpdates) {
        if let Some(cache) = &mut self.cache {
            cache.reffs.remove_running(&eupdates.to_stop);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helpers for user item bases
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
            Some(cache) => cache.r_item = r_item,
            None => {
                self.cache = Some(ItemBaseCache {
                    r_item,
                    reffs: REffs::new(),
                })
            }
        }
        // Runtime item is replaced only while all effects of the item are stopped (type change,
        // mutation, data source switch)
        self.reffs_pregen_up_to = Some(RState::Ghost);
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
    use crate::{
        dbg::DebugResult,
        ud::{UData, item::base::UItemEffectsResolver},
    };

    impl UItemBase {
        pub(in crate::ud::item) fn consistency_check(
            &self,
            u_data: &UData,
            effect_options: RItemEffectOptions,
        ) -> DebugResult {
            let Self {
                effect_modes,
                cache,
                reffs_pregen_up_to,
                // Not checked
                item_id: _,
                type_aid: _,
                state: _,
            } = self;
            if let Some(ItemBaseCache { r_item, reffs }) = cache {
                for cap_consumer in r_item.base.cap_consumers.iter() {
                    cap_consumer.consistency_check(u_data)?;
                }
                r_item.flex_data.consistency_check(u_data)?;
                for attr_rid in r_item.flex_data.attrs.keys() {
                    attr_rid.consistency_check(u_data)?;
                }
                for effect_rid in reffs.iter() {
                    effect_rid.consistency_check(u_data)?;
                }
                UItemEffectsResolver::effects_pregen_consistency_check(&u_data.r_data, r_item)?;
                // Running effects have to be exactly pregenerated effects up to the stored state
                if let Some(reffs_pregen_up_to) = *reffs_pregen_up_to {
                    let pregen_entry = u_data
                        .r_data
                        .get_item_effects_pregen_entry(r_item.base.get_effects_pregen_id(effect_options));
                    let pregen_rids = pregen_entry.get_above_up_to(RState::Ghost, reffs_pregen_up_to);
                    if pregen_rids.len() != reffs.len()
                        || pregen_rids.iter().any(|effect_rid| !reffs.contains(*effect_rid))
                    {
                        return Err(Default::default());
                    }
                }
            }
            effect_modes.consistency_check(u_data)?;
            Ok(())
        }
    }
}
