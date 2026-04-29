use super::{
    map::CseqMap,
    shared::{CyclingOptions, SelfKillerEffectInfo, SelfKillerItemInfo},
};
use crate::{
    def::SERVER_TICK_S,
    misc::{InfCount, OptionalReload},
    nd::{NEffectChargeDepl, NEffectChargeDeplCrystal},
    num::{Count, PValue, UnitInterval, Value},
    rd::{REffectChargeLoc, REffectId},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{
            CycleActive, CycleDataFull, CycleDtSoft, CycleSeq,
            effect_charge_info::{
                get_eci_autocharge, get_eci_charge_rate, get_eci_crystal, get_eci_uncharged, get_eci_undepletable,
            },
            seq_inf::CSeqInf,
            seq_lim::CSeqLim,
            seq_lim_inf::CSeqLimInf,
            seq_lim_sin_inf::CSeqLimSinInf,
            seq_loop_lim_sin::CSeqLoopLimSin,
        },
        funcs,
    },
    ud::{UItem, UItemId, UModule},
};

#[must_use]
pub(super) fn get_module_cseq_map(
    reuse_cseq_map: &mut CseqMap,
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    item: &UItem,
    module: &UModule,
    options: CyclingOptions,
) -> bool {
    if !module.is_loaded() {
        return false;
    };
    reuse_cseq_map.clear();
    let mut sk_item_info = SelfKillerItemInfo::new();
    for &effect_rid in module.get_reffs().unwrap().iter() {
        fill_module_effect_info(
            reuse_cseq_map,
            &mut sk_item_info,
            ctx,
            calc,
            item_uid,
            item,
            module,
            effect_rid,
            options,
        );
    }
    // If there are any self-killer effects, the fastest one is used, and other effects are
    // discarded
    if let Some(sk_effect_rid) = sk_item_info.get_effect_rid() {
        reuse_cseq_map.retain(|&k, _| k == sk_effect_rid);
    }
    true
}

