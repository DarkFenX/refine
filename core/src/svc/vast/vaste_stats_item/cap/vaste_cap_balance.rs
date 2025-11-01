use super::super::checks::check_item_ship;
use crate::{
    ac,
    def::{AttrVal, OF},
    nd::NCapBoostGetter,
    rd::REffectKey,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleOptionReload, CycleOptions, get_item_cycle_info},
        err::StatItemCheckError,
        vast::Vast,
    },
    ud::UItemKey,
    util::RMapRMap,
};

/// Capacitor change sources which will be considered for cap balance stats.
#[derive(Copy, Clone)]
pub struct StatCapSrcKinds {
    pub regen: bool,
    pub cap_boosters: bool,
    pub consumers: bool,
    pub incoming_transfers: bool,
    pub incoming_neuts: bool,
}
impl StatCapSrcKinds {
    /// Include all capacitor change sources.
    pub fn all_enabled() -> Self {
        Self {
            regen: true,
            cap_boosters: true,
            consumers: true,
            incoming_transfers: true,
            incoming_neuts: true,
        }
    }
    /// Exclude all capacitor change sources.
    pub fn all_disabled() -> Self {
        Self {
            regen: false,
            cap_boosters: false,
            consumers: false,
            incoming_transfers: false,
            incoming_neuts: false,
        }
    }
}

const CAP_BOOST_OPTIONS: CycleOptions = CycleOptions {
    reload_mode: CycleOptionReload::Sim,
    charged_optionals: false,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_cap_balance(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        src_kinds: StatCapSrcKinds,
        regen_perc: Option<AttrVal>,
    ) -> Result<AttrVal, StatItemCheckError> {
        let item = ctx.u_data.items.get(item_key);
        check_item_ship(item_key, item)?;
        let fit_data = self.fit_datas.get(&item.get_ship().unwrap().get_fit_key()).unwrap();
        let mut balance = OF(0.0);
        if src_kinds.regen {
            balance += Vast::internal_get_stat_item_cap_regen_unchecked(ctx, calc, item_key, regen_perc);
        }
        if src_kinds.cap_boosters {
            balance += Vast::internal_get_stat_item_cap_boosts_unchecked(ctx, calc, &fit_data.cap_boosts);
        }
        Ok(balance)
    }
    fn internal_get_stat_item_cap_regen_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        regen_perc: Option<AttrVal>,
    ) -> AttrVal {
        let max_amount = Vast::internal_get_stat_item_cap_unchecked(ctx, calc, item_key);
        let cap_regen_time = calc
            .get_item_attr_val_extra(ctx, item_key, &ac::attrs::RECHARGE_RATE)
            .unwrap()
            / OF(1000.0);
        let cap_perc = match regen_perc {
            Some(cap_perc) => cap_perc.clamp(OF(0.0), OF(1.0)),
            None => OF(0.25),
        };
        let result = OF(10.0) * max_amount / cap_regen_time * (OF(cap_perc.sqrt()) - cap_perc);
        match result.is_finite() {
            true => result,
            false => OF(0.0),
        }
    }
    fn internal_get_stat_item_cap_boosts_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_data: &RMapRMap<UItemKey, REffectKey, NCapBoostGetter>,
    ) -> AttrVal {
        let mut cps = OF(0.0);
        for (&item_key, item_data) in fit_data.iter() {
            let cycle_map = match get_item_cycle_info(ctx, calc, item_key, CAP_BOOST_OPTIONS, false) {
                Some(cycle_map) => cycle_map,
                None => continue,
            };
            for (&effect_key, rep_getter) in item_data.iter() {
                let output_per_cycle = match rep_getter(ctx, calc, item_key) {
                    Some(output_per_cycle) => output_per_cycle,
                    None => continue,
                };
                let effect_cycles = match cycle_map.get(&effect_key) {
                    Some(effect_cycles) => effect_cycles,
                    None => continue,
                };
                if !effect_cycles.is_infinite() {
                    continue;
                }
                cps += output_per_cycle.get_total() / effect_cycles.get_average_cycle_time();
            }
        }
        cps
    }
}
