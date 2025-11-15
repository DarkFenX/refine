use super::super::checks::check_item_drone_fighter_ship;
use crate::{
    def::{AttrVal, OF},
    misc::Spool,
    nd::{NLocalRepGetter, NRemoteRepGetter},
    rd::REffectKey,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleOptionReload, CycleOptions, get_item_cycle_info},
        err::StatItemCheckError,
        vast::{StatTankRegen, Vast},
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
        shield_perc: UnitInterval,
        spool: Option<Spool>,
    ) -> Result<StatTankRegen<StatLayerRps, StatLayerRpsRegen>, StatItemCheckError> {
        let u_item = ctx.u_data.items.get(item_key);
        check_item_drone_fighter_ship(item_key, u_item)?;
        Ok(self.get_stat_item_rps_unchecked(ctx, calc, item_key, u_item, shield_perc, spool))
    }
    pub(super) fn get_stat_item_rps_unchecked(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        u_item: &UItem,
        shield_perc: UnitInterval,
        spool: Option<Spool>,
    ) -> StatTankRegen<StatLayerRps, StatLayerRpsRegen> {
        // Local reps
        let (local_shield, local_armor, local_hull) = match u_item {
            UItem::Ship(u_ship) => {
                let fit_data = self.get_fit_data(&u_ship.get_fit_key());
                let local_shield = get_local_rps(ctx, calc, &fit_data.lr_shield);
                let local_armor = get_local_rps(ctx, calc, &fit_data.lr_armor);
                let local_hull = get_local_rps(ctx, calc, &fit_data.lr_hull);
                (local_shield, local_armor, local_hull)
            }
            _ => (OF(0.0), OF(0.0), OF(0.0)),
        };
        // Incoming remote reps
        let shield_irr_data = get_irr_data(ctx, calc, item_key, spool, &self.irr_shield);
        let armor_irr_data = get_irr_data(ctx, calc, item_key, spool, &self.irr_armor);
        let hull_irr_data = get_irr_data(ctx, calc, item_key, spool, &self.irr_hull);
        StatTankRegen {
            shield: StatLayerRpsRegen {
                local: local_shield,
                remote: irr_data_to_raw(&shield_irr_data),
                remote_penalized: irr_data_to_penalized(shield_irr_data),
                regen: OF(0.0),
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

const RPS_CYCLE_OPTIONS: CycleOptions = CycleOptions {
    reload_mode: CycleOptionReload::Burst,
    reload_optionals: false,
};

fn get_local_rps(ctx: SvcCtx, calc: &mut Calc, rep_data: &RMapRMap<UItemKey, REffectKey, NLocalRepGetter>) -> AttrVal {
    let mut total_rps = OF(0.0);
    for (&item_key, item_data) in rep_data.iter() {
        let cycle_map = match get_item_cycle_info(ctx, calc, item_key, RPS_CYCLE_OPTIONS, false) {
            Some(projector_cycle_map) => projector_cycle_map,
            None => continue,
        };
        for (&effect_key, rep_getter) in item_data.iter() {
            let effect_cycles = match cycle_map.get(&effect_key) {
                Some(effect_cycles) => effect_cycles,
                None => continue,
            };
            if !effect_cycles.is_infinite() {
                continue;
            }
            let effect = ctx.u_data.src.get_effect(effect_key);
            let output_per_cycle = match rep_getter(ctx, calc, item_key, effect) {
                Some(hp_per_cycle) => hp_per_cycle,
                None => continue,
            };
            total_rps += output_per_cycle.get_total() / effect_cycles.get_average_cycle_time();
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
    spool: Option<Spool>,
    sol_irrs: &RMapRMapRMap<UItemKey, UItemKey, REffectKey, NRemoteRepGetter>,
) -> Vec<IrrEntry> {
    let mut result = Vec::new();
    let incoming_reps = match sol_irrs.get_l1(&projectee_item_key) {
        Some(incoming_reps) => incoming_reps,
        None => return result,
    };
    for (&projector_item_key, projector_data) in incoming_reps.iter() {
        let projector_cycle_map = match get_item_cycle_info(ctx, calc, projector_item_key, RPS_CYCLE_OPTIONS, false) {
            Some(projector_cycle_map) => projector_cycle_map,
            None => continue,
        };
        for (&effect_key, rep_getter) in projector_data.iter() {
            let effect_cycles = match projector_cycle_map.get(&effect_key) {
                Some(effect_cycles) => effect_cycles,
                None => continue,
            };
            let r_effect = ctx.u_data.src.get_effect(effect_key);
            let output_per_cycle =
                match rep_getter(ctx, calc, projector_item_key, r_effect, spool, Some(projectee_item_key)) {
                    Some(hp_per_cycle) => hp_per_cycle,
                    None => continue,
                };
            let cycle_time_s = effect_cycles.get_average_cycle_time();
            result.push(IrrEntry {
                // For now there are no reps which spread effect over multiple cycles, so we just
                // record total amount for the purposes of RR penalty
                amount: output_per_cycle.get_total(),
                cycle_time: cycle_time_s,
            });
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
