use crate::{svc::cycle::effect_charge_info::EffectChargeInfo, util::InfCount};

pub(in crate::svc::cycle) fn get_eci_uncharged() -> EffectChargeInfo {
    EffectChargeInfo {
        fully_charged: InfCount::Count(0),
        part_charged: None,
        can_run_uncharged: true,
    }
}
