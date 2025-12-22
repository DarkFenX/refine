use either::Either;
use ordered_float::Float;

use super::shared::{CycleOptions, SelfKillerInfo};
use crate::{
    def::{AttrVal, Count, OF, SERVER_TICK_S},
    nd::{NEffectChargeDepl, NEffectChargeDeplCrystal},
    rd::{REffectChargeLoc, REffectKey},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{
            Cycle,
            cycle_inf::CycleInf,
            cycle_lim::CycleLim,
            cycle_lim_inf::CycleLimInf,
            cycle_lim_sin_inf::CycleLimSinInf,
            cycle_loop_lim_sin::CycleLoopLimSin,
            effect_charge_info::{
                get_eci_autocharge, get_eci_charge_rate, get_eci_crystal, get_eci_uncharged, get_eci_undepletable,
            },
        },
        eff_funcs,
    },
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
    let duration = match eff_funcs::get_effect_duration_s(ctx, calc, item_key, effect) {
        Some(duration) => duration,
        None => return,
    };
    // Charge count info
    let charge_info = match &effect.charge {
        Some(n_charge) => match n_charge.location {
            REffectChargeLoc::Autocharge(_) => get_eci_autocharge(item, effect.key),
            REffectChargeLoc::Loaded(n_charge_depletion) => match n_charge_depletion {
                NEffectChargeDepl::ChargeRate(n_charge_rate) => get_eci_charge_rate(ctx, module, n_charge_rate),
                NEffectChargeDepl::Crystal(n_charge_crystal) => get_eci_crystal(ctx, calc, module, n_charge_crystal),
                NEffectChargeDepl::Undepletable => get_eci_undepletable(),
            },
            // targetAttack effect has 2 distinct options for modules:
            // - lasers: regular crystal cycle getter
            // - civilian guns: infinite cycles
            // Here, we rely on module capacity to differentiate between those
            REffectChargeLoc::TargetAttack => match module.get_axt().unwrap().capacity > OF(0.0) {
                true => get_eci_crystal(ctx, calc, module, NEffectChargeDeplCrystal { .. }),
                false => get_eci_undepletable(),
            },
        },
        None => get_eci_uncharged(),
    };
    // Completely skip effects which can't cycle
    if charge_info.is_unrunnable() {
        return;
    }
    // Record info about self-killers and bail, those do not depend on cycling options
    if effect.kills_item {
        self_killers.push(SelfKillerInfo {
            effect_key,
            duration_s: duration,
        });
        cycle_infos.insert(
            effect_key,
            Cycle::Lim(CycleLim {
                active_time: duration,
                inactive_time: OF(0.0),
                interrupt: true,
                charged: charge_info.get_first_cycle_chargeness(),
                repeat_count: 1,
            }),
        );
        return;
    }
    let cycle_dt = Float::max(
        OF(0.0),
        calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().mod_reactivation_delay, OF(0.0))
            .unwrap()
            / 1000.0,
    );
    // Decide if interruptions happen every cycle based on reactivation delay value
    let force_int = cycle_dt > FLOAT_TOLERANCE;
    let sim_options = match options {
        CycleOptions::Sim(sim_options) => sim_options,
        // If burst cycle mode was requested, just assume first cycle is the "most charged", and
        // infinitely repeat it
        CycleOptions::Burst => {
            cycle_infos.insert(
                effect_key,
                Cycle::Inf(CycleInf {
                    active_time: duration,
                    inactive_time: cycle_dt,
                    interrupt: force_int,
                    charged: charge_info.get_first_cycle_chargeness(),
                }),
            );
            return;
        }
    };
    let full_count = match charge_info.fully_charged {
        InfCount::Count(full_count) => full_count,
        InfCount::Infinite => {
            cycle_infos.insert(
                effect_key,
                Cycle::Inf(CycleInf {
                    active_time: duration,
                    inactive_time: cycle_dt,
                    interrupt: force_int,
                    charged: Some(OF(1.0)),
                }),
            );
            return;
        }
    };
    let cycle = match (
        full_count > 0,
        charge_info.part_charged.is_some(),
        charge_info.can_run_uncharged,
    ) {
        // Can't cycle at all, should've been handled earlier
        (false, false, false) => return,
        // Infinitely cycling modules without charge
        (false, false, true) => Cycle::Inf(CycleInf {
            active_time: duration,
            inactive_time: cycle_dt,
            interrupt: force_int,
            charged: None,
        }),
        // Only partially charged, has to reload every cycle
        (false, true, false) => part_r(ctx, calc, item_key, duration, cycle_dt, charge_info.part_charged),
        // Only partially charged cycle, but can cycle without charges
        (false, true, true) => match ctx
            .u_data
            .get_item_key_reload_optionals(item_key, sim_options.reload_optionals)
        {
            true => part_r(ctx, calc, item_key, duration, cycle_dt, charge_info.part_charged),
            false => Cycle::LimInf(CycleLimInf {
                p1_active_time: duration,
                p1_inactive_time: cycle_dt,
                p1_interrupt: force_int,
                p1_charged: charge_info.part_charged,
                p1_repeat_count: 1,
                p2_active_time: duration,
                p2_inactive_time: cycle_dt,
                p2_interrupt: force_int,
                p2_charged: None,
            }),
        },
        // Only fully charged, has to reload after charges are out
        (true, false, false) => full_r(ctx, calc, item_key, duration, cycle_dt, force_int, full_count),
        // Only fully charged, but can cycle without charges
        (true, false, true) => match ctx
            .u_data
            .get_item_key_reload_optionals(item_key, sim_options.reload_optionals)
        {
            true => full_r(ctx, calc, item_key, duration, cycle_dt, force_int, full_count),
            false => Cycle::LimInf(CycleLimInf {
                p1_active_time: duration,
                p1_inactive_time: cycle_dt,
                p1_interrupt: force_int,
                p1_charged: Some(OF(1.0)),
                p1_repeat_count: full_count,
                p2_active_time: duration,
                p2_inactive_time: cycle_dt,
                p2_interrupt: force_int,
                p2_charged: None,
            }),
        },
        // Fully charged + partially charged + can't run uncharged
        (true, true, false) => both_r(
            ctx,
            calc,
            item_key,
            duration,
            cycle_dt,
            force_int,
            full_count,
            charge_info.part_charged,
        ),
        // Fully charged + partially charged + can cycle uncharged
        (true, true, true) => {
            match ctx
                .u_data
                .get_item_key_reload_optionals(item_key, sim_options.reload_optionals)
            {
                true => both_r(
                    ctx,
                    calc,
                    item_key,
                    duration,
                    cycle_dt,
                    force_int,
                    full_count,
                    charge_info.part_charged,
                ),
                false => Cycle::LimSinInf(CycleLimSinInf {
                    p1_active_time: duration,
                    p1_inactive_time: cycle_dt,
                    p1_interrupt: force_int,
                    p1_charged: Some(OF(1.0)),
                    p1_repeat_count: full_count,
                    p2_active_time: duration,
                    p2_inactive_time: cycle_dt,
                    p2_interrupt: force_int,
                    p2_charged: charge_info.part_charged,
                    p3_active_time: duration,
                    p3_inactive_time: cycle_dt,
                    p3_interrupt: force_int,
                    p3_charged: None,
                }),
            }
        }
    };
    cycle_infos.insert(effect_key, cycle);
}

