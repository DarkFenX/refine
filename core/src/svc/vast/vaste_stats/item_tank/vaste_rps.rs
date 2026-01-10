use crate::{
    num::{PValue, UnitInterval, Value},
    rd::{REffectId, REffectLocalOpcSpec, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        aggr::{
            aggr_local_first_ps, aggr_local_looped_ps, aggr_local_time_ps, aggr_proj_first_amount, aggr_proj_looped_ps,
            aggr_proj_time_ps,
        },
        calc::Calc,
        cycle::{CyclingOptions, get_item_cseq_map},
        err::StatItemCheckError,
        vast::{
            StatTankRegen, StatTimeOptions, Vast, shared::calc_regen,
            vaste_stats::item_checks::check_drone_fighter_ship,
        },
    },
    ud::{UItem, UItemId},
    util::{RMapRMap, RMapRMapRMap},
};

pub struct StatLayerRps {
    pub local: PValue,
    pub remote: PValue,
    pub remote_penalized: PValue,
}

pub struct StatLayerRpsRegen {
    pub local: PValue,
    pub remote: PValue,
    pub remote_penalized: PValue,
    pub regen: PValue,
}

impl Vast {
    pub(in crate::svc) fn get_stat_item_rps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        shield_perc: UnitInterval,
    ) -> Result<StatTankRegen<StatLayerRps, StatLayerRpsRegen>, StatItemCheckError> {
        let item = check_drone_fighter_ship(ctx.u_data, item_uid)?;
        Ok(self.get_stat_item_rps_unchecked(ctx, calc, item_uid, item, time_options, shield_perc))
    }
    pub(super) fn get_stat_item_rps_unchecked(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        item: &UItem,
        time_options: StatTimeOptions,
        shield_perc: UnitInterval,
    ) -> StatTankRegen<StatLayerRps, StatLayerRpsRegen> {
        // Local reps
        let (local_shield, local_armor, local_hull) = match item {
            UItem::Ship(u_ship) => {
                let fit_data = self.get_fit_data(&u_ship.get_fit_uid());
                let local_shield = get_local_rps(ctx, calc, time_options, &fit_data.lr_shield);
                let local_armor = get_local_rps(ctx, calc, time_options, &fit_data.lr_armor);
                let local_hull = get_local_rps(ctx, calc, time_options, &fit_data.lr_hull);
                (local_shield, local_armor, local_hull)
            }
            _ => (PValue::ZERO, PValue::ZERO, PValue::ZERO),
        };
        // Incoming remote reps
        let shield_irr_data = get_irr_data(ctx, calc, item_uid, time_options, &self.irr_shield);
        let armor_irr_data = get_irr_data(ctx, calc, item_uid, time_options, &self.irr_armor);
        let hull_irr_data = get_irr_data(ctx, calc, item_uid, time_options, &self.irr_hull);
        // Regen
        let shield_regen = get_shield_regen(ctx, calc, item_uid, shield_perc);
        StatTankRegen {
            shield: StatLayerRpsRegen {
                local: local_shield,
                remote: irr_data_to_raw(&shield_irr_data),
                remote_penalized: irr_data_to_penalized(shield_irr_data),
                regen: shield_regen,
            },
            armor: StatLayerRps {
                local: local_armor,
                remote: irr_data_to_raw(&armor_irr_data),
                remote_penalized: irr_data_to_penalized(armor_irr_data),
            },
            hull: StatLayerRps {
                local: local_hull,
                remote: irr_data_to_raw(&hull_irr_data),
                remote_penalized: irr_data_to_penalized(hull_irr_data),
            },
        }
    }
}

fn get_local_rps(
    ctx: SvcCtx,
    calc: &mut Calc,
    time_options: StatTimeOptions,
    lrr_data: &RMapRMap<UItemId, REffectId, REffectLocalOpcSpec<PValue>>,
) -> PValue {
    let mut total_rps = PValue::ZERO;
    let cycling_options = CyclingOptions::from_time_options(time_options);
    for (&item_uid, item_data) in lrr_data.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, item_uid, cycling_options, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        for (&effect_rid, ospec) in item_data.iter() {
            let cseq = match cseq_map.get(&effect_rid) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            match time_options {
                StatTimeOptions::Burst(_) => {
                    if let Some(effect_rps) = aggr_local_first_ps(ctx, calc, item_uid, effect, cseq, ospec) {
                        total_rps += effect_rps;
                    }
                }
                StatTimeOptions::Sim(sim_options) => match sim_options.time {
                    Some(time) if time > PValue::ZERO => {
                        if let Some(effect_rps) = aggr_local_time_ps(ctx, calc, item_uid, effect, cseq, ospec, time) {
                            total_rps += effect_rps;
                        }
                    }
                    _ => {
                        if let Some(effect_rps) = aggr_local_looped_ps(ctx, calc, item_uid, effect, cseq, ospec) {
                            total_rps += effect_rps;
                        }
                    }
                },
            }
        }
    }
    total_rps
}