fn fill_module_effect_info(
    reuse_cseq_map: &mut CseqMap,
    sk_item_info: &mut SelfKillerItemInfo,
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    item: &UItem,
    module: &UModule,
    effect_rid: REffectId,
    options: CyclingOptions,
) {
    let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
    if !effect.is_active_with_duration {
        return;
    }
    // No appropriate duration - no info
    let active_duration = match funcs::get_effect_duration_s(ctx, calc, item_uid, effect) {
        Some(duration) => duration,
        None => return,
    };
    // Charge count info
    let charge_info = match &effect.charge {
        Some(n_charge) => match n_charge.location {
            REffectChargeLoc::Autocharge(_) => get_eci_autocharge(item, effect.rid),
            REffectChargeLoc::Loaded(n_charge_depletion) => match n_charge_depletion {
                NEffectChargeDepl::ChargeRate(n_charge_rate) => get_eci_charge_rate(ctx, module, n_charge_rate),
                NEffectChargeDepl::Crystal(n_charge_crystal) => get_eci_crystal(ctx, calc, module, n_charge_crystal),
                NEffectChargeDepl::Undepletable => get_eci_undepletable(),
            },
            // targetAttack effect has 2 distinct options for modules:
            // - lasers: regular crystal cycle getter
            // - civilian guns: infinite cycles
            // Here, we rely on module capacity to differentiate between those
            REffectChargeLoc::TargetAttack => match module.get_axt().unwrap().capacity > PValue::FLOAT_TOLERANCE {
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
        sk_item_info.push(SelfKillerEffectInfo {
            effect_rid,
            active_duration,
        });
        reuse_cseq_map.insert(
            effect_rid,
            CycleSeq::Lim(CSeqLim {
                data: CycleDataFull {
                    active: CycleActive {
                        duration: active_duration,
                        chargedness: charge_info.get_first_cycle_chargedness(),
                    },
                    dt_soft: None,
                },
                repeat_count: Count::ONE,
            }),
        );
        return;
    }
    let cooldown_duration = PValue::from_value_clamped(
        calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().mod_reactivation_delay, Value::ZERO)
            .unwrap()
            / Value::THOUSAND,
    );
    // Decide if interruptions happen every cycle based on reactivation delay value
    let dt_soft_cd = cooldown_duration > PValue::FLOAT_TOLERANCE;
    let sim_options = match options {
        CyclingOptions::Sim(sim_options) => sim_options,
        // If burst cycle mode was requested, just assume first cycle is the "most charged", and
        // infinitely repeat it
        CyclingOptions::Burst => {
            reuse_cseq_map.insert(
                effect_rid,
                CycleSeq::Inf(CSeqInf {
                    data: CycleDataFull {
                        active: CycleActive {
                            duration: active_duration,
                            chargedness: charge_info.get_first_cycle_chargedness(),
                        },
                        dt_soft: CycleDtSoft::try_new(cooldown_duration, dt_soft_cd, false),
                    },
                    dt_hard: None,
                }),
            );
            return;
        }
    };
    let full_count = match charge_info.fully_charged {
        InfCount::Count(full_count) => full_count,
        InfCount::Infinite => {
            reuse_cseq_map.insert(
                effect_rid,
                CycleSeq::Inf(CSeqInf {
                    data: CycleDataFull {
                        active: CycleActive {
                            duration: active_duration,
                            chargedness: Some(UnitInterval::ONE),
                        },
                        dt_soft: CycleDtSoft::try_new(cooldown_duration, dt_soft_cd, false),
                    },
                    dt_hard: None,
                }),
            );
            return;
        }
    };
    let cseq = match (
        full_count > Count::ZERO,
        charge_info.part_charged.is_some(),
        charge_info.can_run_uncharged,
    ) {
        // Can't cycle at all, should've been handled earlier
        (false, false, false) => return,
        // Infinitely cycling modules without charge
        (false, false, true) => CycleSeq::Inf(CSeqInf {
            data: CycleDataFull {
                active: CycleActive {
                    duration: active_duration,
                    chargedness: None,
                },
                dt_soft: CycleDtSoft::try_new(cooldown_duration, dt_soft_cd, false),
            },
            dt_hard: None,
        }),
        // Only partially charged, has to reload every cycle
        (false, true, false) => part_r(
            ctx,
            calc,
            item_uid,
            active_duration,
            cooldown_duration,
            dt_soft_cd,
            charge_info.part_charged,
        ),
        // Only partially charged cycle, but can cycle without charges
        (false, true, true) => match ctx
            .u_data
            .get_item_optional_reload(item_uid, sim_options.optional_reloads)
        {
            OptionalReload::OnEmpty => part_r(
                ctx,
                calc,
                item_uid,
                active_duration,
                cooldown_duration,
                dt_soft_cd,
                charge_info.part_charged,
            ),
            OptionalReload::Disabled => {
                let dt_soft = CycleDtSoft::try_new(cooldown_duration, dt_soft_cd, false);
                CycleSeq::LimInf(CSeqLimInf {
                    p1_data: CycleDataFull {
                        active: CycleActive {
                            duration: active_duration,
                            chargedness: charge_info.part_charged,
                        },
                        dt_soft,
                    },
                    p1_repeat_count: Count::ONE,
                    p2_data: CycleDataFull {
                        active: CycleActive {
                            duration: active_duration,
                            chargedness: None,
                        },
                        dt_soft,
                    },
                })
            }
        },
        // Only fully charged, has to reload after charges are out
        (true, false, false) => full_r(
            ctx,
            calc,
            item_uid,
            active_duration,
            cooldown_duration,
            dt_soft_cd,
            full_count,
        ),
        // Only fully charged, but can cycle without charges
        (true, false, true) => match ctx
            .u_data
            .get_item_optional_reload(item_uid, sim_options.optional_reloads)
        {
            OptionalReload::OnEmpty => full_r(
                ctx,
                calc,
                item_uid,
                active_duration,
                cooldown_duration,
                dt_soft_cd,
                full_count,
            ),
            OptionalReload::Disabled => {
                let dt_soft = CycleDtSoft::try_new(cooldown_duration, dt_soft_cd, false);
                CycleSeq::LimInf(CSeqLimInf {
                    p1_data: CycleDataFull {
                        active: CycleActive {
                            duration: active_duration,
                            chargedness: Some(UnitInterval::ONE),
                        },
                        dt_soft,
                    },
                    p1_repeat_count: full_count,
                    p2_data: CycleDataFull {
                        active: CycleActive {
                            duration: active_duration,
                            chargedness: None,
                        },
                        dt_soft,
                    },
                })
            }
        },
        // Fully charged + partially charged + can't run uncharged
        (true, true, false) => both_r(
            ctx,
            calc,
            item_uid,
            active_duration,
            cooldown_duration,
            dt_soft_cd,
            full_count,
            charge_info.part_charged,
        ),
        // Fully charged + partially charged + can cycle uncharged
        (true, true, true) => {
            match ctx
                .u_data
                .get_item_optional_reload(item_uid, sim_options.optional_reloads)
            {
                OptionalReload::OnEmpty => both_r(
                    ctx,
                    calc,
                    item_uid,
                    active_duration,
                    cooldown_duration,
                    dt_soft_cd,
                    full_count,
                    charge_info.part_charged,
                ),
                OptionalReload::Disabled => {
                    let dt_soft = CycleDtSoft::try_new(cooldown_duration, dt_soft_cd, false);
                    CycleSeq::LimSinInf(CSeqLimSinInf {
                        p1_data: CycleDataFull {
                            active: CycleActive {
                                duration: active_duration,
                                chargedness: Some(UnitInterval::ONE),
                            },
                            dt_soft,
                        },
                        p1_repeat_count: full_count,
                        p2_data: CycleDataFull {
                            active: CycleActive {
                                duration: active_duration,
                                chargedness: charge_info.part_charged,
                            },
                            dt_soft,
                        },
                        p3_data: CycleDataFull {
                            active: CycleActive {
                                duration: active_duration,
                                chargedness: None,
                            },
                            dt_soft,
                        },
                    })
                }
            }
        }
    };
    reuse_cseq_map.insert(effect_rid, cseq);
}

fn get_reload_duration(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> PValue {
    // All reloads can't take less than server tick realistically. E.g. lasers have almost 0 reload
    // duration but take 1-2 seconds to reload in EVE
    PValue::from_f64_unchecked(SERVER_TICK_S).max_value(
        calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().reload_time, Value::ZERO)
            .unwrap()
            / Value::THOUSAND,
    )
}

fn part_r(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    active_duration: PValue,
    cooldown_duration: PValue,
    dt_soft_cd: bool,
    chargedness: Option<UnitInterval>,
) -> CycleSeq<CycleDataFull> {
    CycleSeq::Inf(CSeqInf {
        data: CycleDataFull {
            active: CycleActive {
                duration: active_duration,
                chargedness,
            },
            dt_soft: CycleDtSoft::try_new(
                get_reload_duration(ctx, calc, item_uid).max(cooldown_duration),
                dt_soft_cd,
                true,
            ),
        },
        dt_hard: None,
    })
}

fn full_r(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    active_duration: PValue,
    cooldown_duration: PValue,
    dt_soft_cd: bool,
    full_count: Count,
) -> CycleSeq<CycleDataFull> {
    match full_count {
        Count::ONE => CycleSeq::Inf(CSeqInf {
            data: CycleDataFull {
                active: CycleActive {
                    duration: active_duration,
                    chargedness: Some(UnitInterval::ONE),
                },
                dt_soft: CycleDtSoft::try_new(
                    get_reload_duration(ctx, calc, item_uid).max(cooldown_duration),
                    dt_soft_cd,
                    true,
                ),
            },
            dt_hard: None,
        }),
        _ => CycleSeq::LoopLimSin(CSeqLoopLimSin {
            p1_data: CycleDataFull {
                active: CycleActive {
                    duration: active_duration,
                    chargedness: Some(UnitInterval::ONE),
                },
                dt_soft: CycleDtSoft::try_new(cooldown_duration, dt_soft_cd, false),
            },
            p1_repeat_count: full_count - Count::ONE,
            p2_data: CycleDataFull {
                active: CycleActive {
                    duration: active_duration,
                    chargedness: Some(UnitInterval::ONE),
                },
                dt_soft: CycleDtSoft::try_new(
                    get_reload_duration(ctx, calc, item_uid).max(cooldown_duration),
                    dt_soft_cd,
                    true,
                ),
            },
            dt_hard: None,
        }),
    }
}

fn both_r(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    active_duration: PValue,
    cooldown_duration: PValue,
    dt_soft_cd: bool,
    full_count: Count,
    chargedness: Option<UnitInterval>,
) -> CycleSeq<CycleDataFull> {
    CycleSeq::LoopLimSin(CSeqLoopLimSin {
        p1_data: CycleDataFull {
            active: CycleActive {
                duration: active_duration,
                chargedness: Some(UnitInterval::ONE),
            },
            dt_soft: CycleDtSoft::try_new(cooldown_duration, dt_soft_cd, false),
        },
        p1_repeat_count: full_count,
        p2_data: CycleDataFull {
            active: CycleActive {
                duration: active_duration,
                chargedness,
            },
            dt_soft: CycleDtSoft::try_new(
                get_reload_duration(ctx, calc, item_uid).max(cooldown_duration),
                dt_soft_cd,
                true,
            ),
        },
        dt_hard: None,
    })
}
