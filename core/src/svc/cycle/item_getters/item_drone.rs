use super::map::CseqMap;
use crate::{
    num::PValue,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleDataFull, CycleSeq, seq_inf::CSeqInf},
        funcs,
    },
    ud::{UDrone, UItemId},
};

#[must_use]
pub(super) fn get_drone_cseq_map(
    reuse_cseq_map: &mut CseqMap,
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    drone: &UDrone,
) -> bool {
    if !drone.is_loaded() {
        return false;
    };
    reuse_cseq_map.clear();
    for &effect_rid in drone.get_reffs().unwrap().iter() {
        let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
        if !effect.is_active_with_duration {
            continue;
        }
        let duration_s = match funcs::get_effect_duration_s(ctx, calc, item_uid, effect) {
            Some(duration_s) => duration_s,
            None => continue,
        };
        // Assume all drone effects just repeat themselves - ignoring all settings, self-destruction
        // flags, limited charges & reloads
        reuse_cseq_map.insert(
            effect_rid,
            CycleSeq::Inf(CSeqInf {
                data: CycleDataFull {
                    active_duration: duration_s,
                    soft_dt_duration: PValue::ZERO,
                    hard_dt_duration: PValue::ZERO,
                    interrupt: None,
                    chargedness: None,
                },
            }),
        );
    }
    true
}
