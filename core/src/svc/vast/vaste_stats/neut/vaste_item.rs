use super::shared::NEUT_CYCLE_OPTIONS;
use crate::{
    def::{AttrVal, OF},
    svc::{
        SvcCtx,
        aggr::aggr_proj_first_ps,
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
        projectee_item_key: Option<UItemKey>,
    ) -> Result<AttrVal, StatItemCheckError> {
        check_charge_drone_fighter_module(ctx.u_data, item_key)?;
        let mut item_nps = OF(0.0);
        let cseq_map = match get_item_cseq_map(ctx, calc, item_key, NEUT_CYCLE_OPTIONS, ignore_state) {
            Some(cseq_map) => cseq_map,
            None => return Ok(item_nps),
        };
        for (effect_key, cseq) in cseq_map {
            let effect = ctx.u_data.src.get_effect(effect_key);
            if let Some(ospec) = effect.neut_opc_spec
                && let Some(effect_nps) =
                    aggr_proj_first_ps(ctx, calc, item_key, effect, &cseq, &ospec, projectee_item_key, None)
            {
                item_nps += effect_nps;
            }
        }
        if include_charges {
            for charge_key in ctx.u_data.items.get(item_key).iter_charges() {
                if let Ok(charge_nps) =
                    Vast::get_stat_item_outgoing_nps(ctx, calc, charge_key, false, ignore_state, projectee_item_key)
                {
                    item_nps += charge_nps;
                }
            }
        }
        Ok(item_nps)
    }
}
