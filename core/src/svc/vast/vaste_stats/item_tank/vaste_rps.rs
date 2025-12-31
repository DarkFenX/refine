use crate::{
    def::{AttrVal, OF},
    rd::{REffectKey, REffectLocalOpcSpec, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        aggr::{
            aggr_local_first_ps, aggr_local_looped_ps, aggr_local_time_ps, aggr_proj_first_amount, aggr_proj_looped_ps,
            aggr_proj_time_ps,
        },
        calc::Calc,
        cycle::get_item_cseq_map,
        err::StatItemCheckError,
        vast::{
            StatTankRegen, StatTimeOptions, Vast, shared::calc_regen,
            vaste_stats::item_checks::check_drone_fighter_ship,
        },
    },
    ud::{UItem, UItemKey},
    util::{RMapRMap, RMapRMapRMap, UnitInterval, trunc_unerr},
};

pub struct StatLayerRps {
    pub local: AttrVal,
    pub remote: AttrVal,
    pub remote_penalized: AttrVal,
}

pub struct StatLayerRpsRegen {
    pub local: AttrVal,
    pub remote: AttrVal,
    pub remote_penalized: AttrVal,
    pub regen: AttrVal,
}

impl Vast {
    pub(in crate::svc) fn get_stat_item_rps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        time_options: StatTimeOptions,
        shield_perc: UnitInterval,
    ) -> Result<StatTankRegen<StatLayerRps, StatLayerRpsRegen>, StatItemCheckError> {
        let item = check_drone_fighter_ship(ctx.u_data, item_key)?;
        Ok(self.get_stat_item_rps_unchecked(ctx, calc, item_key, item, time_options, shield_perc))
    }
    pub(super) fn get_stat_item_rps_unchecked(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        item: &UItem,
        time_options: StatTimeOptions,
        shield_perc: UnitInterval,
    ) -> StatTankRegen<StatLayerRps, StatLayerRpsRegen> {
        // Local reps
        let (local_shield, local_armor, local_hull) = match item {
            UItem::Ship(u_ship) => {
                let fit_data = self.get_fit_data(&u_ship.get_fit_key());
                let local_shield = get_local_rps(ctx, calc, time_options, &fit_data.lr_shield);
                let local_armor = get_local_rps(ctx, calc, time_options, &fit_data.lr_armor);
                let local_hull = get_local_rps(ctx, calc, time_options, &fit_data.lr_hull);
                (local_shield, local_armor, local_hull)
            }
            _ => (OF(0.0), OF(0.0), OF(0.0)),
        };
        // Incoming remote reps
        let shield_irr_data = get_irr_data(ctx, calc, item_key, time_options, &self.irr_shield);
        let armor_irr_data = get_irr_data(ctx, calc, item_key, time_options, &self.irr_armor);
        let hull_irr_data = get_irr_data(ctx, calc, item_key, time_options, &self.irr_hull);
        // Regen
        let shield_regen = get_shield_regen(ctx, calc, item_key, shield_perc);
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
    lrr_data: &RMapRMap<UItemKey, REffectKey, REffectLocalOpcSpec<AttrVal>>,
) -> AttrVal {
    let mut total_rps = OF(0.0);
    let cycling_options = time_options.into();
    for (&item_key, item_data) in lrr_data.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, item_key, cycling_options, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        for (&effect_key, ospec) in item_data.iter() {
            let cseq = match cseq_map.get(&effect_key) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect(effect_key);
            match time_options {
                StatTimeOptions::Burst(_) => {
                    if let Some(effect_rps) = aggr_local_first_ps(ctx, calc, item_key, effect, cseq, ospec) {
                        total_rps += effect_rps;
                    }
                }
                StatTimeOptions::Sim(sim_options) => match sim_options.time {
                    Some(time) if time > OF(0.0) => {
                        if let Some(effect_rps) = aggr_local_time_ps(ctx, calc, item_key, effect, cseq, ospec, time) {
                            total_rps += effect_rps;
                        }
                    }
                    _ => {
                        if let Some(effect_rps) = aggr_local_looped_ps(ctx, calc, item_key, effect, cseq, ospec) {
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
    amount: AttrVal,
    cycle_time: AttrVal,
}

fn get_irr_data(
    ctx: SvcCtx,
    calc: &mut Calc,
    projectee_item_key: UItemKey,
    time_options: StatTimeOptions,
    irr_data: &RMapRMapRMap<UItemKey, UItemKey, REffectKey, REffectProjOpcSpec<AttrVal>>,
) -> Vec<IrrEntry> {
    let mut result = Vec::new();
    let incoming_reps = match irr_data.get_l1(&projectee_item_key) {
        Some(incoming_reps) => incoming_reps,
        None => return result,
    };
    let cycling_options = time_options.into();
    for (&projector_item_key, projector_data) in incoming_reps.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, projector_item_key, cycling_options, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        for (&effect_key, ospec) in projector_data.iter() {
            let cseq = match cseq_map.get(&effect_key) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect(effect_key);
            match time_options {
                StatTimeOptions::Burst(burst_opts) => {
                    if let Some(effect_rep) = aggr_proj_first_amount(
                        ctx,
                        calc,
                        projector_item_key,
                        effect,
                        cseq,
                        ospec,
                        Some(projectee_item_key),
                        burst_opts.spool,
                    ) {
                        result.push(IrrEntry {
                            amount: effect_rep.amount,
                            cycle_time: effect_rep.time,
                        });
                    }
                }
                StatTimeOptions::Sim(sim_options) => match sim_options.time {
                    Some(time) if time > OF(0.0) => {
                        if let Some(effect_rps) = aggr_proj_time_ps(
                            ctx,
                            calc,
                            projector_item_key,
                            effect,
                            cseq,
                            ospec,
                            Some(projectee_item_key),
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
                            projector_item_key,
                            effect,
                            cseq,
                            ospec,
                            Some(projectee_item_key),
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

fn irr_data_to_raw(irr_data: &[IrrEntry]) -> AttrVal {
    irr_data.iter().map(|v| v.amount / v.cycle_time).sum()
}

const RR_PEN_ADDITION: f64 = 7000.0;
const RR_PEN_MULTIPLIER: f64 = 20.0;

fn irr_data_to_penalized(irr_data: Vec<IrrEntry>) -> AttrVal {
    // For considerations of RR diminishing returns multiplier, cycle time is rounded this way
    let total_adjusted_rps: AttrVal = irr_data.iter().map(|v| v.amount / trunc_unerr(v.cycle_time)).sum();
    let mut result = OF(0.0);
    for entry in irr_data.into_iter() {
        let adjusted_rps = entry.amount / trunc_unerr(entry.cycle_time);
        let modified_rps = adjusted_rps.mul_add(RR_PEN_MULTIPLIER, RR_PEN_ADDITION);
        let mult = OF(1.0) - (((adjusted_rps + modified_rps) / (total_adjusted_rps + modified_rps)) - OF(1.0)).powi(2);
        // Truncated cycle time is used only for multiplier
        result += mult * entry.amount / entry.cycle_time;
    }
    result
}

fn get_shield_regen(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey, shield_perc: UnitInterval) -> AttrVal {
    let attr_consts = ctx.ac();
    let shield_hp = calc
        .get_item_oattr_afb_oextra(ctx, item_key, attr_consts.shield_capacity, OF(0.0))
        .unwrap();
    let shield_regen_time = calc
        .get_item_oattr_afb_oextra(ctx, item_key, attr_consts.shield_recharge_rate, OF(0.0))
        .unwrap()
        / OF(1000.0);
    calc_regen(shield_hp, shield_regen_time, shield_perc.get_inner())
}
