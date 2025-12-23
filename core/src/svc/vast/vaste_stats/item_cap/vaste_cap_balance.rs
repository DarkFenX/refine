use std::cmp::Ordering;

use super::shared::{CYCLE_OPTIONS_BURST, CYCLE_OPTIONS_SIM};
use crate::{
    def::{AttrVal, OF},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::get_item_cycle_info,
        err::StatItemCheckError,
        vast::{Vast, VastFitData, shared::calc_regen, vaste_stats::item_checks::check_ship},
    },
    ud::UItemKey,
    util::UnitInterval,
};

/// Capacitor change sources which will be considered for cap balance stats.
#[derive(Copy, Clone)]
pub struct StatCapSrcKinds {
    pub regen: StatCapRegenOptions,
    pub cap_injectors: bool,
    pub nosfs: bool,
    pub consumers: StatCapConsumerOptions,
    pub incoming_transfers: bool,
    pub incoming_neuts: bool,
}
impl StatCapSrcKinds {
    /// Include all capacitor change sources.
    pub fn all_enabled() -> Self {
        Self {
            regen: StatCapRegenOptions { enabled: true, .. },
            cap_injectors: true,
            nosfs: true,
            consumers: StatCapConsumerOptions { enabled: true, .. },
            incoming_transfers: true,
            incoming_neuts: true,
        }
    }
    /// Exclude all capacitor change sources.
    pub fn all_disabled() -> Self {
        Self {
            regen: StatCapRegenOptions { enabled: false, .. },
            cap_injectors: false,
            nosfs: false,
            consumers: StatCapConsumerOptions { enabled: false, .. },
            incoming_transfers: false,
            incoming_neuts: false,
        }
    }
}

#[derive(Copy, Clone)]
pub struct StatCapRegenOptions {
    pub enabled: bool,
    pub cap_perc: UnitInterval = UnitInterval::new_const(OF(0.25)),
}

#[derive(Copy, Clone)]
pub struct StatCapConsumerOptions {
    pub enabled: bool,
    pub reload: bool = false,
}

impl Vast {
    pub(in crate::svc) fn get_stat_item_cap_balance(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        src_kinds: StatCapSrcKinds,
    ) -> Result<AttrVal, StatItemCheckError> {
        let ship = check_ship(ctx.u_data, item_key)?;
        let fit_data = self.fit_datas.get(&ship.get_fit_key()).unwrap();
        let mut balance = OF(0.0);
        if src_kinds.regen.enabled {
            balance += get_cap_regen(ctx, calc, item_key, src_kinds.regen.cap_perc);
        }
        if src_kinds.cap_injectors {
            balance += get_cap_injects(ctx, calc, fit_data);
        }
        if src_kinds.consumers.enabled || src_kinds.nosfs {
            balance -= get_cap_consumed(
                ctx,
                calc,
                src_kinds.consumers.reload,
                fit_data,
                src_kinds.consumers.enabled,
                src_kinds.nosfs,
            );
        }
        if src_kinds.incoming_transfers {
            balance += get_cap_transfers(ctx, calc, item_key, self);
        }
        if src_kinds.incoming_neuts {
            balance -= get_neuts(ctx, calc, item_key, self);
        }
        Ok(balance)
    }
}

fn get_cap_regen(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey, cap_perc: UnitInterval) -> AttrVal {
    let max_amount = Vast::internal_get_stat_item_cap_unchecked(ctx, calc, item_key);
    let cap_regen_time = calc
        .get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().recharge_rate, OF(0.0))
        .unwrap()
        / OF(1000.0);
    calc_regen(max_amount, cap_regen_time, cap_perc.get_inner())
}

fn get_cap_injects(ctx: SvcCtx, calc: &mut Calc, fit_data: &VastFitData) -> AttrVal {
    let mut cps = OF(0.0);
    for (&item_key, item_data) in fit_data.cap_injects.iter() {
        let cycle_map = match get_item_cycle_info(ctx, calc, item_key, CYCLE_OPTIONS_SIM, false) {
            Some(cycle_map) => cycle_map,
            None => continue,
        };
        for (&effect_key, cap_getter) in item_data.iter() {
            let cap_injected = match cap_getter(ctx, calc, item_key) {
                Some(cap_injected) => cap_injected,
                None => continue,
            };
            let effect_cycles = match cycle_map.get(&effect_key) {
                Some(effect_cycles) => effect_cycles,
                None => continue,
            };
            cps += cap_injected / effect_cycles.get_average_time();
        }
    }
    cps
}

