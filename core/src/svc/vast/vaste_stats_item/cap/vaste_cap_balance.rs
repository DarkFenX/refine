use super::{
    super::checks::check_item_ship,
    shared::{CYCLE_OPTIONS_BURST, CYCLE_OPTIONS_SIM},
};
use crate::{
    ac,
    def::{AttrVal, OF},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::get_item_cycle_info,
        err::StatItemCheckError,
        vast::{Vast, VastFitData},
    },
    ud::UItemKey,
    util::UnitInterval,
};

/// Capacitor change sources which will be considered for cap balance stats.
#[derive(Copy, Clone)]
pub struct StatCapSrcKinds {
    pub regen: StatCapRegenOptions,
    pub cap_injectors: bool,
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
            consumers: StatCapConsumerOptions { enabled: false, .. },
            incoming_transfers: false,
            incoming_neuts: false,
        }
    }
}

#[derive(Copy, Clone)]
pub struct StatCapRegenOptions {
    pub enabled: bool,
    pub cap_perc: Option<UnitInterval> = None,
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
        let item = ctx.u_data.items.get(item_key);
        check_item_ship(item_key, item)?;
        let fit_data = self.fit_datas.get(&item.get_ship().unwrap().get_fit_key()).unwrap();
        let mut balance = OF(0.0);
        if src_kinds.regen.enabled {
            balance += get_cap_regen(ctx, calc, item_key, src_kinds.regen.cap_perc);
        }
        if src_kinds.cap_injectors {
            balance += get_cap_injects(ctx, calc, fit_data);
        }
        if src_kinds.consumers.enabled {
            balance -= get_cap_consumed(ctx, calc, src_kinds.consumers.reload, fit_data);
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

fn get_cap_regen(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey, cap_perc: Option<UnitInterval>) -> AttrVal {
    let max_amount = Vast::internal_get_stat_item_cap_unchecked(ctx, calc, item_key);
    let cap_regen_time = calc
        .get_item_attr_val_extra(ctx, item_key, &ac::attrs::RECHARGE_RATE)
        .unwrap()
        / OF(1000.0);
    let cap_perc = match cap_perc {
        Some(cap_perc) => cap_perc.get_inner(),
        None => OF(0.25),
    };
    let result = OF(10.0) * max_amount / cap_regen_time * (OF(cap_perc.sqrt()) - cap_perc);
    match result.is_finite() {
        true => result,
        false => OF(0.0),
    }
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
            cps += cap_injected / effect_cycles.get_average_cycle_time();
        }
    }
    cps
}

fn get_cap_consumed(ctx: SvcCtx, calc: &mut Calc, reload: bool, fit_data: &VastFitData) -> AttrVal {
    let mut cps = OF(0.0);
    let cycle_options = match reload {
        true => CYCLE_OPTIONS_SIM,
        false => CYCLE_OPTIONS_BURST,
    };
    for (&item_key, item_data) in fit_data.cap_consumers.iter() {
        let cycle_map = match get_item_cycle_info(ctx, calc, item_key, cycle_options, false) {
            Some(cycle_map) => cycle_map,
            None => continue,
        };
        for (&effect_key, attr_id) in item_data.iter() {
            let cap_used = match calc.get_item_attr_val_extra(ctx, item_key, attr_id) {
                Ok(cap_used) => cap_used,
                Err(_) => continue,
            };
            let effect_cycles = match cycle_map.get(&effect_key) {
                Some(effect_cycles) => effect_cycles,
                None => continue,
            };
            cps += cap_used / effect_cycles.get_average_cycle_time();
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
            cps += output_per_cycle.get_total() / effect_cycles.get_average_cycle_time();
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
            nps += output_per_cycle.get_total() / effect_cycles.get_average_cycle_time();
        }
    }
    nps
}
