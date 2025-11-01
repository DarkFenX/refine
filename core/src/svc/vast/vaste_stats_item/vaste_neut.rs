use super::checks::check_item_key_charge_drone_fighter_module;
use crate::{
    def::{AttrVal, OF},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleOptionReload, CycleOptions, get_item_cycle_info},
        err::StatItemCheckError,
        vast::Vast,
    },
    ud::UItemKey,
};

const NEUT_CYCLE_OPTIONS: CycleOptions = CycleOptions {
    reload_mode: CycleOptionReload::Burst,
    reload_optionals: false,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_remote_nps(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        include_charges: bool,
        ignore_state: bool,
        projectee_key: Option<UItemKey>,
    ) -> Result<AttrVal, StatItemCheckError> {
        check_item_key_charge_drone_fighter_module(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_remote_nps_unchecked(
            ctx,
            calc,
            item_key,
            include_charges,
            ignore_state,
            projectee_key,
        ))
    }
    fn internal_get_stat_item_remote_nps_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        include_charges: bool,
        ignore_state: bool,
        projectee_key: Option<UItemKey>,
    ) -> AttrVal {
        let mut item_nps = OF(0.0);
        let cycle_map = match get_item_cycle_info(ctx, calc, item_key, NEUT_CYCLE_OPTIONS, ignore_state) {
            Some(cycle_map) => cycle_map,
            None => return item_nps,
        };
        for (effect_key, effect_cycle) in cycle_map {
            if !effect_cycle.is_infinite() {
                continue;
            }
            let r_effect = ctx.u_data.src.get_effect(effect_key);
            if let Some(neut_getter) = r_effect.get_neut_opc_getter()
                && let Some(neut_amount) = neut_getter(ctx, calc, item_key, r_effect, projectee_key)
            {
                item_nps += neut_amount.get_total() / effect_cycle.get_average_cycle_time();
            }
        }
        if include_charges {
            for charge_key in ctx.u_data.items.get(item_key).iter_charges() {
                if let Ok(charge_nps) =
                    Vast::get_stat_item_remote_nps(ctx, calc, charge_key, false, ignore_state, projectee_key)
                {
                    item_nps += charge_nps;
                }
            }
        }
        item_nps
    }
}
