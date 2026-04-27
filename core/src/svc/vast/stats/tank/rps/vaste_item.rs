use super::stat::{StatRps, StatRpsLayer, StatRpsLayerRegen};
use crate::{
    nd::NEffectGeneralOutputGetter,
    num::{PValue, UnitInterval},
    rd::{REffectId, REffectLocalOpcSpec, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CseqMap, CyclingOptions, get_item_cseq_map},
        err::StatItemCheckError,
        vast::{
            StatTimeOptions, Vast,
            aggr::{
                SeqAccum, aggr_local_first, aggr_local_looped, aggr_local_time, aggr_proj_first, aggr_proj_looped,
                aggr_proj_time,
            },
            stats::{item_checks::check_drone_fighter_ship, shared::calc_regen_for_attrs},
        },
    },
    ud::{UItem, UItemId},
    util::{RMapRMap, RMapRMapRMap},
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_rps(
        &self,
        reuse_cseq_map: &mut CseqMap,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        shield_perc: UnitInterval,
    ) -> Result<StatRps, StatItemCheckError> {
        let item = check_drone_fighter_ship(ctx.u_data, item_uid)?;
        Ok(self.get_stat_item_rps_unchecked(reuse_cseq_map, ctx, calc, item_uid, item, time_options, shield_perc))
    }
    pub(in crate::svc::vast::stats::tank) fn get_stat_item_rps_unchecked(
        &self,
        reuse_cseq_map: &mut CseqMap,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        item: &UItem,
        time_options: StatTimeOptions,
        shield_perc: UnitInterval,
    ) -> StatRps {
        // Local reps
        let (local_shield, local_armor, local_hull) = match item {
            UItem::Ship(u_ship) => {
                let fit_data = self.get_fit_data(&u_ship.get_fit_uid());
                let local_shield = get_local_rps(reuse_cseq_map, ctx, calc, time_options, &fit_data.lr_shield);
                let local_armor = get_local_rps(reuse_cseq_map, ctx, calc, time_options, &fit_data.lr_armor);
                let local_hull = get_local_rps(reuse_cseq_map, ctx, calc, time_options, &fit_data.lr_hull);
                (local_shield, local_armor, local_hull)
            }
            _ => (PValue::ZERO, PValue::ZERO, PValue::ZERO),
        };
        // Incoming remote reps - shield
        let mut reuse_irr_entries = Vec::new();
        get_irr_data(
            reuse_cseq_map,
            &mut reuse_irr_entries,
            ctx,
            calc,
            item_uid,
            time_options,
            &self.irr_shield,
        );
        let remote_shield = irr_data_to_raw(&reuse_irr_entries);
        let remote_shield_penalized = irr_data_to_penalized(&reuse_irr_entries);
        // Incoming remote reps - armor
        reuse_irr_entries.clear();
        get_irr_data(
            reuse_cseq_map,
            &mut reuse_irr_entries,
            ctx,
            calc,
            item_uid,
            time_options,
            &self.irr_armor,
        );
        let remote_armor = irr_data_to_raw(&reuse_irr_entries);
        let remote_armor_penalized = irr_data_to_penalized(&reuse_irr_entries);
        // Incoming remote reps - hull
        reuse_irr_entries.clear();
        get_irr_data(
            reuse_cseq_map,
            &mut reuse_irr_entries,
            ctx,
            calc,
            item_uid,
            time_options,
            &self.irr_hull,
        );
        let remote_hull = irr_data_to_raw(&reuse_irr_entries);
        let remote_hull_penalized = irr_data_to_penalized(&reuse_irr_entries);
        // Regen
        let shield_regen = get_shield_regen(ctx, calc, item_uid, shield_perc);
        StatRps {
            shield: StatRpsLayerRegen {
                local: local_shield,
                remote: remote_shield,
                remote_penalized: remote_shield_penalized,
                regen: shield_regen,
            },
            armor: StatRpsLayer {
                local: local_armor,
                remote: remote_armor,
                remote_penalized: remote_armor_penalized,
            },
            hull: StatRpsLayer {
                local: local_hull,
                remote: remote_hull,
                remote_penalized: remote_hull_penalized,
            },
        }
    }
}

fn get_local_rps(
    reuse_cseq_map: &mut CseqMap,
    ctx: SvcCtx,
    calc: &mut Calc,
    time_options: StatTimeOptions,
    lrr_data: &RMapRMap<UItemId, REffectId, REffectLocalOpcSpec<NEffectGeneralOutputGetter>>,
) -> PValue {
    let mut total_rps = PValue::ZERO;
    let cycling_options = CyclingOptions::from_time_options(time_options);
    for (&item_uid, item_data) in lrr_data.iter() {
        if !get_item_cseq_map(reuse_cseq_map, ctx, calc, item_uid, cycling_options) {
            continue;
        };
        for (&effect_rid, ospec) in item_data.iter() {
            let cseq = match reuse_cseq_map.get(&effect_rid) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            let mut accum = SeqAccum::new_stack();
            if match time_options {
                StatTimeOptions::Burst(_) => aggr_local_first(ctx, calc, item_uid, effect, cseq, ospec, (), &mut accum),
                StatTimeOptions::Sim(sim_options) => match sim_options.time {
                    Some(time) if time > PValue::ZERO => {
                        aggr_local_time(ctx, calc, item_uid, effect, cseq, ospec, (), &mut accum, time)
                    }
                    _ => aggr_local_looped(ctx, calc, item_uid, effect, cseq, ospec, (), &mut accum),
                },
            } {
                total_rps += accum.get_per_second();
            }
        }
    }
    total_rps
}

