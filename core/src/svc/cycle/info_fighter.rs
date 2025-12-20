use either::Either;

use super::{
    cycle::Cycle,
    cycle_inner_limited::CycleInnerLimited,
    cycle_old_reload1::CycleReload1,
    cycle_old_simple::CycleSimple,
    cycle_reload2::CycleReload2,
    info_shared::{CycleOptionReload, CycleOptions, SelfKillerInfo},
};
use crate::{
    def::{AttrVal, OF},
    rd::REffectKey,
    svc::{SvcCtx, calc::Calc, eff_funcs},
    ud::{UFighter, UItemKey},
    util::{InfCount, RMap},
};

struct FtrEffectInfo {
    cycle: Cycle,
    rearm: Option<FtrEffectRearmInfo>,
}

#[derive(Copy, Clone)]
struct FtrEffectRearmInfo {
    time_until_rearm_s: AttrVal,
    full_rearm_time_s: AttrVal,
    charge_rearm_time_s: AttrVal,
}

pub(super) fn get_fighter_cycle_info(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    fighter: &UFighter,
    options: CycleOptions,
    ignore_state: bool,
) -> Option<RMap<REffectKey, Cycle>> {
    if !fighter.is_loaded() {
        return None;
    };
    let mut effect_infos = RMap::new();
    let mut self_killers = Vec::new();
    let effect_keys = match ignore_state {
        true => Either::Left(fighter.get_effect_datas().unwrap().keys().copied()),
        false => Either::Right(fighter.get_reffs().unwrap().iter().copied()),
    };
    for effect_key in effect_keys {
        fill_fighter_effect_info(
            &mut effect_infos,
            &mut self_killers,
            ctx,
            calc,
            item_key,
            fighter,
            effect_key,
            options,
        );
    }
    // If there are any self-killer effects, choose the fastest one, and discard all other effects
    if !self_killers.is_empty() {
        let fastest_sk_effect_key = self_killers
            .into_iter()
            .min_by_key(|sk_info| sk_info.duration_s)
            .unwrap()
            .effect_key;
        let effect_cycle = effect_infos.get(&fastest_sk_effect_key).unwrap().cycle;
        let mut cycle_infos = RMap::new();
        cycle_infos.insert(fastest_sk_effect_key, effect_cycle);
        return Some(cycle_infos);
    }
    Some(process_refuel(effect_infos))
}

