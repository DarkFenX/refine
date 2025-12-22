use crate::{svc::cycle::effect_charge_info::EffectChargeInfo, util::InfCount};

pub(in crate::svc::cycle) fn get_eci_undepletable() -> EffectChargeInfo {
    EffectChargeInfo {
        fully_charged: InfCount::Infinite,
        part_charged: None,
        can_run_uncharged: false,
    }
}
