use super::{
    map::CseqMap,
    shared::{CyclingOptions, SelfKillerEffectInfo, SelfKillerItemInfo},
};
use crate::{
    misc::{InfCount, OptionalReload},
    nd::{NEffectChargeDepl, NEffectChargeDeplCrystal},
    num::{Count, PValue, UnitInterval, Value},
    rd::{REffectChargeLoc, REffectId},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{
            CSeqHardDtFull, CycleActive, CycleDataFull, CycleSeq, CycleSoftDtFull, CycleSoftDtReasons,
            effect_charge_info::{
                get_eci_autocharge, get_eci_charge_rate, get_eci_crystal, get_eci_uncharged, get_eci_undepletable,
            },
            seq_var_lim::CSeqLim,
            seq_var_lim_inf::CSeqLimInf,
            seq_var_lim_sin_inf::CSeqLimSinInf,
            seq_var_loop_lim_sin::CSeqLoopLimSin,
            seq_var_loop_sin::CSeqLoopSin,
        },
        funcs::{get_effect_duration_s, is_oattr_flag_set},
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
    let Some(active_duration) = get_effect_duration_s(ctx, calc, item_uid, effect) else {
        return;
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
                    soft_dt: None,
                },
                repeat_count: Count::ONE,
            }),
        );
        return;
    }
    let sim_options = match options {
        CyclingOptions::Sim(sim_options) => sim_options,
        // If burst cycle mode was requested, just assume first cycle is the "most charged", and
        // infinitely repeat it
        CyclingOptions::Burst => {
            reuse_cseq_map.insert(
                effect_rid,
                CycleSeq::LoopSin(CSeqLoopSin {
                    data: CycleDataFull {
                        active: CycleActive {
                            duration: active_duration,
                            chargedness: charge_info.get_first_cycle_chargedness(),
                        },
                        soft_dt: CycleSoftDtFull::try_new_for_module_regular(ctx, calc, item_uid, module),
                    },
                    hard_dt: None,
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
                CycleSeq::LoopSin(CSeqLoopSin {
                    data: CycleDataFull {
                        active: CycleActive {
                            duration: active_duration,
                            chargedness: Some(UnitInterval::ONE),
                        },
                        soft_dt: CycleSoftDtFull::try_new_for_module_regular(ctx, calc, item_uid, module),
                    },
                    hard_dt: None,
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
        (false, false, true) => CycleSeq::LoopSin(CSeqLoopSin {
            data: CycleDataFull {
                active: CycleActive {
                    duration: active_duration,
                    chargedness: None,
                },
                soft_dt: CycleSoftDtFull::try_new_for_module_regular(ctx, calc, item_uid, module),
            },
            hard_dt: None,
        }),
        // Only partially charged, has to reload every cycle
        (false, true, false) => part_r(ctx, calc, item_uid, module, active_duration, charge_info.part_charged),
        // Only partially charged cycle, but can cycle without charges
        (false, true, true) => match ctx
            .u_data
            .get_item_optional_reload(item_uid, sim_options.optional_reloads)
        {
            OptionalReload::OnEmpty => part_r(ctx, calc, item_uid, module, active_duration, charge_info.part_charged),
            OptionalReload::Disabled => {
                let soft_dt = CycleSoftDtFull::try_new_for_module_regular(ctx, calc, item_uid, module);
                CycleSeq::LimInf(CSeqLimInf {
                    p1_data: CycleDataFull {
                        active: CycleActive {
                            duration: active_duration,
                            chargedness: charge_info.part_charged,
                        },
                        soft_dt,
                    },
                    p1_repeat_count: Count::ONE,
                    p2_data: CycleDataFull {
                        active: CycleActive {
                            duration: active_duration,
                            chargedness: None,
                        },
                        soft_dt,
                    },
                })
            }
        },
        // Only fully charged, has to reload after charges are out
        (true, false, false) => full_r(ctx, calc, item_uid, module, active_duration, full_count),
        // Only fully charged, but can cycle without charges
        (true, false, true) => match ctx
            .u_data
            .get_item_optional_reload(item_uid, sim_options.optional_reloads)
        {
            OptionalReload::OnEmpty => full_r(ctx, calc, item_uid, module, active_duration, full_count),
            OptionalReload::Disabled => {
                let soft_dt = CycleSoftDtFull::try_new_for_module_regular(ctx, calc, item_uid, module);
                CycleSeq::LimInf(CSeqLimInf {
                    p1_data: CycleDataFull {
                        active: CycleActive {
                            duration: active_duration,
                            chargedness: Some(UnitInterval::ONE),
                        },
                        soft_dt,
                    },
                    p1_repeat_count: full_count,
                    p2_data: CycleDataFull {
                        active: CycleActive {
                            duration: active_duration,
                            chargedness: None,
                        },
                        soft_dt,
                    },
                })
            }
        },
        // Fully charged + partially charged + can't run uncharged
        (true, true, false) => both_r(
            ctx,
            calc,
            item_uid,
            module,
            active_duration,
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
                    module,
                    active_duration,
                    full_count,
                    charge_info.part_charged,
                ),
                OptionalReload::Disabled => {
                    let soft_dt = CycleSoftDtFull::try_new_for_module_regular(ctx, calc, item_uid, module);
                    CycleSeq::LimSinInf(CSeqLimSinInf {
                        p1_data: CycleDataFull {
                            active: CycleActive {
                                duration: active_duration,
                                chargedness: Some(UnitInterval::ONE),
                            },
                            soft_dt,
                        },
                        p1_repeat_count: full_count,
                        p2_data: CycleDataFull {
                            active: CycleActive {
                                duration: active_duration,
                                chargedness: charge_info.part_charged,
                            },
                            soft_dt,
                        },
                        p3_data: CycleDataFull {
                            active: CycleActive {
                                duration: active_duration,
                                chargedness: None,
                            },
                            soft_dt,
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
    PValue::SERVER_TICK_S.max_value(
        calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().reload_time, Value::ZERO)
            .unwrap()
            / Value::THOUSAND,
    )
}

fn part_r(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    module: &UModule,
    active_duration: PValue,
    chargedness: Option<UnitInterval>,
) -> CycleSeq<CycleDataFull, CSeqHardDtFull> {
    CycleSeq::LoopSin(CSeqLoopSin {
        data: CycleDataFull {
            active: CycleActive {
                duration: active_duration,
                chargedness,
            },
            soft_dt: Some(CycleSoftDtFull::new_for_module_reload(ctx, calc, item_uid, module)),
        },
        hard_dt: None,
    })
}

fn full_r(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    module: &UModule,
    active_duration: PValue,
    full_count: Count,
) -> CycleSeq<CycleDataFull, CSeqHardDtFull> {
    match full_count {
        Count::ONE => CycleSeq::LoopSin(CSeqLoopSin {
            data: CycleDataFull {
                active: CycleActive {
                    duration: active_duration,
                    chargedness: Some(UnitInterval::ONE),
                },
                soft_dt: Some(CycleSoftDtFull::new_for_module_reload(ctx, calc, item_uid, module)),
            },
            hard_dt: None,
        }),
        _ => {
            let soft_dts = SoftDts::new_for_module(ctx, calc, item_uid, module);
            CycleSeq::LoopLimSin(CSeqLoopLimSin {
                p1_data: CycleDataFull {
                    active: CycleActive {
                        duration: active_duration,
                        chargedness: Some(UnitInterval::ONE),
                    },
                    soft_dt: soft_dts.regular,
                },
                p1_repeat_count: full_count - Count::ONE,
                p2_data: CycleDataFull {
                    active: CycleActive {
                        duration: active_duration,
                        chargedness: Some(UnitInterval::ONE),
                    },
                    soft_dt: Some(soft_dts.reload),
                },
                hard_dt: None,
            })
        }
    }
}

fn both_r(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    module: &UModule,
    active_duration: PValue,
    full_count: Count,
    chargedness: Option<UnitInterval>,
) -> CycleSeq<CycleDataFull, CSeqHardDtFull> {
    let soft_dts = SoftDts::new_for_module(ctx, calc, item_uid, module);
    CycleSeq::LoopLimSin(CSeqLoopLimSin {
        p1_data: CycleDataFull {
            active: CycleActive {
                duration: active_duration,
                chargedness: Some(UnitInterval::ONE),
            },
            soft_dt: soft_dts.regular,
        },
        p1_repeat_count: full_count,
        p2_data: CycleDataFull {
            active: CycleActive {
                duration: active_duration,
                chargedness,
            },
            soft_dt: Some(soft_dts.reload),
        },
        hard_dt: None,
    })
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Soft downtime constructors
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CycleSoftDtFull {
    fn try_new_for_module_regular(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId, module: &UModule) -> Option<Self> {
        let axt = module.get_axt().unwrap();
        // If auto-repeats are allowed - there should be no downtime for non-reload cycles (since
        // reactivation delay kicks in only when cycling stops, as seen on e.g. cynos/cloaks)
        if !(axt.specs_disallow_repeats
            && is_oattr_flag_set(ctx, calc, item_uid, ctx.ac().disallow_repeating_activation).unwrap_or(false))
        {
            return None;
        }
        // If auto-repeats are not allowed, but there is no reactivation delay - set downtime
        // duration to one tick. Tested on Singularity on 2026-06-14 by using direct DD on random
        // capitals (DD cycle duration 252 seconds, damage intervals were 253 seconds)
        let mut total_duration = PValue::SERVER_TICK_S;
        if !axt.specs_reactivation_delay {
            return Some(Self {
                duration: total_duration,
                reasons: CycleSoftDtReasons { reload: false },
            });
        }
        let reactivation_delay = PValue::from_value_clamped(
            calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().mod_reactivation_delay, Value::ZERO)
                .unwrap()
                / Value::THOUSAND,
        );
        if reactivation_delay > total_duration {
            total_duration = reactivation_delay;
        }
        Some(Self {
            duration: total_duration,
            reasons: CycleSoftDtReasons { reload: false },
        })
    }
    fn new_for_module_reload(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId, module: &UModule) -> Self {
        let axt = module.get_axt().unwrap();
        let mut total_duration = get_reload_duration(ctx, calc, item_uid);
        // When item reloads, reactivation delay always kicks in, if set
        if axt.specs_reactivation_delay {
            let reactivation_delay = PValue::from_value_clamped(
                calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().mod_reactivation_delay, Value::ZERO)
                    .unwrap()
                    / Value::THOUSAND,
            );
            if reactivation_delay > total_duration {
                total_duration = reactivation_delay;
            }
        }
        Self {
            duration: total_duration,
            reasons: CycleSoftDtReasons { reload: true },
        }
    }
}

struct SoftDts {
    regular: Option<CycleSoftDtFull>,
    reload: CycleSoftDtFull,
}
impl SoftDts {
    fn new_for_module(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId, module: &UModule) -> Self {
        let axt = module.get_axt().unwrap();
        let mut soft_dt_regular = None;
        let mut soft_dt_reload = CycleSoftDtFull {
            duration: get_reload_duration(ctx, calc, item_uid),
            reasons: CycleSoftDtReasons { reload: true },
        };
        if axt.specs_disallow_repeats
            && is_oattr_flag_set(ctx, calc, item_uid, ctx.ac().disallow_repeating_activation).unwrap_or(false)
        {
            soft_dt_regular = Some(CycleSoftDtFull {
                duration: PValue::SERVER_TICK_S,
                reasons: CycleSoftDtReasons { reload: false },
            });
        }
        if axt.specs_reactivation_delay {
            let reactivation_delay = PValue::from_value_clamped(
                calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().mod_reactivation_delay, Value::ZERO)
                    .unwrap()
                    / Value::THOUSAND,
            );
            if reactivation_delay > soft_dt_reload.duration {
                soft_dt_reload.duration = reactivation_delay;
            }
            if let Some(soft_dt_regular) = soft_dt_regular.as_mut()
                && reactivation_delay > soft_dt_regular.duration
            {
                soft_dt_regular.duration = reactivation_delay;
            }
        }
        Self {
            regular: soft_dt_regular,
            reload: soft_dt_reload,
        }
    }
}
