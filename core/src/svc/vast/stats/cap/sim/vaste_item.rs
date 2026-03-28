use crate::{
    misc::OptionalReload,
    num::UnitInterval,
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
        stagger: StatCapSimStaggerInt,
    ) -> Result<StatCapSim, StatItemCheckError> {
        let ship = check_ship(ctx.u_data, item_uid)?;
        let max_cap = Vast::get_stat_item_cap_amount(ctx, calc, item_uid).unwrap();
        let recharge_time = Vast::internal_get_stat_item_cap_recharge_time_unchecked(ctx, calc, item_uid);
        let start_cap = max_cap * cap_perc.into_pvalue();
        let fit_data = self.fit_datas.get(&ship.get_fit_uid()).unwrap();
        let events = prepare_events(
            reuse_cseq_map,
            ctx,
            calc,
            self,
            optional_reloads,
            stagger,
            fit_data,
            item_uid,
        );
        let mut sim = CapSim::new(start_cap, max_cap, recharge_time, events);
        Ok(sim.run())
    }
}
