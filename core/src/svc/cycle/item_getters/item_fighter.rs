use either::Either;

use super::{
    map::CseqMap,
    shared::{CyclingOptions, SelfKillerEffectInfo, SelfKillerItemInfo},
};
use crate::{
    misc::RearmMinion,
    num::{Count, PValue, UnitInterval},
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
            let mut sk_item_info = SelfKillerItemInfo::new();
            for effect_rid in effect_rids {
                burst_fill_effect_cseq(
                    reuse_cseq_map,
                    &mut sk_item_info,
                    ctx,
                    calc,
                    item_uid,
                    fighter,
                    effect_rid,
                )
            }
            // If there are any self-killer effects, the fastest one is used, and other effects are
            // discarded
            if let Some(sk_effect_rid) = sk_item_info.get_effect_rid() {
                reuse_cseq_map.retain(|&k, _| k == sk_effect_rid);
            }
        }
        CyclingOptions::Sim(sim_options) => {
            let rearm_minions = ctx.u_data.get_item_rearm_minion(item_uid, sim_options.rearm_minions);
            match rearm_minions {
                RearmMinion::Disabled => {
                    let mut sk_item_info = SelfKillerItemInfo::new();
                    for effect_rid in effect_rids {
                        sim_no_rearm_fill_effect_cseq(
                            reuse_cseq_map,
                            &mut sk_item_info,
                            ctx,
                            calc,
                            item_uid,
                            fighter,
                            effect_rid,
                        )
                    }
                    // If there are any self-killer effects, the fastest one is used, and other effects are
                    // discarded
                    if let Some(sk_effect_rid) = sk_item_info.get_effect_rid() {
                        reuse_cseq_map.retain(|&k, _| k == sk_effect_rid);
                    }
                }
                RearmMinion::OnFirstEmpty => {
                    let effect_infos = rearm_collect_effect_infos(ctx, calc, item_uid, fighter, effect_rids);
                    if rearm_process_sks(reuse_cseq_map, &effect_infos) {
                        return true;
                    }
                    rearm_process_refuel(reuse_cseq_map, effect_infos, fighter);
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
    charge_count: Option<Count>,
    charge_rearm_duration: PValue,
}
impl EffectInfo {
    fn to_chargedness(&self) -> Option<UnitInterval> {
        match &self.charge_count {
            Some(_) => Some(UnitInterval::ONE),
            None => None,
        }
    }
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
    // Completely skip effects which can't cycle
    if effect_data.charge_count == Some(Count::ZERO) {
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
        charge_count: effect_data.charge_count,
        charge_rearm_duration: effect_data.charge_reload_duration,
    })
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// No rearm considered
////////////////////////////////////////////////////////////////////////////////////////////////////
fn burst_fill_effect_cseq(
    cseq_map: &mut CseqMap,
    sk_item_info: &mut SelfKillerItemInfo,
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    fighter: &UFighter,
    effect_rid: REffectId,
) {
    let effect_info = match get_effect_info(ctx, calc, item_uid, fighter, effect_rid) {
        Some(effect_info) => effect_info,
        None => return,
    };
    if effect_info.kills_item {
        process_effect_sk(cseq_map, sk_item_info, effect_rid, effect_info);
        return;
    }
    cseq_map.insert(effect_rid, burst_info_to_cseq(effect_info));
}
fn burst_info_to_cseq(effect_info: EffectInfo) -> CycleSeq<CycleDataFull> {
    CycleSeq::Inf(CSeqInf {
        data: CycleDataFull {
            duration: effect_info.cycle_duration_with_cd,
            interrupt: CycleInterrupt::try_new(effect_info.int_cd, false),
            chargedness: effect_info.to_chargedness(),
        },
    })
}

fn sim_no_rearm_fill_effect_cseq(
    cseq_map: &mut CseqMap,
    sk_item_info: &mut SelfKillerItemInfo,
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    fighter: &UFighter,
    effect_rid: REffectId,
) {
    let effect_info = match get_effect_info(ctx, calc, item_uid, fighter, effect_rid) {
        Some(effect_info) => effect_info,
        None => return,
    };
    if effect_info.kills_item {
        process_effect_sk(cseq_map, sk_item_info, effect_rid, effect_info);
        return;
    }
    cseq_map.insert(effect_rid, sim_no_rearm_info_to_cseq(effect_info));
}
fn sim_no_rearm_info_to_cseq(effect_info: EffectInfo) -> CycleSeq<CycleDataFull> {
    match effect_info.charge_count {
        Some(charge_count) => CycleSeq::Lim(CSeqLim {
            data: CycleDataFull {
                duration: effect_info.cycle_duration_with_cd,
                interrupt: CycleInterrupt::try_new(effect_info.int_cd, false),
                chargedness: effect_info.to_chargedness(),
            },
            repeat_count: charge_count,
        }),
        None => CycleSeq::Inf(CSeqInf {
            data: CycleDataFull {
                duration: effect_info.cycle_duration_with_cd,
                interrupt: CycleInterrupt::try_new(effect_info.int_cd, false),
                chargedness: effect_info.to_chargedness(),
            },
        }),
    }
}

fn process_effect_sk(
    cseq_map: &mut CseqMap,
    sk_item_info: &mut SelfKillerItemInfo,
    effect_rid: REffectId,
    effect_info: EffectInfo,
) {
    sk_item_info.push(SelfKillerEffectInfo {
        effect_rid,
        duration: effect_info.cycle_duration,
    });
    cseq_map.insert(
        effect_rid,
        CycleSeq::Lim(CSeqLim {
            data: CycleDataFull {
                duration: effect_info.cycle_duration,
                interrupt: None,
                chargedness: effect_info.to_chargedness(),
            },
            repeat_count: Count::ONE,
        }),
    );
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rearm is considered
////////////////////////////////////////////////////////////////////////////////////////////////////
fn rearm_collect_effect_infos(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    fighter: &UFighter,
    effect_rids: impl Iterator<Item = REffectId>,
) -> RMap<REffectId, EffectInfo> {
    let mut effect_infos = RMap::new();
    for effect_rid in effect_rids {
        match get_effect_info(ctx, calc, item_uid, fighter, effect_rid) {
            Some(effect_info) => effect_infos.insert(effect_rid, effect_info),
            None => continue,
        };
    }
    effect_infos
}

fn rearm_process_sks(cseq_map: &mut CseqMap, effect_infos: &RMap<REffectId, EffectInfo>) -> bool {
    match effect_infos
        .iter()
        .filter_map(|(effect_rid, effect_info)| match effect_info.kills_item {
            true => Some((*effect_rid, *effect_info)),
            false => None,
        })
        .min_by_key(|(_, effect_info)| effect_info.cycle_duration)
    {
        Some((fastest_sk_effect_rid, fastest_sk_info)) => {
            cseq_map.insert(
                fastest_sk_effect_rid,
                CycleSeq::Lim(CSeqLim {
                    data: CycleDataFull {
                        duration: fastest_sk_info.cycle_duration,
                        interrupt: None,
                        chargedness: fastest_sk_info.to_chargedness(),
                    },
                    repeat_count: Count::ONE,
                }),
            );
            true
        }
        None => false,
    }
}

fn rearm_process_refuel(cseq_map: &mut CseqMap, mut effect_infos: RMap<REffectId, EffectInfo>, fighter: &UFighter) {
    cseq_map.reserve(effect_infos.len());
    // Get effect which runs out of its charges fastest
    let (trigger_effect_rid, trigger_effect_info, trigger_rearm_info) = match effect_infos
        .iter()
        .filter_map(|(effect_rid, effect_info)| match RearmInfo::try_build(&effect_info) {
            Some(rearm_info) => Some((*effect_rid, *effect_info, rearm_info)),
            None => None,
        })
        .min_by_key(|(_, _, rearm_info)| rearm_info.in_space_duration)
    {
        Some((trigger_effect_rid, trigger_effect_info, trigger_rearm_info)) => {
            // Remove it from source map, since we extracted the data we needed anyway
            effect_infos.remove(&trigger_effect_rid);
            (trigger_effect_rid, trigger_effect_info, trigger_rearm_info)
        }
        None => {
            // When no effect needs fighter to be recalled for rearming, process it the no-rearm way
            for (effect_rid, effect_info) in effect_infos.into_iter() {
                cseq_map.insert(effect_rid, sim_no_rearm_info_to_cseq(effect_info));
            }
            return;
        }
    };
    // Here it is assumed that ability which triggers reload is the one which takes the longest time
    // to rearm its charges. On top of that, fighters take extra second to land, some time to
    // refuel, and extra second to launch.
    let in_space_duration = trigger_rearm_info.in_space_duration;
    let refuel_duration = fighter.get_axt().unwrap().fighter_refuel_duration;
    let downtime_duration = PValue::from_f64_unchecked(2.0) + refuel_duration + trigger_rearm_info.rearm_duration;
    // Fill data for triggering effect
    cseq_map.insert(
        trigger_effect_rid,
        rearm_trigger_info_to_cseq(trigger_effect_info, trigger_rearm_info, downtime_duration),
    );
    // Fill data for the rest of effects
    for (effect_rid, effect_info) in effect_infos.into_iter() {
        if let Some(cseq) = rearm_other_info_to_cseq(effect_info, in_space_duration, downtime_duration) {
            cseq_map.insert(effect_rid, cseq);
        }
    }
}
fn rearm_trigger_info_to_cseq(
    effect_info: EffectInfo,
    rearm: RearmInfo,
    downtime_duration: PValue,
) -> CycleSeq<CycleDataFull> {
    match rearm.charge_count {
        Count::ZERO => unreachable!("trigger effect should always have at least 1 charge"),
        Count::ONE => CycleSeq::Inf(CSeqInf {
            data: CycleDataFull {
                duration: effect_info.cycle_duration + downtime_duration,
                interrupt: CycleInterrupt::try_new(effect_info.int_cd, true),
                chargedness: effect_info.to_chargedness(),
            },
        }),
        charge_count => {
            let p1_repeat_count = charge_count - Count::ONE;
            CycleSeq::LoopLimSin(CSeqLoopLimSin {
                p1_data: CycleDataFull {
                    duration: effect_info.cycle_duration_with_cd,
                    interrupt: CycleInterrupt::try_new(effect_info.int_cd, false),
                    chargedness: effect_info.to_chargedness(),
                },
                p1_repeat_count,
                p2_data: CycleDataFull {
                    duration: effect_info.cycle_duration + downtime_duration,
                    interrupt: CycleInterrupt::try_new(effect_info.int_cd, true),
                    chargedness: effect_info.to_chargedness(),
                },
            })
        }
    }
}
fn rearm_other_info_to_cseq(
    effect_info: EffectInfo,
    in_space_duration: PValue,
    downtime_duration: PValue,
) -> Option<CycleSeq<CycleDataFull>> {
    let mut cycle_count = Count::from_pvalue_trunced(in_space_duration / effect_info.cycle_duration_with_cd);
    if in_space_duration % effect_info.cycle_duration_with_cd >= effect_info.cycle_duration {
        cycle_count += Count::ONE;
    }
    match cycle_count {
        Count::ZERO => None,
        Count::ONE => Some(CycleSeq::Inf(CSeqInf {
            data: CycleDataFull {
                duration: in_space_duration + downtime_duration,
                interrupt: CycleInterrupt::try_new(effect_info.int_cd, true),
                chargedness: effect_info.to_chargedness(),
            },
        })),
        cycle_count => {
            let p1_repeat_count = cycle_count - Count::ONE;
            let p1_duration = effect_info.cycle_duration_with_cd;
            let p2_duration =
                PValue::from_value_clamped(in_space_duration - p1_duration * p1_repeat_count.into_pvalue())
                    + downtime_duration;
            Some(CycleSeq::LoopLimSin(CSeqLoopLimSin {
                p1_data: CycleDataFull {
                    duration: p1_duration,
                    interrupt: CycleInterrupt::try_new(effect_info.int_cd, false),
                    chargedness: effect_info.to_chargedness(),
                },
                p1_repeat_count,
                p2_data: CycleDataFull {
                    duration: p2_duration,
                    interrupt: CycleInterrupt::try_new(effect_info.int_cd, true),
                    chargedness: effect_info.to_chargedness(),
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
    fn try_build(effect_info: &EffectInfo) -> Option<Self> {
        let charge_count = effect_info.charge_count?;
        // Send fighter into rearm as soon as final effect cycle is completed, do not wait for
        // cooldowns
        match charge_count {
            Count::ZERO => None,
            Count::ONE => Some(Self {
                charge_count,
                in_space_duration: effect_info.cycle_duration,
                rearm_duration: effect_info.charge_rearm_duration,
            }),
            charge_count => Some(Self {
                charge_count,
                in_space_duration: effect_info.cycle_duration_with_cd * (charge_count - Count::ONE).into_pvalue()
                    + effect_info.cycle_duration,
                rearm_duration: effect_info.charge_rearm_duration * charge_count.into_pvalue(),
            }),
        }
    }
}
