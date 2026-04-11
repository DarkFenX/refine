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
    match options {
        CyclingOptions::Burst => {
            let mut self_killers = Vec::new();
            for effect_rid in effect_rids {
                burst_fill_effect_cseq(
                    reuse_cseq_map,
                    &mut self_killers,
                    ctx,
                    calc,
                    item_uid,
                    fighter,
                    effect_rid,
                )
            }
            if !self_killers.is_empty() {
                process_fighter_sk(reuse_cseq_map, self_killers);
            }
        }
        CyclingOptions::Sim(sim_options) => {
            let rearm_minions = ctx.u_data.get_item_rearm_minion(item_uid, sim_options.rearm_minions);
            match rearm_minions {
                RearmMinion::Disabled => {
                    let mut self_killers = Vec::new();
                    for effect_rid in effect_rids {
                        sim_no_rearm_fill_effect_cseq(
                            reuse_cseq_map,
                            &mut self_killers,
                            ctx,
                            calc,
                            item_uid,
                            fighter,
                            effect_rid,
                        )
                    }
                    if !self_killers.is_empty() {
                        process_fighter_sk(reuse_cseq_map, self_killers);
                    }
                }
                RearmMinion::OnFirstEmpty => {
                    let info_map = rearm_collect_infos(ctx, calc, item_uid, fighter, effect_rids);
                    if rearm_process_sks(reuse_cseq_map, &info_map) {
                        return true;
                    }
                    rearm_process_refuel(reuse_cseq_map, info_map, fighter);
                }
            }
        }
    }
    true
}

#[derive(Copy, Clone)]
struct EffectInfo {
    kills_item: bool,
    cycle_duration: PValue,
    cycle_duration_with_cd: PValue,
    int_cd: bool,
    charge_count: InfCount,
    charge_rearm_duration: PValue,
}

