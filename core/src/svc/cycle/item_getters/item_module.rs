use either::Either;
use ordered_float::Float;

use super::shared::{CyclingOptions, SelfKillerInfo};
use crate::{
    def::{AttrVal, Count, OF, SERVER_TICK_S},
    nd::{NEffectChargeDepl, NEffectChargeDeplCrystal},
    rd::{REffectChargeLoc, REffectKey},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{
            CycleDataFull, CycleInterrupt, CycleSeq,
            effect_charge_info::{
                get_eci_autocharge, get_eci_charge_rate, get_eci_crystal, get_eci_uncharged, get_eci_undepletable,
            },
            seq_inf::CSeqInf,
            seq_lim::CSeqLim,
            seq_lim_inf::CSeqLimInf,
            seq_lim_sin_inf::CSeqLimSinInf,
            seq_loop_lim_sin::CycleSeqLoopLimSin,
        },
        funcs,
    },
    ud::{UItem, UItemId, UModule},
    util::{FLOAT_TOLERANCE, InfCount, RMap},
};

pub(super) fn get_module_cseq_map(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemId,
    item: &UItem,
    module: &UModule,
    options: CyclingOptions,
    ignore_state: bool,
) -> Option<RMap<REffectKey, CycleSeq>> {
    if !module.is_loaded() {
        return None;
    };
    let mut cseq_map = RMap::new();
    let mut self_killers = Vec::new();
    let effect_keys = match ignore_state {
        true => Either::Left(module.get_effect_datas().unwrap().keys().copied()),
        false => Either::Right(module.get_reffs().unwrap().iter().copied()),
    };
    for effect_key in effect_keys {
        fill_module_effect_info(
            &mut cseq_map,
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
            .min_by_key(|sk_info| sk_info.duration)
            .unwrap()
            .effect_key;
        cseq_map.retain(|&k, _| k == fastest_sk_effect_key);
    }
    Some(cseq_map)
}

fn fill_module_effect_info(
    cseq_map: &mut RMap<REffectKey, CycleSeq>,
    self_killers: &mut Vec<SelfKillerInfo>,
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemId,
    item: &UItem,
    module: &UModule,
    effect_key: REffectKey,
    options: CyclingOptions,
) {
    let effect = ctx.u_data.src.get_effect(effect_key);
    if !effect.is_active_with_duration {
        return;
    }
    // No appropriate duration - no info
    let duration = match funcs::get_effect_duration_s(ctx, calc, item_key, effect) {
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
            duration: duration,
        });
        cseq_map.insert(
            effect_key,
            CycleSeq::Lim(CSeqLim {
                data: CycleDataFull {
                    time: duration,
                    interrupt: None,
                    chargedness: charge_info.get_first_cycle_chargeness(),
                },
                repeat_count: 1,
            }),
        );
        return;
    }
    let cooldown = Float::max(
        OF(0.0),
        calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().mod_reactivation_delay, OF(0.0))
            .unwrap()
            / 1000.0,
    );
    // Decide if interruptions happen every cycle based on reactivation delay value
    let int_cd = cooldown > FLOAT_TOLERANCE;
    let sim_options = match options {
        CyclingOptions::Sim(sim_options) => sim_options,
        // If burst cycle mode was requested, just assume first cycle is the "most charged", and
        // infinitely repeat it
        CyclingOptions::Burst => {
            cseq_map.insert(
                effect_key,
                CycleSeq::Inf(CSeqInf {
                    data: CycleDataFull {
                        time: duration + cooldown,
                        interrupt: CycleInterrupt::try_new(int_cd, false),
                        chargedness: charge_info.get_first_cycle_chargeness(),
                    },
                }),
            );
            return;
        }
    };
    let full_count = match charge_info.fully_charged {
        InfCount::Count(full_count) => full_count,
        InfCount::Infinite => {
            cseq_map.insert(
                effect_key,
                CycleSeq::Inf(CSeqInf {
                    data: CycleDataFull {
                        time: duration + cooldown,
                        interrupt: CycleInterrupt::try_new(int_cd, false),
                        chargedness: Some(OF(1.0)),
                    },
                }),
            );
            return;
        }
    };
    let cseq = match (
        full_count > 0,
        charge_info.part_charged.is_some(),
        charge_info.can_run_uncharged,
    ) {
        // Can't cycle at all, should've been handled earlier
        (false, false, false) => return,
        // Infinitely cycling modules without charge
        (false, false, true) => CycleSeq::Inf(CSeqInf {
            data: CycleDataFull {
                time: duration + cooldown,
                interrupt: CycleInterrupt::try_new(int_cd, false),
                chargedness: None,
            },
        }),
        // Only partially charged, has to reload every cycle
        (false, true, false) => part_r(
            ctx,
            calc,
            item_key,
            duration,
            cooldown,
            int_cd,
            charge_info.part_charged,
        ),
        // Only partially charged cycle, but can cycle without charges
        (false, true, true) => match ctx
            .u_data
            .get_item_key_reload_optionals(item_key, sim_options.reload_optionals)
        {
            true => part_r(
                ctx,
                calc,
                item_key,
                duration,
                cooldown,
                int_cd,
                charge_info.part_charged,
            ),
            false => CycleSeq::LimInf(CSeqLimInf {
                p1_data: CycleDataFull {
                    time: duration + cooldown,
                    interrupt: CycleInterrupt::try_new(int_cd, false),
                    chargedness: charge_info.part_charged,
                },
                p1_repeat_count: 1,
                p2_data: CycleDataFull {
                    time: duration + cooldown,
                    interrupt: CycleInterrupt::try_new(int_cd, false),
                    chargedness: None,
                },
            }),
        },
        // Only fully charged, has to reload after charges are out
        (true, false, false) => full_r(ctx, calc, item_key, duration, cooldown, int_cd, full_count),
        // Only fully charged, but can cycle without charges
        (true, false, true) => match ctx
            .u_data
            .get_item_key_reload_optionals(item_key, sim_options.reload_optionals)
        {
            true => full_r(ctx, calc, item_key, duration, cooldown, int_cd, full_count),
            false => CycleSeq::LimInf(CSeqLimInf {
                p1_data: CycleDataFull {
                    time: duration + cooldown,
                    interrupt: CycleInterrupt::try_new(int_cd, false),
                    chargedness: Some(OF(1.0)),
                },
                p1_repeat_count: full_count,
                p2_data: CycleDataFull {
                    time: duration + cooldown,
                    interrupt: CycleInterrupt::try_new(int_cd, false),
                    chargedness: None,
                },
            }),
        },
        // Fully charged + partially charged + can't run uncharged
        (true, true, false) => both_r(
            ctx,
            calc,
            item_key,
            duration,
            cooldown,
            int_cd,
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
                    cooldown,
                    int_cd,
                    full_count,
                    charge_info.part_charged,
                ),
                false => CycleSeq::LimSinInf(CSeqLimSinInf {
                    p1_data: CycleDataFull {
                        time: duration + cooldown,
                        interrupt: CycleInterrupt::try_new(int_cd, false),
                        chargedness: Some(OF(1.0)),
                    },
                    p1_repeat_count: full_count,
                    p2_data: CycleDataFull {
                        time: duration + cooldown,
                        interrupt: CycleInterrupt::try_new(int_cd, false),
                        chargedness: charge_info.part_charged,
                    },
                    p3_data: CycleDataFull {
                        time: duration + cooldown,
                        interrupt: CycleInterrupt::try_new(int_cd, false),
                        chargedness: None,
                    },
                }),
            }
        }
    };
    cseq_map.insert(effect_key, cseq);
}

