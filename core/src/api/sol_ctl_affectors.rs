use std::cmp::Ordering;

use crate::{
    api::{Modification, ModuleState, Op},
    num::Value,
    rd::RAttrId,
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
    pub(in crate::api) fn internal_ctl_affectors_switch(
        &mut self,
        item_uid: UItemId,
        attr_rid: RAttrId,
        direction: AffectionDir,
        reuse_saved_states: &mut RMap<UItemId, ModuleState>,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        // For now, process only direct modifications
        let Ok(mod_infos) = self.svc.iter_item_attr_mods(&self.u_data, item_uid, attr_rid) else {
            return;
        };
        for mod_info in mod_infos {
            // Ignore modifications which are not impacting requested direction
            if !needs_switch(&mod_info, direction) {
                continue;
            }
            // Ignore modifications with empty or complex affector definitions
            let affector = match mod_info.affectors.len() {
                1 => mod_info.affectors.get(1).unwrap(),
                _ => continue,
            };
            // Ignore non-module item kinds
            // Ignore modules from other fits
        }
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
