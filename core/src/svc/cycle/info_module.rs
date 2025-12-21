use either::Either;

use super::{
    charged_info::{
        get_autocharge_charged_info, get_charge_rate_charged_info, get_crystal_charged_info,
        get_uncharged_charged_info, get_undepletable_charged_info,
    },
    cycle::Cycle,
    cycle_inner_limited::CycleInnerLimited,
    cycle_limited::CycleLimited,
    cycle_reload2::CycleReload2,
    info_shared::{CycleOptions, SelfKillerInfo},
};
use crate::{
    def::{OF, SERVER_TICK_S},
    nd::{NEffectChargeDepl, NEffectChargeDeplCrystal},
    rd::{REffectChargeLoc, REffectKey},
    svc::{SvcCtx, calc::Calc, eff_funcs},
    ud::{UItem, UItemKey, UModule},
    util::{FLOAT_TOLERANCE, InfCount, RMap},
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
    if !effect.is_active_with_duration {
        return;
    }
    // No appropriate duration - no info
    let duration_s = match eff_funcs::get_effect_duration_s(ctx, calc, item_key, effect) {
        Some(duration_s) => duration_s,
        None => return,
    };
    // Charge count info
    let charged_cycle_count = match &effect.charge {
        Some(n_charge) => match n_charge.location {
            REffectChargeLoc::Autocharge(_) => get_autocharge_charged_info(item, effect.key),
            REffectChargeLoc::Loaded(n_charge_depletion) => match n_charge_depletion {
                NEffectChargeDepl::ChargeRate(n_charge_rate) => {
                    get_charge_rate_charged_info(ctx, module, n_charge_rate)
                }
                NEffectChargeDepl::Crystal(n_charge_crystal) => {
                    get_crystal_charged_info(ctx, calc, module, n_charge_crystal)
                }
                NEffectChargeDepl::Undepletable => get_undepletable_charged_info(),
            },
            // targetAttack effect has 2 distinct options for modules:
            // - lasers: regular crystal cycle getter
            // - civilian guns: infinite cycles
            // Here, we rely on module capacity to differentiate between those
            REffectChargeLoc::TargetAttack => match module.get_axt().unwrap().capacity > OF(0.0) {
                true => get_crystal_charged_info(ctx, calc, module, NEffectChargeDeplCrystal { .. }),
                false => get_undepletable_charged_info(),
            },
        },
        None => get_uncharged_charged_info(),
    };
    // Completely skip effects which can't cycle
    if charged_cycle_count.is_unrunnable() {
        return;
    }
    // Self-killers are fairly trivial. Record info about them and go to next effect
    if effect.kills_item {
        self_killers.push(SelfKillerInfo { effect_key, duration_s });
        cycle_infos.insert(
            effect_key,
            Cycle::Limited(CycleLimited {
                inner: CycleInnerLimited {
                    active_time: duration_s,
                    inactive_time: OF(0.0),
                    interrupt: true,
                    charged: charged_cycle_count.get_first_cycle_chargeness(),
                    repeat_count: 1,
                },
            }),
        );
        return;
    }
    let reactivation_delay_s = (calc
        .get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().mod_reactivation_delay, OF(0.0))
        .unwrap()
        / 1000.0)
        .max(OF(0.0));
    let count_until_reload = match cycle_count {
        InfCount::Count(count_until_reload) => count_until_reload,
        // No need for complex logic when module is infinitely cycling
        InfCount::Infinite => {
            cycle_infos.insert(
                effect_key,
                Cycle::Simple(CycleSimple {
                    active_time: duration_s,
                    inactive_time: reactivation_delay_s,
                    repeat_count: InfCount::Infinite,
                }),
            );
            return;
        }
    };
    let reload_time_s = match options.reload_mode {
        // When considering burst calculations, just set reload to 0
        CycleOptionReload::Burst => OF(0.0),
        CycleOptionReload::Sim => {
            let reload_time_s = calc
                .get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().reload_time, OF(0.0))
                .unwrap()
                / 1000.0;
            match reload_time_s > FLOAT_TOLERANCE {
                // If reload time is defined and positive, ensure it takes at least 1 tick
                true => reload_time_s.max(SERVER_TICK_S),
                false => OF(0.0),
            }
        }
    };
    // Module can be reloaded during reactivation delay; if reactivation delay is longer, return
    // simple cycle
    if reactivation_delay_s >= reload_time_s {
        cycle_infos.insert(
            effect_key,
            Cycle::Reload1(CycleReload1 {
                inner: CycleInnerLimited {
                    active_time: duration_s,
                    inactive_time: reactivation_delay_s,
                    repeat_count: count_until_reload,
                },
            }),
        );
        return;
    }
    // If effect can cycle just 1 time, return simpler cycle as well
    if count_until_reload == 1 {
        cycle_infos.insert(
            effect_key,
            Cycle::Reload1(CycleReload1 {
                inner: CycleInnerLimited {
                    active_time: duration_s,
                    inactive_time: reload_time_s,
                    repeat_count: count_until_reload,
                },
            }),
        );
        return;
    }
    cycle_infos.insert(
        effect_key,
        Cycle::Reload2(CycleReload2 {
            inner1: CycleInnerLimited {
                active_time: duration_s,
                inactive_time: reactivation_delay_s,
                repeat_count: count_until_reload - 1,
            },
            inner2: CycleInnerLimited {
                active_time: duration_s,
                inactive_time: reload_time_s,
                repeat_count: 1,
            },
        }),
    );
}
