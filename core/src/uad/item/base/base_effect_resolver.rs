use crate::{ac, ad, misc::EffectMode, rd, src::Src, uad::item::misc::EffectModes, util::RSet};

const ONLINE_EFFECT_ID: ad::AEffectId = ac::effects::ONLINE;

pub(crate) struct UadEffectUpdates {
    pub(crate) to_start: Vec<rd::RcEffect>,
    pub(crate) to_stop: Vec<rd::RcEffect>,
}
impl UadEffectUpdates {
    pub(crate) fn new() -> Self {
        Self {
            to_start: Vec::new(),
            to_stop: Vec::new(),
        }
    }
    pub(in crate::uad::item) fn clear(&mut self) {
        self.to_start.clear();
        self.to_stop.clear();
    }
}

pub(super) fn process_effects(
    reuse_eupdates: &mut UadEffectUpdates,
    reffs: &mut RSet<ad::AEffectId>,
    src: &Src,
    r_item: &rd::RItem,
    item_a_state: ad::AState,
    item_effect_modes: &EffectModes,
) {
    match item_a_state {
        ad::AState::Ghost => stop_all_effects(reuse_eupdates, reffs, src),
        _ => update_running_effects(reuse_eupdates, reffs, src, r_item, item_a_state, item_effect_modes),
    }
}

fn stop_all_effects(reuse_eupdates: &mut UadEffectUpdates, reffs: &mut RSet<ad::AEffectId>, src: &Src) {
    // We don't want waste time resolving effects when we want them to just stop (which happens
    // before e.g. item removal)
    reuse_eupdates.to_stop.extend(
        reffs
            .drain()
            .map(|a_effect_id| src.get_r_effect(&a_effect_id).unwrap().clone()),
    );
}

fn update_running_effects(
    reuse_eupdates: &mut UadEffectUpdates,
    reffs: &mut RSet<ad::AEffectId>,
    src: &Src,
    r_item: &rd::RItem,
    item_a_state: ad::AState,
    item_effect_modes: &EffectModes,
) {
    // Separate handling for the online effect
    let online_should_run = resolve_online_effect_status(r_item, item_effect_modes, item_a_state);
    let online_running = reffs.contains(&ONLINE_EFFECT_ID);
    // Whenever online effect status changes, it should be guaranteed that online effect is
    // available on the source level, so can just unwrap here
    if online_running && !online_should_run {
        reuse_eupdates.to_stop.push(src.get_r_effect_online().unwrap().clone());
    } else if !online_running && online_should_run {
        reuse_eupdates.to_start.push(src.get_r_effect_online().unwrap().clone());
    }
    for &a_effect_id in r_item.get_effect_datas_ids().keys() {
        // Online effect has already been handled
        if a_effect_id == ONLINE_EFFECT_ID {
            continue;
        }
        let r_effect = match src.get_r_effect(&a_effect_id) {
            Some(a_effect) => a_effect,
            None => continue,
        };
        let should_run = resolve_regular_effect_status(
            item_effect_modes,
            r_item.get_defeff_id(),
            item_a_state,
            online_should_run,
            r_effect,
        );
        let running = reffs.contains(&r_effect.get_id());
        if running && !should_run {
            reuse_eupdates.to_stop.push(r_effect.clone());
        } else if !running && should_run {
            reuse_eupdates.to_start.push(r_effect.clone());
        };
    }
    reffs.extend(reuse_eupdates.to_start.iter().map(|a_effect| a_effect.get_id()));
    for a_effect in reuse_eupdates.to_stop.iter() {
        reffs.remove(&a_effect.get_id());
    }
}

fn resolve_online_effect_status(r_item: &rd::RItem, item_effect_modes: &EffectModes, item_a_state: ad::AState) -> bool {
    if !r_item.has_online_effect() {
        return false;
    }
    match item_effect_modes.get(&ONLINE_EFFECT_ID) {
        // Since other effects from online category depend on the online effect in full compliance
        // mode, use simplified resolution for the online effect itself
        EffectMode::FullCompliance | EffectMode::StateCompliance => item_a_state >= ad::AState::Online,
        // Shouldn't run anything in ghost state even with force run mode
        EffectMode::ForceRun => item_a_state != ad::AState::Ghost,
        EffectMode::ForceStop => false,
    }
}

fn resolve_regular_effect_status(
    item_effect_modes: &EffectModes,
    item_defeff_id: Option<ad::AEffectId>,
    item_a_state: ad::AState,
    online_running: bool,
    r_effect: &rd::REffect,
) -> bool {
    // Ghosted items should never affect anything regardless of effect mode, so check it first
    // wherever applicable
    match item_effect_modes.get(&r_effect.get_id()) {
        EffectMode::FullCompliance => {
            item_a_state != ad::AState::Ghost
                && resolve_regular_effect_status_full(item_defeff_id, item_a_state, r_effect, online_running)
        }
        EffectMode::StateCompliance => item_a_state != ad::AState::Ghost && item_a_state >= r_effect.get_state(),
        EffectMode::ForceRun => item_a_state != ad::AState::Ghost,
        EffectMode::ForceStop => false,
    }
}

fn resolve_regular_effect_status_full(
    item_defeff_id: Option<ad::AEffectId>,
    item_a_state: ad::AState,
    r_effect: &rd::REffect,
    online_running: bool,
) -> bool {
    match r_effect.get_state() {
        ad::AState::Ghost => unreachable!("ghost state should never reach full resolver"),
        // Offline effects require item in offline+ state, and no fitting usage chance attribute
        // (not to run booster side effects by default)
        ad::AState::Offline => item_a_state >= r_effect.get_state() && r_effect.get_chance_attr_id().is_none(),
        // Online effects depend on 'online' effect, ignoring everything else
        ad::AState::Online => online_running,
        // Only default active effect is run, and only if item is in active+ state
        ad::AState::Active => {
            if r_effect.get_state() > item_a_state {
                return false;
            };
            match item_defeff_id {
                Some(defeff_id) => defeff_id == r_effect.get_id(),
                _ => false,
            }
        }
        // No additional restrictions for overload effects except for item being overloaded
        ad::AState::Overload => item_a_state >= r_effect.get_state(),
    }
}
