use super::shared::item_check;
use crate::{
    def::{AttrVal, ItemKey, OF},
    misc::{EffectSpec, Spool},
    nd::{NLocalRepGetter, NRemoteRepGetter},
    svc::{
        SvcCtx,
        calc::Calc,
        efuncs,
        err::StatItemCheckError,
        vast::{StatTank, Vast},
    },
    uad::UadItem,
    util::{RMap, RMapRMap, trunc_unerr},
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

fn get_local_rps(ctx: SvcCtx, calc: &mut Calc, rep_data: &RMap<EffectSpec, NLocalRepGetter>) -> AttrVal {
    let mut total_rps = OF(0.0);
    for (&rep_espec, rep_getter) in rep_data.iter() {
        let rep_amount = match rep_getter(ctx, calc, rep_espec.item_key) {
            Some(rep_amount) => rep_amount,
            None => continue,
        };
        // Can unwrap here because if rep effects is registered, it should have its item loaded and
        // the effect should have duration attribute specified
        let cycle_time = efuncs::get_espec_cycle_time(ctx, calc, rep_espec).unwrap();
        if cycle_time > OF(0.0) {
            total_rps += rep_amount / cycle_time;
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
    item_key: ItemKey,
    spool: Option<Spool>,
    sol_irrs: &RMapRMap<ItemKey, EffectSpec, NRemoteRepGetter>,
) -> Vec<IrrEntry> {
    let mut result = Vec::new();
    if let Some(item_irrs) = sol_irrs.get_l1(&item_key) {
        for (&rep_espec, rep_getter) in item_irrs.iter() {
            let rep_amount = match rep_getter(ctx, calc, rep_espec, spool, Some(item_key)) {
                Some(rep_amount) => rep_amount,
                None => continue,
            };
            // Can unwrap here because if rep effects is registered, it should have its item loaded
            // and the effect should have duration attribute specified
            let cycle_time = efuncs::get_espec_cycle_time(ctx, calc, rep_espec).unwrap();
            result.push(IrrEntry {
                amount: rep_amount,
                cycle_time,
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
