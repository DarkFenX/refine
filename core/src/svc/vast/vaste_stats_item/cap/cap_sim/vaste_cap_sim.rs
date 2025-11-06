use ordered_float::Float;

use super::{event_sim::CapSimEvent, iter::CapSimIter};
use crate::{
    ac,
    def::{AttrVal, OF},
    svc::{
        SvcCtx,
        calc::Calc,
        err::StatItemCheckError,
        vast::{Vast, vaste_stats_item::checks::check_item_ship},
    },
    ud::UItemKey,
};

const TIME_LIMIT: AttrVal = OF(4.0 * 60.0 * 60.0);

pub enum StatCapSim {
    // Low watermark of stability value
    Stable(AttrVal),
    // Time in seconds it takes to drain cap to 0
    Time(AttrVal),
}

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
        let tau = calc
            .get_item_attr_val_extra(ctx, item_key, &ac::attrs::RECHARGE_RATE)
            .unwrap()
            / OF(5000.0);
        let mut sim_time = OF(0.0);
        let mut sim_cap = match cap_perc {
            Some(perc) => max_cap * perc,
            None => max_cap,
        };
        // Injectors available for immediate use
        let mut injectors = Vec::new();
        let fit_data = self.fit_datas.get(&item.get_ship().unwrap().get_fit_key()).unwrap();
        let mut event_iter = CapSimIter::new(ctx, calc, self, fit_data, item_key);
        for event in event_iter {
            let event_time = event.get_time();
            if event_time > TIME_LIMIT {
                break;
            }
            if event_time > sim_time {
                sim_cap = calc_regen(sim_cap, max_cap, tau, sim_time, event_time);
                sim_time = event_time;
            }
            match event {
                CapSimEvent::InjectorReady(event) => {
                    let injected_cap = sim_cap + event.output;
                    match injected_cap > max_cap {
                        // Postpone use of injector if it overshoots max cap
                        true => injectors.push(event),
                        false => {
                            sim_cap = injected_cap;
                            event_iter.injector_used(sim_time, event);
                        }
                    }
                }
                CapSimEvent::CapGain(event) => {
                    sim_cap += event.amount;
                    if sim_cap < OF(0.0) {
                        return Ok(StatCapSim::Time(sim_time));
                    }
                    sim_cap = Float::min(sim_cap, max_cap);
                }
            }
        }
        Ok(StatCapSim::Stable(OF(0.25)))
    }
}

fn calc_regen(c0: AttrVal, c_max: AttrVal, tau: AttrVal, t0: AttrVal, t1: AttrVal) -> AttrVal {
    (OF(1.0) + ((c0 / c_max).sqrt() - OF(1.0)) * ((t0 - t1) / tau).exp()).powi(2) * c_max
}
