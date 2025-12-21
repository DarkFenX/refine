use crate::{svc::cycle::charged_info::EffectChargedInfo, util::InfCount};

pub(in crate::svc::cycle) fn get_uncharged_charged_info() -> EffectChargedInfo {
    EffectChargedInfo {
        fully_charged: InfCount::Count(0),
        part_charged: None,
        can_run_uncharged: true,
    }
}
