use crate::{
    misc::{Count, InfCount},
    svc::cycle::effect_charge_info::EffectChargeInfo,
};

pub(in crate::svc::cycle) fn get_eci_uncharged() -> EffectChargeInfo {
    EffectChargeInfo {
        fully_charged: InfCount::Count(Count::ZERO),
        part_charged: None,
        can_run_uncharged: true,
    }
}
