use crate::{
    def::{AttrVal, OF},
    misc::Spool,
    rd::{REffectKey, REffectLocalOpcSpec, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        aggr::{aggr_local_first_per_second, aggr_proj_first},
        calc::Calc,
        cycle::{CyclingOptions, get_item_cseq_map},
        err::StatItemCheckError,
        vast::{StatTankRegen, Vast, shared::calc_regen, vaste_stats::item_checks::check_drone_fighter_ship},
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
        let item = check_drone_fighter_ship(ctx.u_data, item_key)?;
        Ok(self.get_stat_item_rps_unchecked(ctx, calc, item_key, item, shield_perc, spool))
    }
    pub(super) fn get_stat_item_rps_unchecked(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        item: &UItem,
        shield_perc: UnitInterval,
        spool: Option<Spool>,
    ) -> StatTankRegen<StatLayerRps, StatLayerRpsRegen> {
        // Local reps
        let (local_shield, local_armor, local_hull) = match item {
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

const RPS_CYCLE_OPTIONS: CyclingOptions = CyclingOptions::Burst;

fn get_local_rps(
    ctx: SvcCtx,
    calc: &mut Calc,
    rep_data: &RMapRMap<UItemKey, REffectKey, REffectLocalOpcSpec<AttrVal>>,
) -> AttrVal {
    let mut total_rps = OF(0.0);
    for (&item_key, item_data) in rep_data.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, item_key, RPS_CYCLE_OPTIONS, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        for (&effect_key, ospec) in item_data.iter() {
            let cseq = match cseq_map.get(&effect_key) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect(effect_key);
            let effect_rps = match aggr_local_first_per_second(ctx, calc, item_key, effect, cseq, ospec) {
                Some(effect_rps) => effect_rps,
                None => continue,
            };
            total_rps += effect_rps;
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
    sol_irrs: &RMapRMapRMap<UItemKey, UItemKey, REffectKey, REffectProjOpcSpec<AttrVal>>,
) -> Vec<IrrEntry> {
    let mut result = Vec::new();
    let incoming_reps = match sol_irrs.get_l1(&projectee_item_key) {
        Some(incoming_reps) => incoming_reps,
        None => return result,
    };
    for (&projector_item_key, projector_data) in incoming_reps.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, projector_item_key, RPS_CYCLE_OPTIONS, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        for (&effect_key, ospec) in projector_data.iter() {
            let cseq = match cseq_map.get(&effect_key) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect(effect_key);
            let effect_rep = match aggr_proj_first(
                ctx,
                calc,
                projector_item_key,
                effect,
                cseq,
                ospec,
                spool,
                Some(projectee_item_key),
            ) {
                Some(effect_rep) => effect_rep,
                None => continue,
            };
            result.push(IrrEntry {
                // For now there are no reps which spread effect over multiple cycles, so we just
                // record total amount for the purposes of RR penalty
                amount: effect_rep.amount,
                cycle_time: effect_rep.time,
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