fn get_effect_info(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    fighter: &UFighter,
    effect_rid: REffectId,
) -> Option<EffectInfo> {
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
    Some(EffectInfo {
        kills_item: effect.kills_item,
        cycle_duration: duration,
        cycle_duration_with_cd: duration_with_cd,
        int_cd,
        charge_count,
        charge_rearm_duration: effect_data.charge_reload_duration,
    })
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// No rearm considered
////////////////////////////////////////////////////////////////////////////////////////////////////
fn burst_fill_effect_cseq(
    cseq_map: &mut CseqMap,
    self_killers: &mut Vec<SelfKillerInfo>,
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    fighter: &UFighter,
    effect_rid: REffectId,
) {
    let info = match get_effect_info(ctx, calc, item_uid, fighter, effect_rid) {
        Some(info) => info,
        None => return,
    };
    if info.kills_item {
        process_effect_sk(cseq_map, self_killers, effect_rid, info.cycle_duration);
        return;
    }
    cseq_map.insert(effect_rid, burst_info_to_cseq(info));
}
fn burst_info_to_cseq(info: EffectInfo) -> CycleSeq<CycleDataFull> {
    CycleSeq::Inf(CSeqInf {
        data: CycleDataFull {
            duration: info.cycle_duration_with_cd,
            interrupt: CycleInterrupt::try_new(info.int_cd, false),
            chargedness: None,
        },
    })
}

fn sim_no_rearm_fill_effect_cseq(
    cseq_map: &mut CseqMap,
    self_killers: &mut Vec<SelfKillerInfo>,
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    fighter: &UFighter,
    effect_rid: REffectId,
) {
    let info = match get_effect_info(ctx, calc, item_uid, fighter, effect_rid) {
        Some(info) => info,
        None => return,
    };
    if info.kills_item {
        process_effect_sk(cseq_map, self_killers, effect_rid, info.cycle_duration);
        return;
    }
    cseq_map.insert(effect_rid, sim_no_rearm_info_to_cseq(info));
}
fn sim_no_rearm_info_to_cseq(info: EffectInfo) -> CycleSeq<CycleDataFull> {
    match info.charge_count {
        InfCount::Count(charge_count) => CycleSeq::Lim(CSeqLim {
            data: CycleDataFull {
                duration: info.cycle_duration_with_cd,
                interrupt: CycleInterrupt::try_new(info.int_cd, false),
                chargedness: None,
            },
            repeat_count: charge_count,
        }),
        InfCount::Infinite => CycleSeq::Inf(CSeqInf {
            data: CycleDataFull {
                duration: info.cycle_duration_with_cd,
                interrupt: CycleInterrupt::try_new(info.int_cd, false),
                chargedness: None,
            },
        }),
    }
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

fn process_fighter_sk(cseq_map: &mut CseqMap, self_killers: Vec<SelfKillerInfo>) {
    // If there are any self-killer effects, choose the fastest one, and discard all other effects
    let fastest_sk_effect_rid = self_killers
        .into_iter()
        .min_by_key(|sk_info| sk_info.duration)
        .unwrap()
        .effect_rid;
    cseq_map.retain(|&k, _| k == fastest_sk_effect_rid);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rearm is considered
////////////////////////////////////////////////////////////////////////////////////////////////////
fn rearm_collect_infos(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    fighter: &UFighter,
    effect_rids: impl Iterator<Item = REffectId>,
) -> RMap<REffectId, EffectInfo> {
    let mut info_map = RMap::new();
    for effect_rid in effect_rids {
        match get_effect_info(ctx, calc, item_uid, fighter, effect_rid) {
            Some(info) => info_map.insert(effect_rid, info),
            None => continue,
        };
    }
    info_map
}

fn rearm_process_sks(cseq_map: &mut CseqMap, info_map: &RMap<REffectId, EffectInfo>) -> bool {
    match info_map
        .iter()
        .filter_map(|(effect_rid, info)| match info.kills_item {
            true => Some((*effect_rid, info.cycle_duration)),
            false => None,
        })
        .min_by_key(|(_, cycle_duration)| *cycle_duration)
    {
        Some((fastest_sk_effect_rid, fastest_sk_cycle_duration)) => {
            cseq_map.insert(
                fastest_sk_effect_rid,
                CycleSeq::Lim(CSeqLim {
                    data: CycleDataFull {
                        duration: fastest_sk_cycle_duration,
                        interrupt: None,
                        chargedness: None,
                    },
                    repeat_count: Count::ONE,
                }),
            );
            true
        }
        None => false,
    }
}

fn rearm_process_refuel(cseq_map: &mut CseqMap, mut info_map: RMap<REffectId, EffectInfo>, fighter: &UFighter) {
    cseq_map.reserve(info_map.len());
    // Get effect which runs out of its charges fastest
    let (trigger_effect_rid, trigger_info, trigger_rearm) = match info_map
        .iter()
        .filter_map(|(effect_rid, info)| match RearmInfo::try_build(&info) {
            Some(trigger_rearm) => Some((*effect_rid, *info, trigger_rearm)),
            None => None,
        })
        .min_by_key(|(_, _, trigger_rearm)| trigger_rearm.in_space_duration)
    {
        Some((trigger_effect_rid, trigger_info, trigger_rearm)) => {
            // Remove it from source map, since we extracted the data we needed anyway
            info_map.remove(&trigger_effect_rid);
            (trigger_effect_rid, trigger_info, trigger_rearm)
        }
        None => {
            // When no effect needs fighter to be recalled for rearming, process it the no-rearm way
            for (effect_rid, info) in info_map.into_iter() {
                cseq_map.insert(effect_rid, sim_no_rearm_info_to_cseq(info));
            }
            return;
        }
    };
    // Here it is assumed that ability which triggers reload is the one which takes the longest time
    // to rearm its charges. On top of that, fighters take extra second to land, some time to
    // refuel, and extra second to launch.
    let in_space_duration = trigger_rearm.in_space_duration;
    let refuel_duration = fighter.get_axt().unwrap().fighter_refuel_duration;
    let downtime_duration = PValue::from_f64_unchecked(2.0) + refuel_duration + trigger_rearm.rearm_duration;
    // Fill data for triggering effect
    cseq_map.insert(
        trigger_effect_rid,
        rearm_trigger_info_to_cseq(trigger_info, trigger_rearm, downtime_duration),
    );
    // Fill data for the rest of effects
    for (effect_rid, info) in info_map.into_iter() {
        if let Some(cseq) = rearm_other_info_to_cseq(info, in_space_duration, downtime_duration) {
            cseq_map.insert(effect_rid, cseq);
        }
    }
}
fn rearm_trigger_info_to_cseq(
    info: EffectInfo,
    rearm: RearmInfo,
    downtime_duration: PValue,
) -> CycleSeq<CycleDataFull> {
    match rearm.charge_count {
        Count::ZERO => unreachable!("trigger effect should always have at least 1 charge"),
        Count::ONE => CycleSeq::Inf(CSeqInf {
            data: CycleDataFull {
                duration: info.cycle_duration + downtime_duration,
                interrupt: CycleInterrupt::try_new(info.int_cd, true),
                chargedness: None,
            },
        }),
        charge_count => {
            let p1_repeat_count = charge_count - Count::ONE;
            CycleSeq::LoopLimSin(CSeqLoopLimSin {
                p1_data: CycleDataFull {
                    duration: info.cycle_duration_with_cd,
                    interrupt: CycleInterrupt::try_new(info.int_cd, false),
                    chargedness: None,
                },
                p1_repeat_count,
                p2_data: CycleDataFull {
                    duration: info.cycle_duration + downtime_duration,
                    interrupt: CycleInterrupt::try_new(info.int_cd, true),
                    chargedness: None,
                },
            })
        }
    }
}
fn rearm_other_info_to_cseq(
    info: EffectInfo,
    in_space_duration: PValue,
    downtime_duration: PValue,
) -> Option<CycleSeq<CycleDataFull>> {
    let mut cycle_count = Count::from_pvalue_trunced(in_space_duration / info.cycle_duration_with_cd);
    if in_space_duration % info.cycle_duration_with_cd >= info.cycle_duration {
        cycle_count += Count::ONE;
    }
    match cycle_count {
        Count::ZERO => None,
        Count::ONE => Some(CycleSeq::Inf(CSeqInf {
            data: CycleDataFull {
                duration: in_space_duration + downtime_duration,
                interrupt: CycleInterrupt::try_new(info.int_cd, true),
                chargedness: None,
            },
        })),
        cycle_count => {
            let p1_repeat_count = cycle_count - Count::ONE;
            let p1_duration = info.cycle_duration_with_cd;
            let p2_duration =
                PValue::from_value_clamped(in_space_duration - p1_duration * p1_repeat_count.into_pvalue())
                    + downtime_duration;
            Some(CycleSeq::LoopLimSin(CSeqLoopLimSin {
                p1_data: CycleDataFull {
                    duration: p1_duration,
                    interrupt: CycleInterrupt::try_new(info.int_cd, false),
                    chargedness: None,
                },
                p1_repeat_count,
                p2_data: CycleDataFull {
                    duration: p2_duration,
                    interrupt: CycleInterrupt::try_new(info.int_cd, true),
                    chargedness: None,
                },
            }))
        }
    }
}

struct RearmInfo {
    charge_count: Count,
    in_space_duration: PValue,
    rearm_duration: PValue,
}
impl RearmInfo {
    fn try_build(info: &EffectInfo) -> Option<Self> {
        match info.charge_count {
            // Send fighter into rearm as soon as effect cycle is completed, do not wait for cooldowns
            InfCount::Count(charge_count) => match charge_count {
                Count::ZERO => None,
                Count::ONE => Some(Self {
                    charge_count,
                    in_space_duration: info.cycle_duration,
                    rearm_duration: info.charge_rearm_duration,
                }),
                charge_count => Some(Self {
                    charge_count,
                    in_space_duration: info.cycle_duration_with_cd * (charge_count - Count::ONE).into_pvalue()
                        + info.cycle_duration,
                    rearm_duration: info.charge_rearm_duration * charge_count.into_pvalue(),
                }),
            },
            InfCount::Infinite => None,
        }
    }
}