struct IrrEntry {
    amount: PValue,
    cycle_duration: PValue,
}

fn get_irr_data(
    reuse_cseq_map: &mut CseqMap,
    reuse_result: &mut Vec<IrrEntry>,
    ctx: SvcCtx,
    calc: &mut Calc,
    projectee_item_uid: UItemId,
    time_options: StatTimeOptions,
    irr_data: &RMapRMapRMap<UItemId, UItemId, REffectId, REffectProjOpcSpec<NEffectGeneralOutputGetter>>,
) {
    let incoming_reps = match irr_data.get_l1(&projectee_item_uid) {
        Some(incoming_reps) => incoming_reps,
        None => return,
    };
    let cycling_options = CyclingOptions::from_time_options(time_options);
    for (&projector_item_uid, projector_data) in incoming_reps.iter() {
        if !get_item_cseq_map(reuse_cseq_map, ctx, calc, projector_item_uid, cycling_options) {
            continue;
        }
        for (&effect_rid, ospec) in projector_data.iter() {
            let cseq = match reuse_cseq_map.get(&effect_rid) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            let mut accum = SeqAccum::new_stack();
            match time_options {
                StatTimeOptions::Burst(burst_opts) => {
                    if aggr_proj_first(
                        ctx,
                        calc,
                        projector_item_uid,
                        effect,
                        cseq,
                        ospec,
                        (),
                        Some(projectee_item_uid),
                        burst_opts.spool,
                        &mut accum,
                    ) {
                        reuse_result.push(IrrEntry {
                            amount: accum.instances.stacked,
                            cycle_duration: accum.time,
                        });
                    }
                }
                StatTimeOptions::Sim(sim_options) => match sim_options.time {
                    Some(time) if time > PValue::ZERO => {
                        if aggr_proj_time(
                            ctx,
                            calc,
                            projector_item_uid,
                            effect,
                            cseq,
                            ospec,
                            (),
                            Some(projectee_item_uid),
                            &mut accum,
                            time,
                        ) {
                            // Adjust averaged reps per second to initial cycle duration to for
                            // purposes of RR stacking penalty calculation. This does not provide
                            // accurate result, but is likely to be a good enough approximation.
                            let first_cycle_full_duration = cseq.get_first_cycle().get_full_duration();
                            reuse_result.push(IrrEntry {
                                amount: accum.get_per_second() * first_cycle_full_duration,
                                cycle_duration: first_cycle_full_duration,
                            });
                        }
                    }
                    _ => {
                        if aggr_proj_looped(
                            ctx,
                            calc,
                            projector_item_uid,
                            effect,
                            cseq,
                            ospec,
                            (),
                            Some(projectee_item_uid),
                            &mut accum,
                        ) {
                            // Adjust averaged reps per second to initial cycle duration to for
                            // purposes of RR stacking penalty calculation. This does not provide
                            // accurate result, but is likely to be a good enough approximation.
                            let first_cycle_duration = match cseq.try_loop_cseq() {
                                Some(cseq_looped) => cseq_looped.get_first_cycle().get_full_duration(),
                                None => cseq.get_first_cycle().get_full_duration(),
                            };
                            reuse_result.push(IrrEntry {
                                amount: accum.get_per_second() * first_cycle_duration,
                                cycle_duration: first_cycle_duration,
                            });
                        }
                    }
                },
            }
        }
    }
}

fn irr_data_to_raw(irr_data: &[IrrEntry]) -> PValue {
    irr_data.iter().filter_map(get_normal_rps).sum()
}

const RR_PEN_ADDITION: PValue = PValue::from_f64_clamped(7000.0);
const RR_PEN_MULTIPLIER: PValue = PValue::from_f64_clamped(20.0);

fn irr_data_to_penalized(irr_data: &[IrrEntry]) -> PValue {
    let total_adjusted_rps: PValue = irr_data.iter().filter_map(get_adjusted_rps).sum();
    let mut result = PValue::ZERO;
    for entry in irr_data.iter() {
        let adjusted_rps = match get_adjusted_rps(entry) {
            Some(adjusted_rps) => adjusted_rps,
            None => continue,
        };
        let modified_rps = adjusted_rps.mul_add(RR_PEN_MULTIPLIER, RR_PEN_ADDITION);
        let mult = PValue::from_value_clamped(
            PValue::ONE - (((adjusted_rps + modified_rps) / (total_adjusted_rps + modified_rps)) - PValue::ONE).pow2(),
        );
        // Truncated cycle duration is used only for multiplier
        result += mult * get_normal_rps(entry).unwrap();
    }
    result
}
fn get_normal_rps(entry: &IrrEntry) -> Option<PValue> {
    let rps = entry.amount / entry.cycle_duration;
    match rps.is_finite() {
        true => Some(rps),
        false => None,
    }
}
fn get_adjusted_rps(entry: &IrrEntry) -> Option<PValue> {
    // For considerations of RR diminishing returns multiplier, cycle duration is rounded this way
    let main = entry.amount / entry.cycle_duration.floor_unerr();
    match main.is_finite() {
        true => Some(main),
        // Fallback variants do not exist in the original formula, but provided here just in case
        // some users set cycle duration to values below 1 second
        false => get_normal_rps(entry),
    }
}

fn get_shield_regen(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId, shield_perc: UnitInterval) -> PValue {
    calc_regen_for_attrs(
        ctx,
        calc,
        item_uid,
        ctx.ac().shield_capacity,
        ctx.ac().shield_recharge_rate,
        shield_perc,
    )
}
