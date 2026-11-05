use super::cache::CseqCache;
use crate::{
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleActive, CycleDataFull, CycleSeq, seq_var_loop_sin::CSeqLoopSin},
        funcs,
    },
    ud::{UDrone, UItemId},
};

#[must_use]
pub(super) fn get_drone_cseq_map(
    cseq_cache: &mut CseqCache,
    ctx: &SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    drone: &UDrone,
) -> bool {
    if !drone.is_loaded() {
        return false;
    };
    for &effect_rid in drone.get_reffs().unwrap().iter() {
        let effect = ctx.u_data.r_data.get_effect_by_rid(effect_rid);
        if !effect.is_active_with_duration {
            continue;
        }
        let Some(duration) = funcs::get_effect_duration_s(ctx, calc, item_uid, effect) else {
            continue;
        };
        // Assume all drone effects just repeat themselves - ignoring all settings, self-destruction
        // flags, limited charges & reloads
        cseq_cache.insert(
            effect_rid,
            CycleSeq::LoopSin(CSeqLoopSin {
                data: CycleDataFull {
                    active: CycleActive {
                        duration,
                        chargedness: None,
                    },
                    soft_dt: None,
                },
                hard_dt: None,
            }),
        );
    }
    true
}
