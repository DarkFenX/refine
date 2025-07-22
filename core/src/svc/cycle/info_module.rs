use super::{
    info::{Cycle, CycleInner, CycleReload1, CycleReload2, CycleSimple},
    info_shared::{CycleOptionReload, CycleOptions, SelfKillerInfo},
    until_reload::{get_autocharge_cycle_count, get_charge_rate_cycle_count, get_crystal_cycle_count},
};
use crate::{
    ac, ad,
    def::OF,
    nd::{NEffectChargeDepl, NEffectChargeLoc},
    svc::{SvcCtx, calc::Calc, efuncs},
    uad::{UadItem, UadItemKey, UadModule},
    util::{InfCount, RMap},
};

pub(super) fn get_module_cycle_info(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UadItemKey,
    uad_item: &UadItem,
    uad_module: &UadModule,
    options: CycleOptions,
    ignore_state: bool,
) -> Option<RMap<ad::AEffectId, Cycle>> {
    if !uad_module.is_loaded() {
        return None;
    };
    let mut cycle_infos = RMap::new();
    let mut self_killers = Vec::new();
    match ignore_state {
        true => {
            for &a_effect_id in uad_module.get_a_effect_datas().unwrap().keys() {
                fill_module_effect_info(
                    &mut cycle_infos,
                    &mut self_killers,
                    ctx,
                    calc,
                    item_key,
                    uad_item,
                    uad_module,
                    a_effect_id,
                    options,
                );
            }
        }
        false => {
            for &a_effect_id in uad_module.get_reffs().unwrap().iter() {
                fill_module_effect_info(
                    &mut cycle_infos,
                    &mut self_killers,
                    ctx,
                    calc,
                    item_key,
                    uad_item,
                    uad_module,
                    a_effect_id,
                    options,
                );
            }
        }
    }
    // If there are any self-killer effects, choose the fastest one, and discard all other effects
    if !self_killers.is_empty() {
        let fastest_sk_a_effect_id = self_killers
            .into_iter()
            .min_by_key(|sk_info| sk_info.duration_s)
            .unwrap()
            .a_effect_id;
        cycle_infos.retain(|&k, _| k == fastest_sk_a_effect_id);
    }
    Some(cycle_infos)
}

fn fill_module_effect_info(
    cycle_infos: &mut RMap<ad::AEffectId, Cycle>,
    self_killers: &mut Vec<SelfKillerInfo>,
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UadItemKey,
    uad_item: &UadItem,
    uad_module: &UadModule,
    a_effect_id: ad::AEffectId,
    options: CycleOptions,
) {
    let a_effect = match ctx.uad.src.get_a_effect(&a_effect_id) {
        Some(a_effect) => a_effect,
        None => return,
    };
    if !a_effect.xt.is_active {
        return;
    }
    // No appropriate duration - no info
    let duration_s = match efuncs::get_effect_duration_s(ctx, calc, item_key, a_effect) {
        Some(duration_s) => duration_s,
        None => return,
    };
    // Charge count info
    let cycle_count = match a_effect.hc.charge {
        Some(n_charge) => match n_charge.location {
            NEffectChargeLoc::Autocharge(_) => get_autocharge_cycle_count(uad_item, a_effect),
            NEffectChargeLoc::Loaded(charge_depletion) => match charge_depletion {
                NEffectChargeDepl::ChargeRate { can_run_uncharged } => {
                    get_charge_rate_cycle_count(ctx, uad_module, can_run_uncharged, options.reload_optionals)
                }
                NEffectChargeDepl::Crystal => get_crystal_cycle_count(ctx, uad_module),
                NEffectChargeDepl::None => InfCount::Infinite,
            },
        },
        None => InfCount::Infinite,
    };
    // Completely skip effects which can't cycle
    if cycle_count == InfCount::Count(0) {
        return;
    }
    // Self-killers are fairly trivial. Record info about them and go to next effect
    if a_effect.hc.kills_item {
        self_killers.push(SelfKillerInfo {
            a_effect_id,
            duration_s,
        });
        cycle_infos.insert(
            a_effect_id,
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
                        // If reload time is defined, ensure it is
                        true => reload_time_s.max(OF(1.0)),
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
                    cycle_infos.insert(a_effect_id, Cycle::Reload1(CycleReload1 { inner }));
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
                        a_effect_id,
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
                a_effect_id,
                Cycle::Simple(CycleSimple {
                    active_time: duration_s,
                    inactive_time: reactivation_delay_s,
                    repeat_count: InfCount::Infinite,
                }),
            );
        }
    }
}
