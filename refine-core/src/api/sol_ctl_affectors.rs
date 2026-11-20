use std::{cmp::Ordering, collections::hash_map::Entry};

use crate::{
    ModuleState, Value,
    rd::{RAttrId, REffectId, RState},
    sol::{Savepoint, SolCore},
    svc::calc::CalcOp,
    ud::{UData, UItem, UItemId},
    util::RMap,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// When fetching a stat, what to do with player-controllable affectors which change it in the
/// direction the stat option names (its increasers or decreasers).
///
/// For example, propulsion modules increase ship mass while active, and armor plates while online.
#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone, Default)]
pub enum StatCtlAffectors {
    #[default]
    Unmodified,
    Deactivate,
    Offline,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(in crate::api) enum AffectionDir {
    Increase,
    Decrease,
}

impl SolCore {
    /// This function looks for modules which have to be switched to disable modifications upon
    /// certain attribute. Only modifications which are isolated within the same fit are considered.
    ///
    /// The function is extremely naive for simplicity:
    /// - it considers only modules
    /// - it considers only affectors which modify requested attribute directly
    /// - it ignores base attribute value - e.g. multiplications by >1 are considered as increases
    /// - it ignores possibility of complex math interactions (e.g. during calculation attribute can
    ///   flip sign of its value multiple times)
    /// - it ignores effect mode - it will attempt to deactivate/offline modules even if modifying
    ///   effect is in force-run mode
    /// - it ignores how modifications combine - a module is switched even if its modification does
    ///   not change the value at the moment (e.g. it is in the tail of a stacking penalty chain),
    ///   since it could take effect once other modules are switched
    ///
    /// Returns modules to switch, with states they have to be switched to.
    pub(in crate::api) fn internal_ctl_affectors_find(
        &mut self,
        item_uid: UItemId,
        attr_rid: Option<RAttrId>,
        action: StatCtlAffectors,
        direction: AffectionDir,
    ) -> RMap<UItemId, RState> {
        let mut new_states = RMap::new();
        let Some(attr_rid) = attr_rid else {
            return new_states;
        };
        if matches!(action, StatCtlAffectors::Unmodified) {
            return new_states;
        }
        // Do nothing for items without fit
        let Some(fit_uid) = self.u_data.items.get(item_uid).get_fit_uid() else {
            return new_states;
        };
        let Ok(mods) = self.svc.iter_item_attr_mods_op_str(&self.u_data, item_uid, attr_rid) else {
            return new_states;
        };
        for (affector_espec, op, strength) in mods {
            // Ignore modifications which are not impacting requested direction
            if !needs_switch(op, strength, direction) {
                continue;
            }
            // Ignore modifications which cannot be switched off
            let Some(new_state) = can_be_switched(&self.u_data, affector_espec.effect_rid, action) else {
                continue;
            };
            let affector_item = self.u_data.items.get(affector_espec.item_uid);
            // Ignore non-module item kinds
            match affector_item {
                UItem::Module(..) => (),
                _ => continue,
            }
            // Ignore modules from other fits
            if affector_item.get_fit_uid() != Some(fit_uid) {
                continue;
            }
            match new_states.entry(affector_espec.item_uid) {
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
        new_states
    }
    /// Switches modules to passed states. If there is anything to switch, returns savepoint which
    /// has to be rolled back to once switched states are no longer needed.
    pub(in crate::api) fn internal_ctl_affectors_switch(
        &mut self,
        new_states: RMap<UItemId, RState>,
    ) -> Option<Savepoint> {
        if new_states.is_empty() {
            return None;
        }
        let savepoint = self.internal_tx_savepoint();
        for (affector_uid, new_state) in new_states.into_iter() {
            self.internal_set_module_state(affector_uid, ModuleState::from_r_state(new_state));
        }
        Some(savepoint)
    }
}

fn needs_switch(op: CalcOp, strength: Value, dir: AffectionDir) -> bool {
    match op {
        CalcOp::Add | CalcOp::ExtraAdd | CalcOp::PostPerc | CalcOp::PostPercImmune => {
            match strength.cmp(&Value::ZERO) {
                Ordering::Greater => matches!(dir, AffectionDir::Increase),
                Ordering::Equal => false,
                Ordering::Less => matches!(dir, AffectionDir::Decrease),
            }
        }
        CalcOp::Sub => match strength.cmp(&Value::ZERO) {
            Ordering::Greater => matches!(dir, AffectionDir::Decrease),
            Ordering::Equal => false,
            Ordering::Less => matches!(dir, AffectionDir::Increase),
        },
        CalcOp::PreMul | CalcOp::PostMul | CalcOp::PostMulImmune | CalcOp::ExtraMul => {
            match strength.cmp(&Value::ONE) {
                Ordering::Greater => matches!(dir, AffectionDir::Increase),
                Ordering::Equal => false,
                Ordering::Less => matches!(dir, AffectionDir::Decrease),
            }
        }
        CalcOp::PreDiv | CalcOp::PostDiv => match strength.cmp(&Value::ONE) {
            Ordering::Greater => matches!(dir, AffectionDir::Decrease),
            Ordering::Equal => false,
            Ordering::Less => matches!(dir, AffectionDir::Increase),
        },
        // Just ignore modifications with those operators, hard to analyze them
        CalcOp::PreAssign | CalcOp::PostAssign => false,
    }
}

fn can_be_switched(u_data: &UData, effect_rid: REffectId, action: StatCtlAffectors) -> Option<RState> {
    let effect_state = u_data.r_data.get_effect_by_rid(effect_rid).state;
    match action {
        StatCtlAffectors::Unmodified => None,
        StatCtlAffectors::Deactivate => match effect_state {
            RState::Overload | RState::Active => Some(RState::Online),
            _ => None,
        },
        StatCtlAffectors::Offline => match effect_state {
            RState::Overload | RState::Active => Some(RState::Online),
            RState::Online => Some(RState::Offline),
            _ => None,
        },
    }
}
