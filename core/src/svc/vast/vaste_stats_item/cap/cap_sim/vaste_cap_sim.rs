use ordered_float::Float;

use super::iter::CapSimIter;
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
        let mut current_time = OF(0.0);
        let mut current_cap = match cap_perc {
            Some(perc) => max_cap * perc,
            None => max_cap,
        };
        let fit_data = self.fit_datas.get(&item.get_ship().unwrap().get_fit_key()).unwrap();
        for (event_time, cap_added) in CapSimIter::new(ctx, calc, self, fit_data, item_key) {
            if event_time > TIME_LIMIT {
                break;
            }
            if event_time > current_time {
                current_cap = (OF(1.0)
                    + ((current_cap / max_cap).sqrt() - OF(1.0)) * ((current_time - event_time) / tau).exp())
                .powi(2)
                    * max_cap;
                current_time = event_time;
            }
            current_cap += cap_added;
            if current_cap < OF(0.0) {
                return Ok(StatCapSim::Time(current_time));
            }
        }
        Ok(StatCapSim::Stable(OF(0.25)))
    }
}
