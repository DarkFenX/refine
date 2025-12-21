use crate::{svc::cycle::charged_info::EffectChargedInfo, util::InfCount};

pub(in crate::svc::cycle) fn get_undepletable_charged_info() -> EffectChargedInfo {
    EffectChargedInfo {
        fully_charged: InfCount::Infinite,
        part_charged: None,
        can_run_uncharged: false,
    }
}
