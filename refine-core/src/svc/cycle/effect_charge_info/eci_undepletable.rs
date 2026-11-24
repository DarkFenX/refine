use crate::svc::cycle::{InfCount, effect_charge_info::EffectChargeInfo};

pub(in crate::svc::cycle) fn get_eci_undepletable() -> EffectChargeInfo {
    EffectChargeInfo {
        fully_charged: InfCount::Infinite,
        part_charged: None,
        can_run_uncharged: false,
    }
}
