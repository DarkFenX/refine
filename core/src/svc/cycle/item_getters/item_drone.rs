use either::Either;

use crate::{
    rd::REffectKey,
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
    item_key: UItemId,
    drone: &UDrone,
    ignore_state: bool,
) -> Option<RMap<REffectKey, CycleSeq>> {
    if !drone.is_loaded() {
        return None;
    };
    let mut cseq_map = RMap::new();
    let effect_keys = match ignore_state {
        true => Either::Left(drone.get_effect_datas().unwrap().keys().copied()),
        false => Either::Right(drone.get_reffs().unwrap().iter().copied()),
    };
    for effect_key in effect_keys {
        let effect = ctx.u_data.src.get_effect(effect_key);
        if !effect.is_active_with_duration {
            continue;
        }
        let duration_s = match funcs::get_effect_duration_s(ctx, calc, item_key, effect) {
            Some(duration_s) => duration_s,
            None => continue,
        };
        // Assume all drone effects just repeat themselves - ignoring all settings, self-destruction
        // flags, limited charges & reloads
        cseq_map.insert(
            effect_key,
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
