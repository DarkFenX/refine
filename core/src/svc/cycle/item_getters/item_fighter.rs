use super::shared::CyclingOptions;
use crate::{
    num::PValue,
    rd::REffectId,
    svc::{SvcCtx, calc::Calc, cycle::CycleSeq},
    ud::{UFighter, UItemId},
    util::RMap,
};

struct FtrEffectInfo {
    cseq: CycleSeq,
    rearm: Option<FtrEffectRearmInfo>,
}

#[derive(Copy, Clone)]
struct FtrEffectRearmInfo {
    time_until_rearm: PValue,
    full_rearm_time: PValue,
    charge_rearm_time: PValue,
}

pub(super) fn get_fighter_cseq_map(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    fighter: &UFighter,
    options: CyclingOptions,
    ignore_state: bool,
) -> Option<RMap<REffectId, CycleSeq>> {
    if !fighter.is_loaded() {
        return None;
    };
    None
}
