use crate::{
    def::OF,
    svc::{
        SvcCtx,
        calc::Calc,
        err::StatItemCheckError,
        vast::{
            Vast,
            vaste_stats::{
                item_cap::cap_sim::{
                    prepare::prepare_events,
                    sim::{CapSim, StatCapSim},
                    stagger::StatCapSimStaggerInt,
                },
                item_checks::check_ship,
            },
        },
    },
    ud::UItemKey,
    util::UnitInterval,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_cap_sim(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        cap_perc: UnitInterval,
        stagger: StatCapSimStaggerInt,
    ) -> Result<StatCapSim, StatItemCheckError> {
        let ship = check_ship(ctx.u_data, item_key)?;
        let max_cap = Vast::get_stat_item_cap_amount(ctx, calc, item_key).unwrap();
        let recharge_time = calc
            .get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().recharge_rate, OF(0.0))
            .unwrap()
            / OF(1000.0);
        let start_cap = max_cap * cap_perc.get_inner();
        let fit_data = self.fit_datas.get(&ship.get_fit_key()).unwrap();
        let events = prepare_events(ctx, calc, self, stagger, fit_data, item_key);
        let mut sim = CapSim::new(start_cap, max_cap, recharge_time, events);
        Ok(sim.run())
    }
}
