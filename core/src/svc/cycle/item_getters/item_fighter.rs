use either::Either;

use super::{
    map::CseqMap,
    shared::{CyclingOptions, SelfKillerInfo},
};
use crate::{
    misc::{InfCount, RearmMinion},
    num::{Count, PValue},
    rd::REffectId,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CSeqInf, CSeqLim, CSeqLoopLimSin, CycleDataFull, CycleInterrupt, CycleSeq},
        funcs,
    },
    ud::{UFighter, UItemId},
    util::RMap,
};

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
    let effect_rids = match ignore_state {
        true => Either::Left(fighter.get_effects().unwrap().keys().copied()),
        false => Either::Right(fighter.get_reffs().unwrap().iter().copied()),
    };
    reuse_cseq_map.clear();
    let mut self_killers = Vec::new();
    match options {
        CyclingOptions::Burst => {
            for effect_rid in effect_rids {
                fill_effect_cseq_burst(
                    reuse_cseq_map,
                    &mut self_killers,
                    ctx,
                    calc,
                    item_uid,
                    fighter,
                    effect_rid,
                )
            }
            // If there are any self-killer effects, choose the fastest one, and discard all other effects
            if !self_killers.is_empty() {
                let fastest_sk_effect_rid = self_killers
                    .into_iter()
                    .min_by_key(|sk_info| sk_info.duration)
                    .unwrap()
                    .effect_rid;
                reuse_cseq_map.retain(|&k, _| k == fastest_sk_effect_rid);
            }
            true
        }
        CyclingOptions::Sim(sim_options) => {
            let rearm_minions = ctx.u_data.get_item_rearm_minion(item_uid, sim_options.rearm_minions);
            match rearm_minions {
                RearmMinion::Disabled => {
                    for effect_rid in effect_rids {
                        fill_effect_cseq_sim_no_rearm(
                            reuse_cseq_map,
                            &mut self_killers,
                            ctx,
                            calc,
                            item_uid,
                            fighter,
                            effect_rid,
                        )
                    }
                    // If there are any self-killer effects, choose the fastest one, and discard all other effects
                    if !self_killers.is_empty() {
                        let fastest_sk_effect_rid = self_killers
                            .into_iter()
                            .min_by_key(|sk_info| sk_info.duration)
                            .unwrap()
                            .effect_rid;
                        reuse_cseq_map.retain(|&k, _| k == fastest_sk_effect_rid);
                    }
                    true
                }
                RearmMinion::OnFirstEmpty => {
                    let mut ext_cseq_map = RMap::new();
                    for effect_rid in effect_rids {
                        fill_effect_cseq_sim_rearm(
                            &mut ext_cseq_map,
                            &mut self_killers,
                            ctx,
                            calc,
                            item_uid,
                            fighter,
                            effect_rid,
                        )
                    }
                    // If there are any self-killer effects, choose the fastest one, and discard all other effects
                    if !self_killers.is_empty() {
                        let fastest_sk_effect_rid = self_killers
                            .into_iter()
                            .min_by_key(|sk_info| sk_info.duration)
                            .unwrap()
                            .effect_rid;
                        let fastest_sk_cseq = ext_cseq_map.get(&fastest_sk_effect_rid).unwrap().cseq;
                        reuse_cseq_map.insert(fastest_sk_effect_rid, fastest_sk_cseq);
                        return true;
                    }
                    process_refuel(reuse_cseq_map, ext_cseq_map);
                    true
                }
            }
        }
    }
}

struct CommonInfo {
    kills_item: bool,
    duration: PValue,
    duration_with_cd: PValue,
    int_cd: bool,
    charge_count: InfCount,
    charge_reload_duration: PValue,
}

