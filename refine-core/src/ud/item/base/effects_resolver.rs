use std::cmp::Ordering;

use crate::{
    ad::AEffectId,
    rd::{RData, REffectId, RItem, RItemEffectOptions, RItemEffectsResolver, RState},
    ud::item::{REffs, misc::UEffectModes},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Resolver
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Updates running effects of an item, through pregenerated effects or dynamic resolution.
pub(super) struct UItemEffectsResolver;
impl UItemEffectsResolver {
    pub(super) fn process_effects(
        reuse_eupdates: &mut UEffectUpdates,
        reffs: &mut REffs,
        r_data: &RData,
        item: &RItem,
        reffs_pregen_up_to: Option<RState>,
        item_state: RState,
        item_effect_modes: &UEffectModes,
        effect_options: RItemEffectOptions,
    ) {
        match (reffs_pregen_up_to, item_state) {
            (_, RState::Ghost) => Self::stop_all_effects(reuse_eupdates, reffs, r_data, item),
            (Some(reffs_pregen_up_to), _) => Self::update_running_effects_pregen(
                reuse_eupdates,
                reffs,
                r_data,
                item,
                reffs_pregen_up_to,
                item_state,
                effect_options,
            ),
            (None, _) => Self::update_running_effects_dynamic(
                reuse_eupdates,
                reffs,
                r_data,
                item,
                item_state,
                item_effect_modes,
                effect_options,
            ),
        }
    }
    fn stop_all_effects(reuse_eupdates: &mut UEffectUpdates, reffs: &mut REffs, r_data: &RData, item: &RItem) {
        // We don't want to waste time resolving effects when we want them to just stop (which
        // happens before e.g. item removal)
        reuse_eupdates.to_stop.reserve(reffs.len());
        for effect_rid in reffs.drain() {
            let effect = r_data.get_effect_by_rid(effect_rid);
            if effect.activates_charge_for_item(item) {
                reuse_eupdates.charge = Some(false);
            }
            if effect.activates_autocharge() {
                reuse_eupdates.autocharges.push(UAutochargeActivation {
                    effect_rid,
                    active: false,
                });
            }
            reuse_eupdates.to_stop.push(effect_rid);
        }
    }
    fn update_running_effects_pregen(
        reuse_eupdates: &mut UEffectUpdates,
        reffs: &mut REffs,
        r_data: &RData,
        item: &RItem,
        reffs_pregen_up_to: RState,
        item_state: RState,
        effect_options: RItemEffectOptions,
    ) {
        // Running effects are pregenerated effects up to the given state, and there are no effect
        // mode overrides now, so pregenerated effects between the old and the new state are started
        // or stopped as is
        let pregen_entry = r_data.get_item_effects_pregen_entry(item.base.get_effects_pregen_id(effect_options));
        match item_state.cmp(&reffs_pregen_up_to) {
            Ordering::Equal => (),
            Ordering::Greater => {
                let effect_rids = pregen_entry.get_above_up_to(reffs_pregen_up_to, item_state);
                reuse_eupdates.to_start.reserve(effect_rids.len());
                for &effect_rid in effect_rids {
                    let effect = r_data.get_effect_by_rid(effect_rid);
                    if effect.activates_charge_for_item(item) {
                        reuse_eupdates.charge = Some(true);
                    }
                    if effect.activates_autocharge() {
                        reuse_eupdates.autocharges.push(UAutochargeActivation {
                            effect_rid,
                            active: true,
                        });
                    }
                    reuse_eupdates.to_start.push(effect_rid);
                }
                reffs.extend(effect_rids.iter().copied());
            }
            Ordering::Less => {
                let effect_rids = pregen_entry.get_above_up_to(item_state, reffs_pregen_up_to);
                reuse_eupdates.to_stop.reserve(effect_rids.len());
                for &effect_rid in effect_rids {
                    let effect = r_data.get_effect_by_rid(effect_rid);
                    if effect.activates_charge_for_item(item) {
                        reuse_eupdates.charge = Some(false);
                    }
                    if effect.activates_autocharge() {
                        reuse_eupdates.autocharges.push(UAutochargeActivation {
                            effect_rid,
                            active: false,
                        });
                    }
                    reuse_eupdates.to_stop.push(effect_rid);
                    reffs.remove(effect_rid);
                }
            }
        }
    }
    fn update_running_effects_dynamic(
        reuse_eupdates: &mut UEffectUpdates,
        reffs: &mut REffs,
        r_data: &RData,
        item: &RItem,
        item_state: RState,
        item_effect_modes: &UEffectModes,
        effect_options: RItemEffectOptions,
    ) {
        // Separate handling for the online effect
        let online_should_run = RItemEffectsResolver::resolve_online_effect_status(
            &item.base,
            item_effect_modes.get_by_aid(&AEffectId::ONLINE),
            item_state,
        );
        let online_effect_rid = r_data.get_effect_consts().online;
        let online_running = match online_effect_rid {
            Some(online_effect_rid) => reffs.contains(online_effect_rid),
            None => false,
        };
        // Whenever online effect status changes, it should be guaranteed that online effect is
        // available on the source level, so can just unwrap here
        if online_running && !online_should_run {
            reuse_eupdates.to_stop.push(online_effect_rid.unwrap());
        } else if !online_running && online_should_run {
            reuse_eupdates.to_start.push(online_effect_rid.unwrap());
        }
        for &effect_rid in item.base.effect_rids.iter() {
            // Online effect has already been handled
            if Some(effect_rid) == online_effect_rid {
                continue;
            }
            let effect = r_data.get_effect_by_rid(effect_rid);
            let should_run = RItemEffectsResolver::resolve_effect_status(
                &item.base,
                effect,
                item_effect_modes.get_by_rid(effect_rid),
                item_state,
                effect_options,
                online_should_run,
            );
            let running = reffs.contains(effect_rid);
            if running && !should_run {
                reuse_eupdates.to_stop.push(effect_rid);
                if effect.activates_charge_for_item(item) {
                    reuse_eupdates.charge = Some(false);
                }
                if effect.activates_autocharge() {
                    reuse_eupdates.autocharges.push(UAutochargeActivation {
                        effect_rid,
                        active: false,
                    });
                }
            } else if !running && should_run {
                reuse_eupdates.to_start.push(effect_rid);
                if effect.activates_charge_for_item(item) {
                    reuse_eupdates.charge = Some(true);
                }
                if effect.activates_autocharge() {
                    reuse_eupdates.autocharges.push(UAutochargeActivation {
                        effect_rid,
                        active: true,
                    });
                }
            };
        }
        reffs.extend(reuse_eupdates.to_start.iter().copied());
        for &effect_rid in reuse_eupdates.to_stop.iter() {
            reffs.remove(effect_rid);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Auxiliary entities
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(crate) struct UAutochargeActivation {
    pub(crate) effect_rid: REffectId,
    pub(crate) active: bool,
}

pub(crate) struct UEffectUpdates {
    pub(crate) to_start: Vec<REffectId>,
    pub(crate) to_stop: Vec<REffectId>,
    // Fields which contain info about (auto)charge activation/deactivation
    pub(crate) charge: Option<bool>,
    pub(crate) autocharges: Vec<UAutochargeActivation>,
}
impl UEffectUpdates {
    pub(crate) fn new() -> Self {
        Self {
            to_start: Vec::new(),
            to_stop: Vec::new(),
            charge: None,
            autocharges: Vec::new(),
        }
    }
    pub(in crate::ud::item) fn clear(&mut self) {
        let Self {
            to_start,
            to_stop,
            charge,
            autocharges,
        } = self;
        to_start.clear();
        to_stop.clear();
        *charge = None;
        autocharges.clear();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "dev")]
mod debug {
    use super::*;
    use crate::dbg::DebugResult;

    impl UItemEffectsResolver {
        pub(in crate::ud::item::base) fn effects_pregen_consistency_check(r_data: &RData, item: &RItem) -> DebugResult {
            // Pregenerated effects of every state and options have to match dynamic resolution
            // without effect mode overrides
            let effect_modes = UEffectModes::new();
            for effect_options in RItemEffectOptions::ALL {
                let pregen_entry =
                    r_data.get_item_effects_pregen_entry(item.base.get_effects_pregen_id(effect_options));
                for &state in RState::above_up_to(RState::Ghost, RState::Overload) {
                    let mut reffs = REffs::new();
                    let mut eupdates = UEffectUpdates::new();
                    Self::update_running_effects_dynamic(
                        &mut eupdates,
                        &mut reffs,
                        r_data,
                        item,
                        state,
                        &effect_modes,
                        effect_options,
                    );
                    let pregen_rids = pregen_entry.get_above_up_to(RState::Ghost, state);
                    if pregen_rids.len() != reffs.len()
                        || pregen_rids.iter().any(|effect_rid| !reffs.contains(*effect_rid))
                    {
                        return Err(Default::default());
                    }
                }
            }
            Ok(())
        }
    }
}
