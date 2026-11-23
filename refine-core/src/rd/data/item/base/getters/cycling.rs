use crate::{
    Count, PValue,
    nd::NEffectChargeDepl,
    rd::{REffect, REffectChargeLoc, REffectId, RItemBaseEffectData},
    util::{PDense, RMap},
};

pub(in crate::rd::data::item::base) fn get_has_optional_reload(
    capacity: PValue,
    item_effects: &[REffectId],
    r_effects: &PDense<REffectId, REffect>,
) -> bool {
    capacity > PValue::FLOAT_TOLERANCE
        && item_effects.iter().any(|&effect_rid| {
            let r_effect = r_effects.get(effect_rid).unwrap();
            r_effect.is_active_with_duration && can_run_uncharged(r_effect)
        })
}
fn can_run_uncharged(r_effect: &REffect) -> bool {
    match r_effect.charge.map(|r_charge| r_charge.location) {
        Some(REffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate(n_charge_rate))) => n_charge_rate.can_run_uncharged,
        Some(REffectChargeLoc::Loaded(NEffectChargeDepl::Crystal(n_charge_crystal))) => {
            n_charge_crystal.can_run_uncharged
        }
        _ => false,
    }
}

pub(in crate::rd::data::item::base) fn get_has_rearm(
    item_effect_datas: &RMap<REffectId, RItemBaseEffectData>,
    r_effects: &PDense<REffectId, REffect>,
) -> bool {
    item_effect_datas.iter().any(|(&effect_rid, effect_data)| {
        r_effects.get(effect_rid).unwrap().is_active_with_duration
            && effect_data
                .ability_charge_count
                .is_some_and(|charge_count| charge_count > Count::ZERO)
    })
}

pub(in crate::rd::data::item::base) fn get_has_spool(
    item_effects: &[REffectId],
    r_effects: &PDense<REffectId, REffect>,
) -> bool {
    item_effects
        .iter()
        .any(|&effect_rid| r_effects.get(effect_rid).unwrap().spool_attr_rids.is_some())
}
