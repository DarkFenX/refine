use std::{cmp::Ordering, collections::hash_map::Entry};

use crate::{
    api::{ModuleState, Op},
    num::Value,
    rd::{RAttrId, RState},
    sol::SolarSystem,
    svc::calc::CalcModInfo,
    ud::{UEffectUpdates, UItemId},
    util::RMap,
};

/// When fetching a stat, what to do with controllable affectors impacting that stat in a negative
/// way.
///
/// Good examples would be a ship mass stat: propulsion modules increase mass while active, and
/// armor plates increase mass while online.
#[derive(Copy, Clone)]
pub enum CtlAffectors {
    Unmodified,
    Deactivate,
    Offline,
}

#[derive(Copy, Clone)]
pub(in crate::api) enum AffectionDir {
    Increase,
    Decrease,
}

impl SolarSystem {
    // For now, it processes just direct modifications
    pub(in crate::api) fn internal_ctl_affectors_switch(
        &mut self,
        item_uid: UItemId,
        attr_rid: Option<RAttrId>,
        direction: AffectionDir,
        action: CtlAffectors,
        reuse_saved_states: &mut RMap<UItemId, RState>,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let Some(attr_rid) = attr_rid else {
            return;
        };
        if matches!(action, CtlAffectors::Unmodified) {
            return;
        }
        // Do nothing for items without fit
        let Some(fit_uid) = self.u_data.items.get(item_uid).get_fit_uid() else {
            return;
        };
        let Ok(mod_infos) = self.svc.iter_item_attr_mods(&self.u_data, item_uid, attr_rid) else {
            return;
        };
        for mod_info in mod_infos {
            // Ignore modifications which are not impacting requested direction
            if !needs_switch(&mod_info, direction) {
                continue;
            }
            // Ignore modifications which cannot be switched off
            let Some(new_state) = can_be_switched(&mod_info, action) else {
                continue;
            };
            // Ignore modifications with empty or complex affector definitions
            let affector = match mod_info.affectors.len() {
                1 => mod_info.affectors.get(1).unwrap(),
                _ => continue,
            };
            // Ignore non-module item kinds
            let Ok(affector_module) = self.u_data.items.get(affector.item_uid).dc_module() else {
                continue;
            };
            // Ignore modules from other fits
            if affector_module.get_fit_uid() != fit_uid {
                continue;
            }
            match reuse_saved_states.entry(affector.item_uid) {
                Entry::Occupied(mut entry) => {
                    // If modifier is coming from the same item, overwrite only if new state is
                    // lower
                    if new_state < *entry.get() {
                        entry.insert(new_state);
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert(new_state);
                }
            }
        }
        // Actually switch states
        for (&affector_uid, new_state) in reuse_saved_states.iter_mut() {
            let saved_state = self.u_data.items.get(affector_uid).get_state();
            self.internal_set_module_state(affector_uid, ModuleState::from_r_state(*new_state), reuse_eupdates);
            *new_state = saved_state;
        }
    }
    pub(in crate::api) fn internal_ctl_affectors_restore(
        &mut self,
        reuse_saved_states: &mut RMap<UItemId, RState>,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        for (&affector_uid, &saved_state) in reuse_saved_states.iter() {
            self.internal_set_module_state(affector_uid, ModuleState::from_r_state(saved_state), reuse_eupdates);
        }
        reuse_saved_states.clear();
    }
}

fn needs_switch(mod_info: &CalcModInfo, dir: AffectionDir) -> bool {
    // Implementation of this function is naive: it assumes base value is positive, and ignores
    // various complex math interactions (which are possible, but do not occur in EVE)
    match mod_info.op {
        Op::Add | Op::ExtraAdd | Op::PostPerc => match mod_info.initial_str.cmp(&Value::ZERO) {
            Ordering::Greater => matches!(dir, AffectionDir::Increase),
            Ordering::Equal => false,
            Ordering::Less => matches!(dir, AffectionDir::Decrease),
        },
        Op::Sub => match mod_info.initial_str.cmp(&Value::ZERO) {
            Ordering::Greater => matches!(dir, AffectionDir::Decrease),
            Ordering::Equal => false,
            Ordering::Less => matches!(dir, AffectionDir::Increase),
        },
        Op::PreMul | Op::PostMul | Op::ExtraMul => match mod_info.initial_str.cmp(&Value::ONE) {
            Ordering::Greater => matches!(dir, AffectionDir::Increase),
            Ordering::Equal => false,
            Ordering::Less => matches!(dir, AffectionDir::Decrease),
        },
        Op::PreDiv | Op::PostDiv => match mod_info.initial_str.cmp(&Value::ONE) {
            Ordering::Greater => matches!(dir, AffectionDir::Decrease),
            Ordering::Equal => false,
            Ordering::Less => matches!(dir, AffectionDir::Increase),
        },
        // Just ignore those modification types
        Op::BaseAssign | Op::PreAssign | Op::PostAssign | Op::MinLimit | Op::MaxLimit => false,
    }
}

fn can_be_switched(mod_info: &CalcModInfo, action: CtlAffectors) -> Option<RState> {
    let Some(state) = mod_info.state else {
        return None;
    };
    match action {
        CtlAffectors::Unmodified => None,
        CtlAffectors::Deactivate => match state {
            RState::Overload | RState::Active => Some(RState::Online),
            _ => None,
        },
        CtlAffectors::Offline => match state {
            RState::Overload | RState::Active => Some(RState::Online),
            RState::Online => Some(RState::Offline),
            _ => None,
        },
    }
}
