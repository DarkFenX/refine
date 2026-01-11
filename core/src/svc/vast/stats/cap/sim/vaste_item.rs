use crate::{
    num::UnitInterval,
    svc::{
        SvcCtx,
        calc::Calc,
        err::StatItemCheckError,
        vast::{
            Vast,
            stats::{
                cap::sim::{
                    prepare::prepare_events,
                    sim::{CapSim, StatCapSim},
                    stagger::StatCapSimStaggerInt,
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
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        cap_perc: UnitInterval,
        reload_optionals: Option<bool>,
        stagger: StatCapSimStaggerInt,
    ) -> Result<StatCapSim, StatItemCheckError> {
        let ship = check_ship(ctx.u_data, item_uid)?;
        let max_cap = Vast::get_stat_item_cap_amount(ctx, calc, item_uid).unwrap();
        let recharge_time = Vast::internal_get_stat_item_cap_recharge_time_unchecked(ctx, calc, item_uid);
        let start_cap = max_cap * cap_perc.into_pvalue();
        let fit_data = self.fit_datas.get(&ship.get_fit_uid()).unwrap();
        let events = prepare_events(ctx, calc, self, reload_optionals, stagger, fit_data, item_uid);
        let mut sim = CapSim::new(start_cap, max_cap, recharge_time, events);
        Ok(sim.run())
    }
}
