use crate::{
    ac,
    ad::{AEffectId, AState},
    misc::EffectMode,
    rd::{REffect, REffectKey, RItem, RcEffect},
    src::Src,
    ud::item::misc::EffectModes,
    util::RSet,
};

const ONLINE_EFFECT_ID: AEffectId = ac::effects::ONLINE;

pub(crate) struct UAutochargeActivation {
    pub(crate) effect_key: REffectKey,
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
    reffs: &mut RSet<REffectKey>,
    src: &Src,
    item: &RItem,
    item_state: AState,
    item_effect_modes: &EffectModes,
) {
    match item_state {
        AState::Ghost => stop_all_effects(reuse_eupdates, reffs, src, item),
        _ => update_running_effects(reuse_eupdates, reffs, src, item, item_state, item_effect_modes),
    }
}

fn stop_all_effects(reuse_eupdates: &mut UEffectUpdates, reffs: &mut RSet<REffectKey>, src: &Src, item: &RItem) {
    // We don't want to waste time resolving effects when we want them to just stop (which happens
    // before e.g. item removal)
    reuse_eupdates.to_stop.reserve(reffs.len());
    for effect_key in reffs.drain() {
        let effect = src.get_effect(effect_key).clone();
        if effect.activates_charge_for_item(item) {
            reuse_eupdates.charge = Some(false);
        }
        if effect.activates_autocharge() {
            reuse_eupdates.autocharges.push(UAutochargeActivation {
                effect_key,
                active: false,
            });
        }
        reuse_eupdates.to_stop.push(effect);
    }
    reuse_eupdates
        .to_stop
        .extend(reffs.drain().map(|effect_key| src.get_effect(effect_key).clone()));
}

fn update_running_effects(
    reuse_eupdates: &mut UEffectUpdates,
    reffs: &mut RSet<REffectKey>,
    src: &Src,
    item: &RItem,
    item_state: AState,
    item_effect_modes: &EffectModes,
) {
    // Separate handling for the online effect
    let online_should_run = resolve_online_effect_status(item, item_effect_modes, item_state);
    let online_running = match src.get_online_effect_key() {
        Some(online_effect_key) => reffs.contains(&online_effect_key),
        None => false,
    };
    // Whenever online effect status changes, it should be guaranteed that online effect is
    // available on the source level, so can just unwrap here
    if online_running && !online_should_run {
        reuse_eupdates.to_stop.push(src.get_online_effect().unwrap().clone());
    } else if !online_running && online_should_run {
        reuse_eupdates.to_start.push(src.get_online_effect().unwrap().clone());
    }
    for &effect_key in item.get_effect_datas().keys() {
        // Online effect has already been handled
        if Some(effect_key) == src.get_online_effect_key() {
            continue;
        }
        let effect = src.get_effect(effect_key);
        let should_run = resolve_regular_effect_status(
            item_effect_modes,
            item.get_defeff_key(),
            item_state,
            online_should_run,
            effect,
        );
        let running = reffs.contains(&effect_key);
        if running && !should_run {
            reuse_eupdates.to_stop.push(effect.clone());
            if effect.activates_charge_for_item(item) {
                reuse_eupdates.charge = Some(false);
            }
            if effect.activates_autocharge() {
                reuse_eupdates.autocharges.push(UAutochargeActivation {
                    effect_key,
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
                    effect_key,
                    active: true,
                });
            }
        };
    }
    reffs.extend(reuse_eupdates.to_start.iter().map(|effect| effect.get_key()));
    for effect in reuse_eupdates.to_stop.iter() {
        reffs.remove(&effect.get_key());
    }
}

fn resolve_online_effect_status(item: &RItem, item_effect_modes: &EffectModes, item_state: AState) -> bool {
    if !item.has_online_effect() {
        return false;
    }
    match item_effect_modes.get_by_id(&ONLINE_EFFECT_ID) {
        // Since other effects from online category depend on the online effect in full compliance
        // mode, use simplified resolution for the online effect itself
        EffectMode::FullCompliance | EffectMode::StateCompliance => item_state >= AState::Online,
        // Shouldn't run anything in ghost state even with force run mode
        EffectMode::ForceRun => item_state != AState::Ghost,
        EffectMode::ForceStop => false,
    }
}

fn resolve_regular_effect_status(
    item_effect_modes: &EffectModes,
    item_defeff_key: Option<REffectKey>,
    item_state: AState,
    online_running: bool,
    effect: &REffect,
) -> bool {
    // Ghosted items should never affect anything regardless of effect mode, so check it first
    // wherever applicable
    match item_effect_modes.get_by_key(&effect.get_key()) {
        EffectMode::FullCompliance => {
            resolve_regular_effect_status_full(item_defeff_key, item_state, effect, online_running)
        }
        EffectMode::StateCompliance => item_state >= effect.get_state(),
        EffectMode::ForceRun => true,
        EffectMode::ForceStop => false,
    }
}

fn resolve_regular_effect_status_full(
    item_defeff_key: Option<REffectKey>,
    item_state: AState,
    effect: &REffect,
    online_running: bool,
) -> bool {
    match effect.get_state() {
        AState::Ghost => false,
        AState::Disabled => false,
        // Offline effects require item in offline+ state, and no fitting usage chance attribute
        // (not to run booster side effects by default)
        AState::Offline => item_state >= effect.get_state() && effect.get_chance_attr_id().is_none(),
        // Online effects depend on 'online' effect, ignoring everything else
        AState::Online => online_running,
        // Only default active effect is run, and only if item is in active+ state
        AState::Active => {
            if effect.get_state() > item_state {
                return false;
            };
            match item_defeff_key {
                Some(defeff_key) => defeff_key == effect.get_key(),
                _ => false,
            }
        }
        // No additional restrictions for overload effects except for item being overloaded
        AState::Overload => item_state >= effect.get_state(),
    }
}
