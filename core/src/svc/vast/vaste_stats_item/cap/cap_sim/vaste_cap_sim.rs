use std::collections::BinaryHeap;

use ordered_float::Float;

use crate::{
    ac,
    def::{AttrVal, OF},
    svc::{
        SvcCtx,
        calc::Calc,
        err::StatItemCheckError,
        vast::{
            Vast,
            vaste_stats_item::{
                cap::cap_sim::{
                    event::{CapSimEvent, CapSimEventCapGain, CapSimEventCycleCheck, CapSimEventInjector},
                    prepare::prepare_events,
                    sim::{CapSim, StatCapSim},
                },
                checks::check_item_ship,
            },
        },
    },
    ud::UItemKey,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_cap_sim(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        cap_perc: Option<AttrVal>,
    ) -> Result<StatCapSim, StatItemCheckError> {
        let item = ctx.u_data.items.get(item_key);
        check_item_ship(item_key, item)?;
        let max_cap = Vast::get_stat_item_cap_amount(ctx, calc, item_key).unwrap();
        let recharge_time = calc
            .get_item_attr_val_extra(ctx, item_key, &ac::attrs::RECHARGE_RATE)
            .unwrap()
            / OF(1000.0);
        let start_cap = match cap_perc {
            Some(perc) => max_cap * perc,
            None => max_cap,
        };
        let fit_data = self.fit_datas.get(&item.get_ship().unwrap().get_fit_key()).unwrap();
        let events = prepare_events(ctx, calc, self, fit_data, item_key);
        let mut sim = CapSim::new(start_cap, max_cap, recharge_time, events);
        Ok(sim.run())
    }
}
