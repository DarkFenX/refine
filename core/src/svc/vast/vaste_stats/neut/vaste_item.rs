use super::shared::NEUT_CYCLE_OPTIONS;
use crate::{
    def::{AttrVal, OF},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::get_item_cseq_map,
        err::StatItemCheckError,
        vast::{Vast, vaste_stats::item_checks::check_charge_drone_fighter_module},
    },
    ud::UItemKey,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_outgoing_nps(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        include_charges: bool,
        ignore_state: bool,
        projectee_key: Option<UItemKey>,
    ) -> Result<AttrVal, StatItemCheckError> {
        check_charge_drone_fighter_module(ctx.u_data, item_key)?;
        let mut item_nps = OF(0.0);
        let cycle_map = match get_item_cseq_map(ctx, calc, item_key, NEUT_CYCLE_OPTIONS, ignore_state) {
            Some(cycle_map) => cycle_map,
            None => return Ok(item_nps),
        };
        for (effect_key, effect_cycle) in cycle_map {
            let effect = ctx.u_data.src.get_effect(effect_key);
            if let Some(neut_ospec) = effect.neut_opc_spec
                && let Some(effect_cycle_loop) = effect_cycle.try_loop_cseq()
            {
                let invar_data = neut_ospec.make_invar_data(ctx, calc, item_key, effect, projectee_key);
                let neut_opc = neut_ospec.get_total(ctx, calc, item_key, effect, None, None, invar_data);
                if let Some(neut_opc) = neut_opc {
                    item_nps += neut_opc / effect_cycle_loop.get_average_time();
                }
            }
        }
        if include_charges {
            for charge_key in ctx.u_data.items.get(item_key).iter_charges() {
                if let Ok(charge_nps) =
                    Vast::get_stat_item_outgoing_nps(ctx, calc, charge_key, false, ignore_state, projectee_key)
                {
                    item_nps += charge_nps;
                }
            }
        }
        Ok(item_nps)
    }
}
