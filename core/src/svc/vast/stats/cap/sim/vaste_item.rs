use crate::{
    misc::OptionalReload,
    num::{PValue, UnitInterval, Value},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::CseqMap,
        err::StatItemCheckError,
        vast::{
            Vast,
            stats::{
                cap::sim::{
                    prepare::{StatCapSimStaggerInt, prepare_events},
                    sim::{CapSim, StatCapSim},
                },
                item_checks::check_ship,
            },
        },
    },
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_cap_sim(
        &self,
        reuse_cseq_map: &mut CseqMap,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        cap_perc: UnitInterval,
        optional_reloads: Option<OptionalReload>,
        stagger: &StatCapSimStaggerInt,
        nosf_projectee_item_uid: Option<UItemId>,
    ) -> Result<StatCapSim, StatItemCheckError> {
        let ship = check_ship(ctx.u_data, item_uid)?;
        let max_cap = Self::get_stat_item_cap_amount(ctx, calc, item_uid).unwrap();
        let recharge_time_ms = calc
            .get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().recharge_rate, Value::ZERO)
            .unwrap();
        let recharge_time_s = match recharge_time_ms < Value::FLOAT_TOLERANCE {
            true => None,
            false => Some(PValue::from_value_clamped(recharge_time_ms / Value::THOUSAND)),
        };
        let start_cap = max_cap * cap_perc.into_pvalue();
        let fit_data = self.get_fit_data(ship.get_fit_uid());
        let events = prepare_events(
            reuse_cseq_map,
            ctx,
            calc,
            self,
            optional_reloads,
            stagger,
            nosf_projectee_item_uid,
            fit_data,
            item_uid,
        );
        let mut sim = CapSim::new(start_cap, max_cap, recharge_time_s, events);
        Ok(sim.run())
    }
}