fn get_cap_consumed(
    ctx: SvcCtx,
    calc: &mut Calc,
    reload: bool,
    fit_data: &VastFitData,
    drains: bool,
    gains: bool,
) -> AttrVal {
    let mut cps = OF(0.0);
    let cycle_options = match reload {
        true => CYCLE_OPTIONS_SIM,
        false => CYCLE_OPTIONS_BURST,
    };
    for (&item_key, item_data) in fit_data.cap_consumers_active.iter() {
        let cycle_map = match get_item_cycle_info(ctx, calc, item_key, cycle_options, false) {
            Some(cycle_map) => cycle_map,
            None => continue,
        };
        for (&effect_key, &attr_key) in item_data.iter() {
            let cap_consumed = match calc.get_item_attr_oextra(ctx, item_key, attr_key) {
                Some(cap_consumed) => cap_consumed,
                None => continue,
            };
            match (cap_consumed.cmp(&OF(0.0)), drains, gains) {
                (Ordering::Greater, true, _) | (Ordering::Less, _, true) => (),
                _ => continue,
            };
            let effect_cycles = match cycle_map.get(&effect_key) {
                Some(effect_cycles) => effect_cycles,
                None => continue,
            };
            cps += cap_consumed / effect_cycles.get_average_time();
        }
    }
    cps
}

fn get_cap_transfers(ctx: SvcCtx, calc: &mut Calc, cap_item_key: UItemKey, vast: &Vast) -> AttrVal {
    let mut cps = OF(0.0);
    let transfer_data = match vast.in_cap.get_l1(&cap_item_key) {
        Some(transfer_data) => transfer_data,
        None => return cps,
    };
    for (&transfer_item_key, item_data) in transfer_data.iter() {
        let cycle_map = match get_item_cycle_info(ctx, calc, transfer_item_key, CYCLE_OPTIONS_BURST, false) {
            Some(cycle_map) => cycle_map,
            None => continue,
        };
        for (&effect_key, cap_getter) in item_data.iter() {
            let effect = ctx.u_data.src.get_effect(effect_key);
            let output_per_cycle = match cap_getter(ctx, calc, transfer_item_key, effect, None, Some(cap_item_key)) {
                Some(output_per_cycle) => output_per_cycle,
                None => continue,
            };
            let effect_cycles = match cycle_map.get(&effect_key) {
                Some(effect_cycles) => effect_cycles,
                None => continue,
            };
            cps += output_per_cycle.get_total() / effect_cycles.get_average_time();
        }
    }
    cps
}

fn get_neuts(ctx: SvcCtx, calc: &mut Calc, cap_item_key: UItemKey, vast: &Vast) -> AttrVal {
    let mut nps = OF(0.0);
    let neut_data = match vast.in_neuts.get_l1(&cap_item_key) {
        Some(neut_data) => neut_data,
        None => return nps,
    };
    for (&neut_item_key, item_data) in neut_data.iter() {
        let cycle_map = match get_item_cycle_info(ctx, calc, neut_item_key, CYCLE_OPTIONS_BURST, false) {
            Some(cycle_map) => cycle_map,
            None => continue,
        };
        for (&effect_key, cap_getter) in item_data.iter() {
            let effect = ctx.u_data.src.get_effect(effect_key);
            let output_per_cycle = match cap_getter(ctx, calc, neut_item_key, effect, Some(cap_item_key)) {
                Some(output_per_cycle) => output_per_cycle,
                None => continue,
            };
            let effect_cycles = match cycle_map.get(&effect_key) {
                Some(effect_cycles) => effect_cycles,
                None => continue,
            };
            nps += output_per_cycle.get_total() / effect_cycles.get_average_time();
        }
    }
    nps
}
