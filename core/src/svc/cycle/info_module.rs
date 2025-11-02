use either::Either;

use super::{
    cycle::Cycle,
    cycle_reload1::CycleReload1,
    cycle_reload2::CycleReload2,
    cycle_shared::CycleInner,
    cycle_simple::CycleSimple,
    info_shared::{CycleOptionReload, CycleOptions, SelfKillerInfo},
    until_reload::{get_autocharge_cycle_count, get_charge_rate_cycle_count, get_crystal_cycle_count},
};
use crate::{
    ac,
    def::{OF, SERVER_TICK_S},
    nd::{NEffectChargeDepl, NEffectChargeLoc},
    rd::REffectKey,
    svc::{SvcCtx, calc::Calc, eff_funcs},
    ud::{UItem, UItemKey, UModule},
    util::{InfCount, RMap},
};

pub(super) fn get_module_cycle_info(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    item: &UItem,
    module: &UModule,
    options: CycleOptions,
    ignore_state: bool,
) -> Option<RMap<REffectKey, Cycle>> {
    if !module.is_loaded() {
        return None;
    };
    let mut cycle_infos = RMap::new();
    let mut self_killers = Vec::new();
    let effect_keys = match ignore_state {
        true => Either::Left(module.get_effect_datas().unwrap().keys().copied()),
        false => Either::Right(module.get_reffs().unwrap().iter().copied()),
    };
    for effect_key in effect_keys {
        fill_module_effect_info(
            &mut cycle_infos,
            &mut self_killers,
            ctx,
            calc,
            item_key,
            item,
            module,
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
        cycle_infos.retain(|&k, _| k == fastest_sk_effect_key);
    }
    Some(cycle_infos)
}

fn fill_module_effect_info(
    cycle_infos: &mut RMap<REffectKey, Cycle>,
    self_killers: &mut Vec<SelfKillerInfo>,
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    item: &UItem,
    module: &UModule,
    effect_key: REffectKey,
    options: CycleOptions,
) {
    let effect = ctx.u_data.src.get_effect(effect_key);
    if !effect.is_active_with_duration() {
        return;
    }
    // No appropriate duration - no info
    let duration_s = match eff_funcs::get_effect_duration_s(ctx, calc, item_key, effect) {
        Some(duration_s) => duration_s,
        None => return,
    };
    // Charge count info
    let cycle_count = match effect.get_charge_info() {
        Some(charge_info) => match charge_info.location {
            NEffectChargeLoc::Autocharge(_) => get_autocharge_cycle_count(item, effect),
            NEffectChargeLoc::Loaded(charge_depletion) => match charge_depletion {
                NEffectChargeDepl::ChargeRate { can_run_uncharged } => {
                    get_charge_rate_cycle_count(ctx, module, can_run_uncharged, options.reload_optionals)
                }
                NEffectChargeDepl::Crystal { can_run_uncharged } => {
                    get_crystal_cycle_count(ctx, module, can_run_uncharged, options.reload_optionals)
                }
                NEffectChargeDepl::None => InfCount::Infinite,
            },
            // targetAttack effect has 2 distinct options for modules:
            // - lasers: regular crystal cycle getter
            // - civilian guns: infinite cycles
            // Here, we rely on module capacity to differentiate between those
            NEffectChargeLoc::TargetAttack(_) => match module.get_axt().unwrap().capacity > OF(0.0) {
                true => get_crystal_cycle_count(ctx, module, false, options.reload_optionals),
                false => InfCount::Infinite,
            },
        },
        None => InfCount::Infinite,
    };
    // Completely skip effects which can't cycle
    if cycle_count == InfCount::Count(0) {
        return;
    }
    // Self-killers are fairly trivial. Record info about them and go to next effect
    if effect.kills_item() {
        self_killers.push(SelfKillerInfo { effect_key, duration_s });
        cycle_infos.insert(
            effect_key,
            Cycle::Simple(CycleSimple {
                active_time: duration_s,
                inactive_time: OF(0.0),
                repeat_count: InfCount::Count(1),
            }),
        );
        return;
    }
    let reactivation_delay_s = (calc
        .get_item_attr_val_extra(ctx, item_key, &ac::attrs::MOD_REACTIVATION_DELAY)
        .unwrap()
        / 1000.0)
        .max(OF(0.0));
    match cycle_count {
        // When we have to handle reload, result is a bit complex
        InfCount::Count(count_until_reload) => {
            let reload_time_s = match options.reload_mode {
                // When considering burst calculations, just set reload to 0
                CycleOptionReload::Burst => OF(0.0),
                CycleOptionReload::Sim => {
                    let reload_time_s = calc
                        .get_item_attr_val_extra(ctx, item_key, &ac::attrs::RELOAD_TIME)
                        .unwrap()
                        / 1000.0;
                    match reload_time_s > OF(0.0) {
                        // If reload time is defined and positive, ensure it takes at least 1 tick
                        true => reload_time_s.max(SERVER_TICK_S),
                        false => OF(0.0),
                    }
                }
            };
            // Module can be reloaded during reactivation delay
            let final_inactive_time = reload_time_s.max(reactivation_delay_s);
            let final_cycle_count = 1;
            let early_cycle_count = count_until_reload - final_cycle_count;
            match early_cycle_count {
                // When module can do only one cycle per clip - return reloadable cycle, with
                // inner cycle count of 1
                0 => {
                    let inner = CycleInner {
                        active_time: duration_s,
                        inactive_time: final_inactive_time,
                        repeat_count: final_cycle_count,
                    };
                    cycle_infos.insert(effect_key, Cycle::Reload1(CycleReload1 { inner }));
                }
                // When it does more than one cycle per clip - mark final cycle as reload
                _ => {
                    let inner_early = CycleInner {
                        active_time: duration_s,
                        inactive_time: reactivation_delay_s,
                        repeat_count: early_cycle_count,
                    };
                    let inner_final = CycleInner {
                        active_time: duration_s,
                        inactive_time: final_inactive_time,
                        repeat_count: final_cycle_count,
                    };
                    cycle_infos.insert(
                        effect_key,
                        Cycle::Reload2(CycleReload2 {
                            inner_early,
                            inner_final,
                        }),
                    );
                }
            }
        }
        // Infinitely cycling - return simple infinitely repeating cycle
        InfCount::Infinite => {
            cycle_infos.insert(
                effect_key,
                Cycle::Simple(CycleSimple {
                    active_time: duration_s,
                    inactive_time: reactivation_delay_s,
                    repeat_count: InfCount::Infinite,
                }),
            );
        }
    }
}
