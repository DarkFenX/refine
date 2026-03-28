use super::shared::{CseqMap, CyclingOptions};
use crate::{
    num::PValue,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleDataFull, CycleSeq},
    },
    ud::{UFighter, UItemId},
};

struct FtrEffectInfo {
    cseq: CycleSeq<CycleDataFull>,
    rearm: Option<FtrEffectRearmInfo>,
}

#[derive(Copy, Clone)]
struct FtrEffectRearmInfo {
    duration_until_rearm: PValue,
    full_rearm_duration: PValue,
    charge_rearm_duration: PValue,
}

#[must_use]
pub(super) fn get_fighter_cseq_map(
    reuse_cseq_map: &mut CseqMap,
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    fighter: &UFighter,
    options: CyclingOptions,
    ignore_state: bool,
) -> bool {
    if !fighter.is_loaded() {
        return false;
    };
    false
}
