use super::shared::item_check;
use crate::{
    ad,
    def::{AttrVal, ItemKey, OF},
    misc::Spool,
    nd::{NLocalRepGetter, NRemoteRepGetter},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleOptionReload, CycleOptions, get_item_cycle_info},
        err::StatItemCheckError,
        vast::{StatTank, Vast},
    },
    uad::UadItem,
    util::{RMapRMap, RMapRMapRMap, trunc_unerr},
};

pub struct StatLayerRps {
    pub local: AttrVal,
    pub remote: AttrVal,
    pub remote_penalized: AttrVal,
}

impl Vast {
    pub(in crate::svc) fn get_stat_item_rps_checked(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: ItemKey,
        spool: Option<Spool>,
    ) -> Result<StatTank<StatLayerRps>, StatItemCheckError> {
        let uad_item = ctx.uad.items.get(item_key);
        item_check(item_key, uad_item)?;
        Ok(self.get_stat_item_rps_unchecked(ctx, calc, item_key, uad_item, spool))
    }
    pub(super) fn get_stat_item_rps_unchecked(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: ItemKey,
        uad_item: &UadItem,
        spool: Option<Spool>,
    ) -> StatTank<StatLayerRps> {
        // Local reps
        let (local_shield, local_armor, local_hull) = match uad_item {
            UadItem::Ship(uad_ship) => {
                let fit_data = self.get_fit_data(&uad_ship.get_fit_key());
                let local_shield = get_local_rps(ctx, calc, &fit_data.lr_shield);
                let local_armor = get_local_rps(ctx, calc, &fit_data.lr_armor);
                let local_hull = get_local_rps(ctx, calc, &fit_data.lr_hull);
                (local_shield, local_armor, local_hull)
            }
            _ => (OF(0.0), OF(0.0), OF(0.0)),
        };
        let shield_irr_data = get_irr_data(ctx, calc, item_key, spool, &self.irr_shield);
        let armor_irr_data = get_irr_data(ctx, calc, item_key, spool, &self.irr_armor);
        let hull_irr_data = get_irr_data(ctx, calc, item_key, spool, &self.irr_hull);
        StatTank {
            shield: StatLayerRps {
                local: local_shield,
                remote: irr_data_to_raw(&shield_irr_data),
                remote_penalized: irr_data_to_penalized(shield_irr_data),
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

fn get_local_rps(
    ctx: SvcCtx,
    calc: &mut Calc,
    rep_data: &RMapRMap<ItemKey, ad::AEffectId, NLocalRepGetter>,
) -> AttrVal {
    let mut total_rps = OF(0.0);
    for (&item_key, item_data) in rep_data.iter() {
        let cycle_map = match get_item_cycle_info(ctx, calc, item_key, RPS_CYCLE_OPTIONS, false) {
            Some(projector_cycle_map) => projector_cycle_map,
            None => continue,
        };
        for (a_effect_id, rep_getter) in item_data.iter() {
            let effect_cycles = match cycle_map.get(&a_effect_id) {
                Some(effect_cycles) => effect_cycles,
                None => continue,
            };
            let a_effect = ctx.uad.src.get_a_effect(a_effect_id).unwrap();
            let output_per_cycle = match rep_getter(ctx, calc, item_key, a_effect) {
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
    projectee_item_key: ItemKey,
    spool: Option<Spool>,
    sol_irrs: &RMapRMapRMap<ItemKey, ItemKey, ad::AEffectId, NRemoteRepGetter>,
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
        for (a_effect_id, rep_getter) in projector_data.iter() {
            let effect_cycles = match projector_cycle_map.get(&a_effect_id) {
                Some(effect_cycles) => effect_cycles,
                None => continue,
            };
            let a_effect = ctx.uad.src.get_a_effect(a_effect_id).unwrap();
            let output_per_cycle =
                match rep_getter(ctx, calc, projector_item_key, a_effect, spool, Some(projectee_item_key)) {
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
