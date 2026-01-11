use either::Either;

use crate::{
    rd::REffectId,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleDataFull, CycleSeq, seq_inf::CSeqInf},
        funcs,
    },
    ud::{UDrone, UItemId},
    util::RMap,
};

pub(super) fn get_drone_cseq_map(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    drone: &UDrone,
    ignore_state: bool,
) -> Option<RMap<REffectId, CycleSeq>> {
    if !drone.is_loaded() {
        return None;
    };
    let mut cseq_map = RMap::new();
    let effect_rids = match ignore_state {
        true => Either::Left(drone.get_effects().unwrap().keys().copied()),
        false => Either::Right(drone.get_reffs().unwrap().iter().copied()),
    };
    for effect_rid in effect_rids {
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
        cseq_map.insert(
            effect_rid,
            CycleSeq::Inf(CSeqInf {
                data: CycleDataFull {
                    time: duration_s,
                    interrupt: None,
                    chargedness: None,
                },
            }),
        );
    }
    Some(cseq_map)
}