fn get_reload_time(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> AttrVal {
    // All reloads can't take less than server tick realistically. E.g. lasers have almost 0 reload
    // time but take 1-2 seconds to reload
    Float::max(
        SERVER_TICK_S,
        calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().reload_time, OF(0.0))
            .unwrap()
            / 1000.0,
    )
}

fn part_r(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    duration: AttrVal,
    cycle_dt: AttrVal,
    part_value: Option<AttrVal>,
) -> Cycle {
    Cycle::Inf(CycleInf {
        active_time: duration,
        inactive_time: Float::max(get_reload_time(ctx, calc, item_key), cycle_dt),
        interrupt: true,
        charged: part_value,
    })
}

fn full_r(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    duration: AttrVal,
    cycle_dt: AttrVal,
    force_int: bool,
    full_count: Count,
) -> Cycle {
    match full_count {
        1 => Cycle::Inf(CycleInf {
            active_time: duration,
            inactive_time: Float::max(get_reload_time(ctx, calc, item_key), cycle_dt),
            interrupt: true,
            charged: Some(OF(1.0)),
        }),
        _ => Cycle::LoopLimSin(CycleLoopLimSin {
            p1_active_time: duration,
            p1_inactive_time: cycle_dt,
            p1_interrupt: force_int,
            p1_charged: Some(OF(1.0)),
            p1_repeat_count: full_count - 1,
            p2_active_time: duration,
            p2_inactive_time: Float::max(get_reload_time(ctx, calc, item_key), cycle_dt),
            p2_interrupt: true,
            p2_charged: Some(OF(1.0)),
        }),
    }
}

fn both_r(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    duration: AttrVal,
    cycle_dt: AttrVal,
    force_int: bool,
    full_count: Count,
    part_value: Option<AttrVal>,
) -> Cycle {
    Cycle::LoopLimSin(CycleLoopLimSin {
        p1_active_time: duration,
        p1_inactive_time: cycle_dt,
        p1_interrupt: force_int,
        p1_charged: Some(OF(1.0)),
        p1_repeat_count: full_count,
        p2_active_time: duration,
        p2_inactive_time: Float::max(get_reload_time(ctx, calc, item_key), cycle_dt),
        p2_interrupt: true,
        p2_charged: part_value,
    })
}
