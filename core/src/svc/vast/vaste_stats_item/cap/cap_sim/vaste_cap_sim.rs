use super::iter::CapSimIter;
use crate::{
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
        let fit_data = self.fit_datas.get(&item.get_ship().unwrap().get_fit_key()).unwrap();
        for (time, amount) in CapSimIter::new(ctx, calc, self, fit_data, item_key) {
            if time > TIME_LIMIT {
                break;
            }
        }
        Ok(StatCapSim::Stable(OF(0.25)))
    }
}
