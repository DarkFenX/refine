use super::{
    info::{CycleComplex, CycleInfo, CycleInner, CycleSimple},
    info_shared::{CycleOptions, SelfKillerInfo},
    until_reload::{get_autocharge_cycle_count, get_charge_rate_cycle_count, get_crystal_cycle_count},
};
use crate::{
    ac, ad,
    def::{ItemKey, OF},
    nd::{NEffectCharge, NEffectChargeDepl},
    sol::REffs,
    svc::{SvcCtx, calc::Calc, efuncs},
    uad::{UadItem, UadModule},
    util::{InfCount, RMap},
};

pub(super) fn get_module_cycle_info(
    ctx: SvcCtx,
    reffs: &REffs,
    calc: &mut Calc,
    item_key: ItemKey,
    uad_item: &UadItem,
    uad_module: &UadModule,
    options: CycleOptions,
    ignore_state: bool,
) -> Option<RMap<ad::AEffectId, CycleInfo>> {
    if !uad_module.is_loaded() {
        return None;
    };
    let mut cycle_infos = RMap::new();
    let mut self_killers = Vec::new();
    for &a_effect_id in reffs.iter_running(&item_key) {
        let a_effect = ctx.uad.src.get_a_effect(&a_effect_id).unwrap();
        if !a_effect.xt.is_active {
            continue;
        }
        // No appropriate duration - no info
        let duration_s = match efuncs::get_effect_duration_s(ctx, calc, item_key, a_effect) {
            Some(duration_s) => duration_s,
            None => continue,
        };
        // Charge count info
        let cycle_count = match a_effect.hc.charge {
            Some(n_charge) => match n_charge {
                NEffectCharge::Autocharge(_) => get_autocharge_cycle_count(uad_item, a_effect),
                NEffectCharge::Loaded(charge_depletion) => match charge_depletion {
                    NEffectChargeDepl::ChargeRate => get_charge_rate_cycle_count(ctx, uad_module),
                    NEffectChargeDepl::Crystal => get_crystal_cycle_count(ctx, uad_module),
                    NEffectChargeDepl::None => InfCount::Infinite,
                },
            },
            None => InfCount::Infinite,
        };
        // Completely skip effects which can't cycle
        if cycle_count == InfCount::Count(0) {
            continue;
        }
        // Self-killers are fairly trivial. Record info about them and go to next effect
        if a_effect.hc.kills_item {
            self_killers.push(SelfKillerInfo {
                a_effect_id,
                duration_s,
            });
            cycle_infos.insert(
                a_effect_id,
                CycleInfo::Simple(CycleSimple {
                    active_time: duration_s,
                    inactive_time: OF(0.0),
                    repeat_count: InfCount::Count(1),
                    reload: false,
                }),
            );
            continue;
        }
        let reactivation_delay_s = (calc
            .get_item_attr_val_extra(ctx, item_key, &ac::attrs::MOD_REACTIVATION_DELAY)
            .unwrap()
            / 1000.0)
            .max(OF(0.0));
        match cycle_count {
            // When we have to handle reload, result is a bit complex
            InfCount::Count(count_until_reload) => {
                let reload_time_s = match options {
                    // When considering burst calculations, just set reload to 0
                    CycleOptions::Burst => OF(0.0),
                    CycleOptions::Sim => {
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
                    // When module can do only one cycle per clip - return simple infinitely
                    // repeating cycle, with reload on each repeat
                    0 => {
                        cycle_infos.insert(
                            a_effect_id,
                            CycleInfo::Simple(CycleSimple {
                                active_time: duration_s,
                                inactive_time: final_inactive_time,
                                repeat_count: InfCount::Infinite,
                                reload: true,
                            }),
                        );
                    }
                    // When it does more than one cycle per clip - mark final cycle as reload
                    _ => {
                        let simple_early = CycleInner {
                            active_time: duration_s,
                            inactive_time: reactivation_delay_s,
                            repeat_count: early_cycle_count,
                            reload: false,
                        };
                        let simple_final = CycleInner {
                            active_time: duration_s,
                            inactive_time: final_inactive_time,
                            repeat_count: final_cycle_count,
                            reload: true,
                        };
                        cycle_infos.insert(
                            a_effect_id,
                            CycleInfo::Complex(CycleComplex {
                                inner1: simple_early,
                                inner2: simple_final,
                                repeat_count: InfCount::Infinite,
                            }),
                        );
                    }
                }
            }
            // Infinite charges until reload - return simple infinitely repeating cycle
            InfCount::Infinite => {
                cycle_infos.insert(
                    a_effect_id,
                    CycleInfo::Simple(CycleSimple {
                        active_time: duration_s,
                        inactive_time: reactivation_delay_s,
                        repeat_count: InfCount::Infinite,
                        reload: false,
                    }),
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
