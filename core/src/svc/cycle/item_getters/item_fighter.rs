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
        cycle::{
            CSeqHardDtFull, CSeqInf, CSeqLim, CSeqLoopLimSin, CycleActive, CycleDataFull, CycleSeq, CycleSoftDtFull,
        },
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
) -> bool {
    if !fighter.is_loaded() {
        return false;
    };
    let effect_rids = fighter.get_reffs().unwrap().iter().copied();
    reuse_cseq_map.clear();
    match options {
        CyclingOptions::Burst => burst_fill_cseqs(ctx, calc, item_uid, fighter, effect_rids, reuse_cseq_map),
        CyclingOptions::Sim(sim_options) => {
            let rearm_minions = ctx.u_data.get_item_rearm_minion(item_uid, sim_options.rearm_minions);
            match rearm_minions {
                RearmMinion::Disabled => {
                    sim_no_rearm_fill_cseqs(ctx, calc, item_uid, fighter, effect_rids, reuse_cseq_map)
                }
                RearmMinion::OnFirstEmpty => {
                    sim_rearm_fill_cseqs(ctx, calc, item_uid, fighter, effect_rids, reuse_cseq_map)
                }
            }
        }
    }
    true
}

#[derive(Copy, Clone)]
struct EffectInfo {
    kills_item: bool,
    active_duration: PValue,
    // Counting from end of active duration
    cooldown_duration: PValue,
    soft_dt_cd: bool,
    charge_count: Option<Count>,
    charge_rearm_duration: PValue,
}
impl EffectInfo {
    fn get_chargedness(&self) -> Option<UnitInterval> {
        self.charge_count.map(|_| UnitInterval::ONE)
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
    let active_duration = funcs::get_effect_duration_s(ctx, calc, item_uid, effect)?;
    let effect_data = fighter.get_effects().unwrap().get(&effect_rid).unwrap();
    // Completely skip effects which can't cycle
    if effect_data.charge_count == Some(Count::ZERO) {
        return None;
    }
    // For fighter abilities, cooldown starts as soon as effect starts cycling. It typically is
    // longer than duration, but data format does not guarantee that
    let cooldown_duration = PValue::from_value_clamped(effect_data.cooldown_s - active_duration);
    // Assume any cooldown interrupts cycling, even if it shorter than ability cycle
    let soft_dt_cd = effect_data.cooldown_s > PValue::ZERO;
    Some(EffectInfo {
        kills_item: effect.kills_item,
        active_duration,
        cooldown_duration,
        soft_dt_cd,
        charge_count: effect_data.charge_count,
        charge_rearm_duration: effect_data.charge_reload_duration,
    })
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// No rearm considered
////////////////////////////////////////////////////////////////////////////////////////////////////
fn burst_fill_cseqs(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    fighter: &UFighter,
    effect_rids: impl ExactSizeIterator<Item = REffectId>,
    cseq_map: &mut CseqMap,
) {
    let mut sk_item_info = SelfKillerItemInfo::new();
    for effect_rid in effect_rids {
        burst_fill_effect_cseq(cseq_map, &mut sk_item_info, ctx, calc, item_uid, fighter, effect_rid)
    }
    // If there are any self-killer effects, the fastest one is used, and other effects are
    // discarded
    if let Some(sk_effect_rid) = sk_item_info.get_effect_rid() {
        cseq_map.retain(|&k, _| k == sk_effect_rid);
    }
}
fn burst_fill_effect_cseq(
    cseq_map: &mut CseqMap,
    sk_item_info: &mut SelfKillerItemInfo,
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    fighter: &UFighter,
    effect_rid: REffectId,
) {
    let Some(effect_info) = get_effect_info(ctx, calc, item_uid, fighter, effect_rid) else {
        return;
    };
    if effect_info.kills_item {
        fill_sk_effect_data(cseq_map, sk_item_info, effect_rid, effect_info);
        return;
    }
    cseq_map.insert(effect_rid, burst_info_to_cseq(effect_info));
}
fn burst_info_to_cseq(effect_info: EffectInfo) -> CycleSeq<CycleDataFull, CSeqHardDtFull> {
    CycleSeq::Inf(CSeqInf {
        data: CycleDataFull {
            active: CycleActive {
                duration: effect_info.active_duration,
                chargedness: effect_info.get_chargedness(),
            },
            soft_dt: CycleSoftDtFull::try_new(effect_info.cooldown_duration, effect_info.soft_dt_cd, false, false),
        },
        hard_dt: None,
    })
}

fn sim_no_rearm_fill_cseqs(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    fighter: &UFighter,
    effect_rids: impl ExactSizeIterator<Item = REffectId>,
    cseq_map: &mut CseqMap,
) {
    let mut sk_item_info = SelfKillerItemInfo::new();
    for effect_rid in effect_rids {
        sim_no_rearm_fill_effect_cseq(cseq_map, &mut sk_item_info, ctx, calc, item_uid, fighter, effect_rid)
    }
    // If there are any self-killer effects, the fastest one is used, and other effects are
    // discarded
    if let Some(sk_effect_rid) = sk_item_info.get_effect_rid() {
        cseq_map.retain(|&k, _| k == sk_effect_rid);
    }
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
    let Some(effect_info) = get_effect_info(ctx, calc, item_uid, fighter, effect_rid) else {
        return;
    };
    if effect_info.kills_item {
        fill_sk_effect_data(cseq_map, sk_item_info, effect_rid, effect_info);
        return;
    }
    cseq_map.insert(effect_rid, sim_no_rearm_info_to_cseq(effect_info));
}
fn sim_no_rearm_info_to_cseq(effect_info: EffectInfo) -> CycleSeq<CycleDataFull, CSeqHardDtFull> {
    let cycle_data = CycleDataFull {
        active: CycleActive {
            duration: effect_info.active_duration,
            chargedness: effect_info.get_chargedness(),
        },
        soft_dt: CycleSoftDtFull::try_new(effect_info.cooldown_duration, effect_info.soft_dt_cd, false, false),
    };
    match effect_info.charge_count {
        Some(charge_count) => CycleSeq::Lim(CSeqLim {
            data: cycle_data,
            repeat_count: charge_count,
        }),
        None => CycleSeq::Inf(CSeqInf {
            data: cycle_data,
            hard_dt: None,
        }),
    }
}

fn fill_sk_effect_data(
    cseq_map: &mut CseqMap,
    sk_item_info: &mut SelfKillerItemInfo,
    effect_rid: REffectId,
    effect_info: EffectInfo,
) {
    sk_item_info.push(SelfKillerEffectInfo {
        effect_rid,
        active_duration: effect_info.active_duration,
    });
    cseq_map.insert(
        effect_rid,
        CycleSeq::Lim(CSeqLim {
            data: CycleDataFull {
                active: CycleActive {
                    duration: effect_info.active_duration,
                    chargedness: effect_info.get_chargedness(),
                },
                soft_dt: None,
            },
            repeat_count: Count::ONE,
        }),
    );
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rearm is considered
////////////////////////////////////////////////////////////////////////////////////////////////////
fn sim_rearm_fill_cseqs(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    fighter: &UFighter,
    effect_rids: impl ExactSizeIterator<Item = REffectId>,
    cseq_map: &mut CseqMap,
) {
    let effect_infos = sim_rearm_collect_effect_infos(ctx, calc, item_uid, fighter, effect_rids);
    if sim_rearm_process_sks(cseq_map, &effect_infos) {
        return;
    }
    sim_rearm_process_refuel(cseq_map, effect_infos, fighter);
}
fn sim_rearm_collect_effect_infos(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    fighter: &UFighter,
    effect_rids: impl ExactSizeIterator<Item = REffectId>,
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
fn sim_rearm_process_sks(cseq_map: &mut CseqMap, effect_infos: &RMap<REffectId, EffectInfo>) -> bool {
    match effect_infos
        .iter()
        .filter(|(_, effect_info)| effect_info.kills_item)
        .min_by_key(|(_, effect_info)| effect_info.active_duration)
    {
        Some((&fastest_sk_effect_rid, fastest_sk_info)) => {
            cseq_map.insert(
                fastest_sk_effect_rid,
                CycleSeq::Lim(CSeqLim {
                    data: CycleDataFull {
                        active: CycleActive {
                            duration: fastest_sk_info.active_duration,
                            chargedness: fastest_sk_info.get_chargedness(),
                        },
                        soft_dt: None,
                    },
                    repeat_count: Count::ONE,
                }),
            );
            true
        }
        None => false,
    }
}
fn sim_rearm_process_refuel(cseq_map: &mut CseqMap, mut effect_infos: RMap<REffectId, EffectInfo>, fighter: &UFighter) {
    cseq_map.reserve(effect_infos.len());
    // Get effect which runs out of its charges first
    let (trigger_effect_rid, trigger_effect_info, trigger_rearm_info) = match effect_infos
        .iter()
        .filter_map(|(effect_rid, effect_info)| {
            RearmInfo::try_build(effect_info).map(|rearm_info| (effect_rid, effect_info, rearm_info))
        })
        .min_by_key(|(_, _, rearm_info)| rearm_info.in_space_duration)
    {
        Some((&trigger_effect_rid, &trigger_effect_info, trigger_rearm_info)) => {
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
    let hard_dt_duration = PValue::from_f64_unchecked(2.0) + refuel_duration + trigger_rearm_info.rearm_duration;
    // Fill data for triggering effect
    cseq_map.insert(
        trigger_effect_rid,
        sim_rearm_trigger_info_to_cseq(trigger_effect_info, trigger_rearm_info, hard_dt_duration),
    );
    // Fill data for the rest of effects
    for (effect_rid, effect_info) in effect_infos.into_iter() {
        if let Some(cseq) = sim_rearm_other_info_to_cseq(effect_info, in_space_duration, hard_dt_duration) {
            cseq_map.insert(effect_rid, cseq);
        }
    }
}
fn sim_rearm_trigger_info_to_cseq(
    effect_info: EffectInfo,
    rearm_info: RearmInfo,
    hard_dt_duration: PValue,
) -> CycleSeq<CycleDataFull, CSeqHardDtFull> {
    match rearm_info.charge_count {
        Count::ZERO => unreachable!("0-charged effects are not processed"),
        Count::ONE => CycleSeq::Inf(CSeqInf {
            data: CycleDataFull {
                active: CycleActive {
                    duration: effect_info.active_duration,
                    chargedness: effect_info.get_chargedness(),
                },
                soft_dt: None,
            },
            hard_dt: make_hard_dt(hard_dt_duration),
        }),
        charge_count => {
            let p1_repeat_count = charge_count - Count::ONE;
            CycleSeq::LoopLimSin(CSeqLoopLimSin {
                p1_data: CycleDataFull {
                    active: CycleActive {
                        duration: effect_info.active_duration,
                        chargedness: effect_info.get_chargedness(),
                    },
                    soft_dt: CycleSoftDtFull::try_new(
                        effect_info.cooldown_duration,
                        effect_info.soft_dt_cd,
                        false,
                        false,
                    ),
                },
                p1_repeat_count,
                p2_data: CycleDataFull {
                    active: CycleActive {
                        duration: effect_info.active_duration,
                        chargedness: effect_info.get_chargedness(),
                    },
                    soft_dt: None,
                },
                hard_dt: make_hard_dt(hard_dt_duration),
            })
        }
    }
}
fn sim_rearm_other_info_to_cseq(
    effect_info: EffectInfo,
    in_space_duration: PValue,
    hard_dt_duration: PValue,
) -> Option<CycleSeq<CycleDataFull, CSeqHardDtFull>> {
    let active_and_cooldown_duration = effect_info.active_duration + effect_info.cooldown_duration;
    let full_cycle_count = Count::from_pvalue_trunced(in_space_duration / active_and_cooldown_duration);
    let in_space_duration_left = in_space_duration % active_and_cooldown_duration;
    let extra_cycle = if in_space_duration_left >= effect_info.active_duration {
        ExtraCycle::ActiveFull(PValue::from_value_unchecked(
            in_space_duration_left - effect_info.active_duration,
        ))
    } else if in_space_duration_left + PValue::FLOAT_TOLERANCE >= PValue::SERVER_TICK_S {
        ExtraCycle::ActivePartial(in_space_duration_left)
    } else {
        ExtraCycle::None(in_space_duration_left)
    };
    match (full_cycle_count, extra_cycle) {
        (Count::ZERO, ExtraCycle::None(_)) => None,
        (Count::ZERO, ExtraCycle::ActivePartial(active_duration)) => Some(CycleSeq::Inf(CSeqInf {
            data: make_extra_cycle_active_partial_data(effect_info, active_duration),
            hard_dt: make_hard_dt(hard_dt_duration),
        })),
        (Count::ZERO, ExtraCycle::ActiveFull(soft_dt_duration)) => Some(CycleSeq::Inf(CSeqInf {
            data: make_extra_cycle_active_full_data(effect_info, soft_dt_duration),
            hard_dt: make_hard_dt(hard_dt_duration),
        })),
        (Count::ONE, ExtraCycle::None(idle_duration)) => Some(CycleSeq::Inf(CSeqInf {
            data: make_full_cycle_with_extra_idling_data(effect_info, idle_duration),
            hard_dt: make_hard_dt(hard_dt_duration),
        })),
        (full_cycle_count, ExtraCycle::None(idle_duration)) => Some(CycleSeq::LoopLimSin(CSeqLoopLimSin {
            p1_data: make_full_cycle_data(effect_info),
            p1_repeat_count: full_cycle_count - Count::ONE,
            p2_data: make_full_cycle_with_extra_idling_data(effect_info, idle_duration),
            hard_dt: make_hard_dt(hard_dt_duration),
        })),
        (full_cycle_count, ExtraCycle::ActivePartial(active_duration)) => Some(CycleSeq::LoopLimSin(CSeqLoopLimSin {
            p1_data: make_full_cycle_data(effect_info),
            p1_repeat_count: full_cycle_count,
            p2_data: make_extra_cycle_active_partial_data(effect_info, active_duration),
            hard_dt: make_hard_dt(hard_dt_duration),
        })),
        (full_cycle_count, ExtraCycle::ActiveFull(soft_dt_duration)) => Some(CycleSeq::LoopLimSin(CSeqLoopLimSin {
            p1_data: make_full_cycle_data(effect_info),
            p1_repeat_count: full_cycle_count,
            p2_data: make_extra_cycle_active_full_data(effect_info, soft_dt_duration),
            hard_dt: make_hard_dt(hard_dt_duration),
        })),
    }
}
fn make_hard_dt(duration: PValue) -> Option<CSeqHardDtFull> {
    CSeqHardDtFull::try_new(duration, true)
}
fn make_full_cycle_data(effect_info: EffectInfo) -> CycleDataFull {
    CycleDataFull {
        active: CycleActive {
            duration: effect_info.active_duration,
            chargedness: effect_info.get_chargedness(),
        },
        soft_dt: CycleSoftDtFull::try_new(effect_info.cooldown_duration, effect_info.soft_dt_cd, false, false),
    }
}
fn make_full_cycle_with_extra_idling_data(effect_info: EffectInfo, idle_duration: PValue) -> CycleDataFull {
    CycleDataFull {
        active: CycleActive {
            duration: effect_info.active_duration,
            chargedness: effect_info.get_chargedness(),
        },
        soft_dt: CycleSoftDtFull::try_new(
            effect_info.cooldown_duration + idle_duration,
            effect_info.soft_dt_cd,
            false,
            idle_duration > PValue::ZERO,
        ),
    }
}
fn make_extra_cycle_active_partial_data(effect_info: EffectInfo, active_duration: PValue) -> CycleDataFull {
    CycleDataFull {
        active: CycleActive {
            duration: active_duration,
            chargedness: effect_info.get_chargedness(),
        },
        // Since we are assuming that any cooldown interrupts cycling, follow that logic here and
        // make soft downtime if ability has any cooldown, even if its duration is 0
        soft_dt: CycleSoftDtFull::try_new(PValue::ZERO, effect_info.soft_dt_cd, false, false),
    }
}
fn make_extra_cycle_active_full_data(effect_info: EffectInfo, soft_dt_duration: PValue) -> CycleDataFull {
    CycleDataFull {
        active: CycleActive {
            duration: effect_info.active_duration,
            chargedness: effect_info.get_chargedness(),
        },
        soft_dt: CycleSoftDtFull::try_new(
            soft_dt_duration,
            effect_info.soft_dt_cd,
            false,
            soft_dt_duration > effect_info.cooldown_duration,
        ),
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
                in_space_duration: effect_info.active_duration,
                rearm_duration: effect_info.charge_rearm_duration,
            }),
            charge_count => Some(Self {
                charge_count,
                in_space_duration: (effect_info.active_duration + effect_info.cooldown_duration)
                    * (charge_count - Count::ONE).into_pvalue()
                    + effect_info.active_duration,
                rearm_duration: effect_info.charge_rearm_duration * charge_count.into_pvalue(),
            }),
        }
    }
}

enum ExtraCycle {
    // Full active part of cycle can fit in remaining time, value is time left after it (for
    // cooldown or idling)
    ActiveFull(PValue),
    // Time left only for partial cycle; value is the time left for extra cycle
    ActivePartial(PValue),
    // Not enough time for full or partial cycle, value is the time effect is idling
    None(PValue),
}
