use crate::{
    ad::AEffectId,
    misc::EffectMode,
    rd::{RData, REffect, REffectId, RItem, RState, RcEffect},
    ud::item::misc::UEffectModes,
    util::RSet,
};

pub(crate) struct UAutochargeActivation {
    pub(crate) effect_rid: REffectId,
    pub(crate) active: bool,
}

pub(crate) struct UEffectUpdates {
    pub(crate) to_start: Vec<RcEffect>,
    pub(crate) to_stop: Vec<RcEffect>,
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
        self.to_start.clear();
        self.to_stop.clear();
        self.charge = None;
        self.autocharges.clear();
    }
}

pub(super) fn process_effects(
    reuse_eupdates: &mut UEffectUpdates,
    reffs: &mut RSet<REffectId>,
    r_data: &RData,
    item: &RItem,
    item_state: RState,
    item_effect_modes: &UEffectModes,
    require_disabled_defeff: bool,
    force_active_nondefeff: bool,
) {
    match item_state {
        RState::Ghost => stop_all_effects(reuse_eupdates, reffs, r_data, item),
        _ => update_running_effects(
            reuse_eupdates,
            reffs,
            r_data,
            item,
            item_state,
            item_effect_modes,
            require_disabled_defeff,
            force_active_nondefeff,
        ),
    }
}

fn stop_all_effects(reuse_eupdates: &mut UEffectUpdates, reffs: &mut RSet<REffectId>, r_data: &RData, item: &RItem) {
    // We don't want to waste time resolving effects when we want them to just stop (which happens
    // before e.g. item removal)
    reuse_eupdates.to_stop.reserve(reffs.len());
    for effect_rid in reffs.drain() {
        let effect = r_data.get_effect_by_rid(effect_rid).clone();
        if effect.activates_charge_for_item(item) {
            reuse_eupdates.charge = Some(false);
        }
        if effect.activates_autocharge() {
            reuse_eupdates.autocharges.push(UAutochargeActivation {
                effect_rid,
                active: false,
            });
        }
        reuse_eupdates.to_stop.push(effect);
    }
    reuse_eupdates.to_stop.extend(
        reffs
            .drain()
            .map(|effect_rid| r_data.get_effect_by_rid(effect_rid).clone()),
    );
}

fn update_running_effects(
    reuse_eupdates: &mut UEffectUpdates,
    reffs: &mut RSet<REffectId>,
    r_data: &RData,
    item: &RItem,
    item_state: RState,
    item_effect_modes: &UEffectModes,
    require_disabled_defeff: bool,
    force_active_nondefeff: bool,
) {
    // Separate handling for the online effect
    let online_should_run = resolve_online_effect_status(item, item_effect_modes, item_state);
    let online_running = match r_data.get_effect_consts().online {
        Some(online_effect_rid) => reffs.contains(&online_effect_rid),
        None => false,
    };
    // Whenever online effect status changes, it should be guaranteed that online effect is
    // available on the source level, so can just unwrap here
    if online_running && !online_should_run {
        reuse_eupdates.to_stop.push(r_data.get_online_effect().unwrap().clone());
    } else if !online_running && online_should_run {
        reuse_eupdates
            .to_start
            .push(r_data.get_online_effect().unwrap().clone());
    }
    for &effect_rid in item.effects.keys() {
        // Online effect has already been handled
        if Some(effect_rid) == r_data.get_effect_consts().online {
            continue;
        }
        let effect = r_data.get_effect_by_rid(effect_rid);
        let should_run = resolve_regular_effect_status(
            item_effect_modes,
            item.defeff_rid,
            item_state,
            effect,
            require_disabled_defeff,
            force_active_nondefeff,
            online_should_run,
        );
        let running = reffs.contains(&effect_rid);
        if running && !should_run {
            reuse_eupdates.to_stop.push(effect.clone());
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
            reuse_eupdates.to_start.push(effect.clone());
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
    reffs.extend(reuse_eupdates.to_start.iter().map(|effect| effect.rid));
    for effect in reuse_eupdates.to_stop.iter() {
        reffs.remove(&effect.rid);
    }
}

fn resolve_online_effect_status(item: &RItem, item_effect_modes: &UEffectModes, item_state: RState) -> bool {
    if !item.has_online_effect {
        return false;
    }
    match item_effect_modes.get_by_aid(&AEffectId::ONLINE) {
        // Since other effects from online category depend on the online effect in full compliance
        // mode, use simplified resolution for the online effect itself
        EffectMode::FullCompliance | EffectMode::StateCompliance => item_state >= RState::Online,
        // Shouldn't run anything in ghost state even with force run mode
        EffectMode::ForceRun => true,
        EffectMode::ForceStop => false,
    }
}

fn resolve_regular_effect_status(
    item_effect_modes: &UEffectModes,
    item_defeff_rid: Option<REffectId>,
    item_state: RState,
    effect: &REffect,
    require_disabled_defeff: bool,
    force_active_nondefeff: bool,
    online_running: bool,
) -> bool {
    // Ghosted items should never affect anything regardless of effect mode, so check it first
    // wherever applicable
    match item_effect_modes.get_by_rid(&effect.rid) {
        EffectMode::FullCompliance => resolve_regular_effect_status_full(
            item_defeff_rid,
            item_state,
            effect,
            require_disabled_defeff,
            force_active_nondefeff,
            online_running,
        ),
        EffectMode::StateCompliance => item_state >= effect.state,
        EffectMode::ForceRun => true,
        EffectMode::ForceStop => false,
    }
}

fn resolve_regular_effect_status_full(
    item_defeff_rid: Option<REffectId>,
    item_state: RState,
    effect: &REffect,
    require_disabled_defeff: bool,
    force_active_nondefeff: bool,
    online_running: bool,
) -> bool {
    match effect.state {
        RState::Ghost => false,
        // All effects with disabled state are run if item is in disabled+ state, unless caller
        // requested extra requirement for them to be default
        RState::Disabled => {
            if effect.state > item_state {
                return false;
            };
            match require_disabled_defeff {
                true => match item_defeff_rid {
                    Some(defeff_rid) => defeff_rid == effect.rid,
                    None => false,
                },
                false => true,
            }
        }
        // Offline effects require item in offline+ state, and no fitting usage chance attribute
        // (not to run booster side effects by default)
        RState::Offline => item_state >= effect.state && effect.chance_attr_rid.is_none(),
        // Online effects depend on 'online' effect, ignoring everything else
        RState::Online => online_running,
        // Only default active effect is run, and only if item is in active+ state, except for the
        // case when all active effects are requested to run
        RState::Active => {
            if effect.state > item_state {
                return false;
            };
            match force_active_nondefeff {
                true => true,
                false => match item_defeff_rid {
                    Some(defeff_rid) => defeff_rid == effect.rid,
                    None => false,
                },
            }
        }
        // No additional restrictions for overload effects except for item being overloaded
        RState::Overload => item_state >= effect.state,
    }
}
