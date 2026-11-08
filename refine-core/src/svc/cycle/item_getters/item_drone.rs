use super::{cache::CseqBuild, cycling_options::DroneCyclingOptions};
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
pub(super) fn build_drone_cseq_map(
    cseq_build: &mut CseqBuild,
    ctx: &SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    drone: &UDrone,
    drone_co: DroneCyclingOptions,
) -> bool {
    // Drones read no cycling options
    let DroneCyclingOptions = drone_co;
    if !drone.is_loaded() {
        return false;
    };
    for effect_rid in drone.get_reffs().unwrap().iter() {
        let effect = ctx.u_data.r_data.get_effect_by_rid(effect_rid);
        if !effect.is_active_with_duration {
            continue;
        }
        let Some(duration) = funcs::get_effect_duration_s(ctx, calc, item_uid, effect) else {
            continue;
        };
        // Assume all drone effects just repeat themselves - ignoring all settings, self-destruction
        // flags, limited charges & reloads
        cseq_build.insert(
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
