//! Data customizations which are applied on adapted data generation.

use std::collections::hash_map::Entry;

use crate::{
    ad,
    nd::{N_EFFECTS, NEffect},
};

mod subsystem_slots;

pub(in crate::adg) fn customize(a_data: &mut ad::AData) {
    // Effect customization/fixes
    for n_effect in N_EFFECTS.iter() {
        if let Some(assigned) = assign_effect(a_data, n_effect) {
            match assigned {
                true => add_effect(a_data, n_effect),
                false => tracing::info!("effect {}: no items to assign effect to", n_effect.aid),
            }
        }
        update_effect(a_data, n_effect);
    }
    // Attribute value fixes
    subsystem_slots::fix_subsysem_slot_count(a_data);
}

fn add_effect(a_data: &mut ad::AData, n_effect: &NEffect) {
    if let Some(effect_maker) = n_effect.adg_make_effect_fn {
        let a_effect = effect_maker();
        match a_data.effects.entry(a_effect.id) {
            Entry::Occupied(_) => {
                tracing::info!("effect {}: already exists, not replacing", a_effect.id);
            }
            Entry::Vacant(entry) => {
                entry.insert(a_effect);
            }
        }
    }
}

fn update_effect(a_data: &mut ad::AData, n_effect: &NEffect) {
    if let Some(effect_updater) = n_effect.adg_update_effect_fn {
        let a_effect = match a_data.effects.get_mut(&n_effect.aid) {
            Some(a_effect) => a_effect,
            None => {
                tracing::info!("effect {}: not found for customization", n_effect.aid);
                return;
            }
        };
        effect_updater(a_effect);
    }
}

fn assign_effect(a_data: &mut ad::AData, n_effect: &NEffect) -> Option<bool> {
    let effect_assigner = n_effect.adg_assign_effect_fn?;
    Some(effect_assigner(&mut a_data.items))
}