fn get_reload_time(ctx: SvcCtx, calc: &mut Calc, item_key: UItemId) -> AttrVal {
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
    item_key: UItemId,
    duration: AttrVal,
    cooldown: AttrVal,
    int_cd: bool,
    part_value: Option<AttrVal>,
) -> CycleSeq {
    CycleSeq::Inf(CSeqInf {
        data: CycleDataFull {
            time: duration + Float::max(get_reload_time(ctx, calc, item_key), cooldown),
            interrupt: CycleInterrupt::try_new(int_cd, true),
            chargedness: part_value,
        },
    })
}

fn full_r(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemId,
    duration: AttrVal,
    cooldown: AttrVal,
    int_cd: bool,
    full_count: Count,
) -> CycleSeq {
    match full_count {
        1 => CycleSeq::Inf(CSeqInf {
            data: CycleDataFull {
                time: duration + Float::max(get_reload_time(ctx, calc, item_key), cooldown),
                interrupt: CycleInterrupt::try_new(int_cd, true),
                chargedness: Some(OF(1.0)),
            },
        }),
        _ => CycleSeq::LoopLimSin(CycleSeqLoopLimSin {
            p1_data: CycleDataFull {
                time: duration + cooldown,
                interrupt: CycleInterrupt::try_new(int_cd, false),
                chargedness: Some(OF(1.0)),
            },
            p1_repeat_count: full_count - 1,
            p2_data: CycleDataFull {
                time: duration + Float::max(get_reload_time(ctx, calc, item_key), cooldown),
                interrupt: CycleInterrupt::try_new(int_cd, true),
                chargedness: Some(OF(1.0)),
            },
        }),
    }
}

fn both_r(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemId,
    duration: AttrVal,
    cooldown: AttrVal,
    int_cd: bool,
    full_count: Count,
    part_value: Option<AttrVal>,
) -> CycleSeq {
    CycleSeq::LoopLimSin(CycleSeqLoopLimSin {
        p1_data: CycleDataFull {
            time: duration + cooldown,
            interrupt: CycleInterrupt::try_new(int_cd, false),
            chargedness: Some(OF(1.0)),
        },
        p1_repeat_count: full_count,
        p2_data: CycleDataFull {
            time: duration + Float::max(get_reload_time(ctx, calc, item_key), cooldown),
            interrupt: CycleInterrupt::try_new(int_cd, true),
            chargedness: part_value,
        },
    })
}