fn get_common_info(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    fighter: &UFighter,
    effect_rid: REffectId,
) -> Option<CommonInfo> {
    let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
    if !effect.is_active_with_duration {
        return None;
    }
    // No appropriate duration - effect does not cycle
    let duration = match funcs::get_effect_duration_s(ctx, calc, item_uid, effect) {
        Some(duration) => duration,
        None => return None,
    };
    let effect_data = fighter.get_effects().unwrap().get(&effect_rid).unwrap();
    let charge_count = match effect_data.charge_count {
        Some(charge_count) => InfCount::Count(charge_count),
        None => InfCount::Infinite,
    };
    // Completely skip effects which can't cycle
    if charge_count == InfCount::Count(Count::ZERO) {
        return None;
    }
    // For fighter abilities, cooldown starts as soon as effect starts cycling. It typically is
    // longer than duration, but data format does not guarantee that
    let duration_with_cd = duration.max(effect_data.cooldown_s);
    // Assume any cooldown interrupts cycling, even if it shorter than ability cycle
    let int_cd = effect_data.cooldown_s > PValue::FLOAT_TOLERANCE;
    Some(CommonInfo {
        kills_item: effect.kills_item,
        duration,
        duration_with_cd,
        int_cd,
        charge_count,
        charge_reload_duration: effect_data.charge_reload_duration,
    })
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// No rearm considered
////////////////////////////////////////////////////////////////////////////////////////////////////
fn fill_effect_cseq_burst(
    cseq_map: &mut CseqMap,
    self_killers: &mut Vec<SelfKillerInfo>,
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    fighter: &UFighter,
    effect_rid: REffectId,
) {
    let common = match get_common_info(ctx, calc, item_uid, fighter, effect_rid) {
        Some(common) => common,
        None => return,
    };
    if common.kills_item {
        process_effect_sk(cseq_map, self_killers, effect_rid, common.duration);
        return;
    }
    cseq_map.insert(
        effect_rid,
        CycleSeq::Inf(CSeqInf {
            data: CycleDataFull {
                duration: common.duration_with_cd,
                interrupt: CycleInterrupt::try_new(common.int_cd, false),
                chargedness: None,
            },
        }),
    );
}

fn fill_effect_cseq_sim_no_rearm(
    cseq_map: &mut CseqMap,
    self_killers: &mut Vec<SelfKillerInfo>,
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    fighter: &UFighter,
    effect_rid: REffectId,
) {
    let common = match get_common_info(ctx, calc, item_uid, fighter, effect_rid) {
        Some(common) => common,
        None => return,
    };
    if common.kills_item {
        process_effect_sk(cseq_map, self_killers, effect_rid, common.duration);
        return;
    }
    match common.charge_count {
        InfCount::Count(charge_count) => cseq_map.insert(
            effect_rid,
            CycleSeq::Lim(CSeqLim {
                data: CycleDataFull {
                    duration: common.duration_with_cd,
                    interrupt: CycleInterrupt::try_new(common.int_cd, false),
                    chargedness: None,
                },
                repeat_count: charge_count,
            }),
        ),
        InfCount::Infinite => cseq_map.insert(
            effect_rid,
            CycleSeq::Inf(CSeqInf {
                data: CycleDataFull {
                    duration: common.duration_with_cd,
                    interrupt: CycleInterrupt::try_new(common.int_cd, false),
                    chargedness: None,
                },
            }),
        ),
    };
}

fn process_effect_sk(
    cseq_map: &mut CseqMap,
    self_killers: &mut Vec<SelfKillerInfo>,
    effect_rid: REffectId,
    effect_duration: PValue,
) {
    self_killers.push(SelfKillerInfo {
        effect_rid,
        duration: effect_duration,
    });
    cseq_map.insert(
        effect_rid,
        CycleSeq::Lim(CSeqLim {
            data: CycleDataFull {
                duration: effect_duration,
                interrupt: None,
                chargedness: None,
            },
            repeat_count: Count::ONE,
        }),
    );
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rearm is considered
////////////////////////////////////////////////////////////////////////////////////////////////////
struct EffectInfo {
    cseq: CycleSeq<CycleDataFull>,
    rearm: Option<EffectRearmInfo>,
}

#[derive(Copy, Clone)]
struct EffectRearmInfo {
    duration_until_rearm: PValue,
    full_rearm_duration: PValue,
    charge_rearm_duration: PValue,
}

fn fill_effect_cseq_sim_rearm(
    ext_cseq_map: &mut RMap<REffectId, EffectInfo>,
    self_killers: &mut Vec<SelfKillerInfo>,
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    fighter: &UFighter,
    effect_rid: REffectId,
) {
    let common = match get_common_info(ctx, calc, item_uid, fighter, effect_rid) {
        Some(common) => common,
        None => return,
    };
    if common.kills_item {
        self_killers.push(SelfKillerInfo {
            effect_rid,
            duration: common.duration,
        });
        ext_cseq_map.insert(
            effect_rid,
            EffectInfo {
                cseq: CycleSeq::Lim(CSeqLim {
                    data: CycleDataFull {
                        duration: common.duration,
                        interrupt: None,
                        chargedness: None,
                    },
                    repeat_count: Count::ONE,
                }),
                rearm: None,
            },
        );
        return;
    }
    match common.charge_count {
        // Recalling and releasing fighter resets current effect cycles and cooldowns. Here, for
        // deciding when to recall, we let effect cycle to complete (since some effects are not
        // applied instantly, e.g. micro bomb from LR fighters disappears when fighter is recalled),
        // but ignore ongoing cooldowns.
        InfCount::Count(charge_count) => {
            let full_rearm_duration = common.charge_reload_duration * charge_count.into_pvalue();
            match charge_count {
                Count::ONE => {
                    ext_cseq_map.insert(
                        effect_rid,
                        EffectInfo {
                            cseq: CycleSeq::Inf(CSeqInf {
                                data: CycleDataFull {
                                    duration: common.duration,
                                    interrupt: CycleInterrupt::try_new(false, true),
                                    chargedness: None,
                                },
                            }),
                            rearm: Some(EffectRearmInfo {
                                duration_until_rearm: common.duration,
                                full_rearm_duration,
                                charge_rearm_duration: common.charge_reload_duration,
                            }),
                        },
                    );
                }
                charge_count => {
                    let p1_repeat_count = charge_count - Count::ONE;
                    ext_cseq_map.insert(
                        effect_rid,
                        EffectInfo {
                            cseq: CycleSeq::LoopLimSin(CSeqLoopLimSin {
                                p1_data: CycleDataFull {
                                    duration: common.duration_with_cd,
                                    interrupt: CycleInterrupt::try_new(common.int_cd, false),
                                    chargedness: None,
                                },
                                p1_repeat_count,
                                p2_data: CycleDataFull {
                                    duration: common.duration,
                                    interrupt: CycleInterrupt::try_new(false, true),
                                    chargedness: None,
                                },
                            }),
                            rearm: Some(EffectRearmInfo {
                                duration_until_rearm: common.duration_with_cd * p1_repeat_count.into_pvalue()
                                    + common.duration,
                                full_rearm_duration,
                                charge_rearm_duration: common.charge_reload_duration,
                            }),
                        },
                    );
                }
            }
        }
        InfCount::Infinite => {
            ext_cseq_map.insert(
                effect_rid,
                EffectInfo {
                    cseq: CycleSeq::Inf(CSeqInf {
                        data: CycleDataFull {
                            duration: common.duration_with_cd,
                            interrupt: CycleInterrupt::try_new(common.int_cd, false),
                            chargedness: None,
                        },
                    }),
                    rearm: None,
                },
            );
        }
    };
}

fn process_refuel(cseq_map: &mut CseqMap, mut ext_cseq_map: RMap<REffectId, EffectInfo>) {
    cseq_map.reserve(ext_cseq_map.len());
    // Get effect which runs out of its charges fastest
    let (effect_rid, cseq, rearm) = match ext_cseq_map
        .iter()
        .filter_map(|(effect_rid, effect_info)| match effect_info.rearm {
            Some(rearm_info) => Some((*effect_rid, effect_info.cseq, rearm_info)),
            None => None,
        })
        .min_by_key(|(_, _, rearm)| rearm.duration_until_rearm)
    {
        Some((effect_rid, cseq, rearm)) => {
            // Remove it from source map, since we extracted the data we needed anyway
            ext_cseq_map.remove(&effect_rid);
            (effect_rid, cseq, rearm)
        }
        None => {
            // No rearm data means all effects can cycle infinitely, just return everything we
            // received in this case
            for (effect_key, effect_info) in ext_cseq_map.into_iter() {
                cseq_map.insert(effect_key, effect_info.cseq);
            }
            return;
        }
    };
    // Time it takes to rearm just abilities
    // let mut max_rearm_time_s = rearm.full_rearm_time_s;
    // cycle_infos
}