fn fill_fighter_effect_info(
    effect_infos: &mut RMap<REffectKey, FtrEffectInfo>,
    self_killers: &mut Vec<SelfKillerInfo>,
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    fighter: &UFighter,
    effect_key: REffectKey,
    options: CycleOptions,
) {
    let effect = ctx.u_data.src.get_effect(effect_key);
    if !effect.is_active_with_duration {
        return;
    }
    // No appropriate duration - no info
    let duration_s = match eff_funcs::get_effect_duration_s(ctx, calc, item_key, effect) {
        Some(duration_s) => duration_s,
        None => return,
    };
    let effect_data = fighter.get_effect_datas().unwrap().get(&effect_key).unwrap();
    let cycle_count = match effect_data.charge_count {
        Some(charge_count) => InfCount::Count(charge_count),
        None => InfCount::Infinite,
    };
    // Completely skip effects which can't cycle
    if cycle_count == InfCount::Count(0) {
        return;
    }
    // Self-killers are fairly trivial. Record info about them and go to next effect
    if effect.kills_item {
        self_killers.push(SelfKillerInfo { effect_key, duration_s });
        effect_infos.insert(
            effect_key,
            FtrEffectInfo {
                cycle: Cycle::Simple(CycleSimple {
                    active_time: duration_s,
                    inactive_time: OF(0.0),
                    repeat_count: InfCount::Count(1),
                }),
                rearm: None,
            },
        );
        return;
    }
    // Cooldown starts as soon as effect starts cycling
    let cd_inactivity_s = (effect_data.cooldown_s - duration_s).max(OF(0.0));
    let count_until_rearm = match cycle_count {
        InfCount::Count(count_until_rearm) => count_until_rearm,
        // No need for complex logic when effect is infinitely cycling
        InfCount::Infinite => {
            effect_infos.insert(
                effect_key,
                FtrEffectInfo {
                    cycle: Cycle::Simple(CycleSimple {
                        active_time: duration_s,
                        inactive_time: cd_inactivity_s,
                        repeat_count: InfCount::Infinite,
                    }),
                    rearm: None,
                },
            );
            return;
        }
    };
    match options.reload_mode {
        // Burst mode - assume fighter never docks to rearm, and keeps cycling its effects like it
        // has infinite charges
        CycleOptionReload::Burst => {
            effect_infos.insert(
                effect_key,
                FtrEffectInfo {
                    cycle: Cycle::Reload1(CycleReload1 {
                        inner: CycleInnerLimited {
                            active_time: duration_s,
                            inactive_time: cd_inactivity_s,
                            repeat_count: count_until_rearm,
                        },
                    }),
                    rearm: None,
                },
            );
        }
        // Recalling and releasing fighter resets current effect cycles and cooldowns. Here, for
        // deciding when to recall, we let effect cycle to complete (since some effects are not
        // applied instantly, e.g. micro bomb from LR fighters disappears when fighter is recalled),
        // but ignore ongoing cooldowns.
        CycleOptionReload::Sim => {
            let full_rearm_time_s = effect_data.charge_reload_time_s * count_until_rearm as f64;
            match count_until_rearm {
                // 1 charge - 1 simple cycle, go into rearm as soon as cycle completes
                1 => {
                    effect_infos.insert(
                        effect_key,
                        FtrEffectInfo {
                            cycle: Cycle::Reload1(CycleReload1 {
                                inner: CycleInnerLimited {
                                    active_time: duration_s,
                                    inactive_time: OF(0.0),
                                    repeat_count: count_until_rearm,
                                },
                            }),
                            rearm: Some(FtrEffectRearmInfo {
                                time_until_rearm_s: duration_s,
                                full_rearm_time_s,
                                charge_rearm_time_s: effect_data.charge_reload_time_s,
                            }),
                        },
                    );
                }
                _ => {
                    let early_cycle_count = count_until_rearm - 1;
                    effect_infos.insert(
                        effect_key,
                        FtrEffectInfo {
                            cycle: Cycle::Reload2(CycleReload2 {
                                inner1: CycleInnerLimited {
                                    active_time: duration_s,
                                    inactive_time: cd_inactivity_s,
                                    repeat_count: early_cycle_count,
                                },
                                inner2: CycleInnerLimited {
                                    active_time: duration_s,
                                    inactive_time: OF(0.0),
                                    repeat_count: 1,
                                },
                            }),
                            rearm: Some(FtrEffectRearmInfo {
                                time_until_rearm_s: (duration_s + cd_inactivity_s) * early_cycle_count as f64
                                    + duration_s,
                                full_rearm_time_s,
                                charge_rearm_time_s: effect_data.charge_reload_time_s,
                            }),
                        },
                    );
                }
            }
        }
    }
}

fn process_refuel(mut effect_infos: RMap<REffectKey, FtrEffectInfo>) -> RMap<REffectKey, Cycle> {
    let mut cycle_infos = RMap::with_capacity(effect_infos.len());
    // Get effect which runs out of its charges fastest
    let (effect_key, cycle, rearm) = match effect_infos
        .iter()
        .filter_map(|(effect_key, effect_info)| match effect_info.rearm {
            Some(rearm_info) => Some((*effect_key, effect_info.cycle, rearm_info)),
            None => None,
        })
        .min_by_key(|(_, _, rearm)| rearm.time_until_rearm_s)
    {
        Some((effect_key, cycle, rearm)) => {
            // Remove it from source map, since we extracted the data we needed anyway
            effect_infos.remove(&effect_key);
            (effect_key, cycle, rearm)
        }
        None => {
            // No rearm data means all effects can cycle infinitely, just return everything we
            // received in this case
            for (effect_key, effect_info) in effect_infos.into_iter() {
                cycle_infos.insert(effect_key, effect_info.cycle);
            }
            return cycle_infos;
        }
    };
    // Time it takes to rearm just abilities
    let mut max_rearm_time_s = rearm.full_rearm_time_s;
    cycle_infos
}