struct IrrEntry {
    amount: PValue,
    cycle_time: PValue,
}

fn get_irr_data(
    ctx: SvcCtx,
    calc: &mut Calc,
    projectee_item_uid: UItemId,
    time_options: StatTimeOptions,
    irr_data: &RMapRMapRMap<UItemId, UItemId, REffectId, REffectProjOpcSpec<PValue>>,
) -> Vec<IrrEntry> {
    let mut result = Vec::new();
    let incoming_reps = match irr_data.get_l1(&projectee_item_uid) {
        Some(incoming_reps) => incoming_reps,
        None => return result,
    };
    let cycling_options = CyclingOptions::from_time_options(time_options);
    for (&projector_item_uid, projector_data) in incoming_reps.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, projector_item_uid, cycling_options, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        for (&effect_rid, ospec) in projector_data.iter() {
            let cseq = match cseq_map.get(&effect_rid) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            match time_options {
                StatTimeOptions::Burst(burst_opts) => {
                    if let Some(effect_rep) = aggr_proj_first_amount(
                        ctx,
                        calc,
                        projector_item_uid,
                        effect,
                        cseq,
                        ospec,
                        Some(projectee_item_uid),
                        burst_opts.spool,
                    ) {
                        result.push(IrrEntry {
                            amount: effect_rep.amount,
                            cycle_time: effect_rep.time,
                        });
                    }
                }
                StatTimeOptions::Sim(sim_options) => match sim_options.time {
                    Some(time) if time > PValue::ZERO => {
                        if let Some(effect_rps) = aggr_proj_time_ps(
                            ctx,
                            calc,
                            projector_item_uid,
                            effect,
                            cseq,
                            ospec,
                            Some(projectee_item_uid),
                            time,
                        ) {
                            // Adjust averaged reps per second to initial cycle duration to for
                            // purposes of RR stacking penalty calculation. This does not provide
                            // accurate result, but is likely to be a good enough approximation.
                            let first_cycle_duration = cseq.get_first_cycle().time;
                            result.push(IrrEntry {
                                amount: effect_rps * first_cycle_duration,
                                cycle_time: first_cycle_duration,
                            });
                        }
                    }
                    _ => {
                        if let Some(effect_rps) = aggr_proj_looped_ps(
                            ctx,
                            calc,
                            projector_item_uid,
                            effect,
                            cseq,
                            ospec,
                            Some(projectee_item_uid),
                        ) {
                            // Adjust averaged reps per second to initial cycle duration to for
                            // purposes of RR stacking penalty calculation. This does not provide
                            // accurate result, but is likely to be a good enough approximation.
                            let first_cycle_duration = cseq.get_first_cycle().time;
                            result.push(IrrEntry {
                                amount: effect_rps * first_cycle_duration,
                                cycle_time: first_cycle_duration,
                            });
                        }
                    }
                },
            }
        }
    }
    result
}

fn irr_data_to_raw(irr_data: &[IrrEntry]) -> PValue {
    irr_data.iter().filter_map(|v| get_normal_rps(v)).sum()
}

const RR_PEN_ADDITION: PValue = PValue::from_f64_clamped(7000.0);
const RR_PEN_MULTIPLIER: PValue = PValue::from_f64_clamped(20.0);

fn irr_data_to_penalized(irr_data: Vec<IrrEntry>) -> PValue {
    let total_adjusted_rps: PValue = irr_data.iter().filter_map(|v| get_adjusted_rps(v)).sum();
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
        // Truncated cycle time is used only for multiplier
        result += mult * get_normal_rps(entry).unwrap();
    }
    result
}
fn get_normal_rps(entry: &IrrEntry) -> Option<PValue> {
    let rps = entry.amount / entry.cycle_time;
    match rps.is_finite() {
        true => Some(rps),
        false => None,
    }
}
fn get_adjusted_rps(entry: &IrrEntry) -> Option<PValue> {
    // For considerations of RR diminishing returns multiplier, cycle time is rounded this way
    let main = entry.amount / entry.cycle_time.floor_unerr();
    match main.is_finite() {
        true => Some(main),
        // Fallback variants do not exist in the original formula, but provided here just in case
        // some users set cycle time to values below 1 second
        false => get_normal_rps(entry),
    }
}

fn get_shield_regen(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId, shield_perc: UnitInterval) -> PValue {
    let attr_consts = ctx.ac();
    let shield_hp = PValue::from_value_clamped(
        calc.get_item_oattr_afb_oextra(ctx, item_uid, attr_consts.shield_capacity, Value::ZERO)
            .unwrap(),
    );
    let shield_regen_time = PValue::from_value_clamped(
        calc.get_item_oattr_afb_oextra(ctx, item_uid, attr_consts.shield_recharge_rate, Value::ZERO)
            .unwrap()
            / Value::THOUSAND,
    );
    calc_regen(shield_hp, shield_regen_time, shield_perc)
}
