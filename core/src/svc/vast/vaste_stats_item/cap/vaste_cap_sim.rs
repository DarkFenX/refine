use super::super::checks::check_item_ship;
use crate::{
    def::{AttrVal, OF},
    svc::{SvcCtx, calc::Calc, err::StatItemCheckError, vast::Vast},
    ud::UItemKey,
};

pub enum StatCapSimResult {
    // Low watermark of stability value
    Stable(AttrVal),
    // Time in seconds it takes to drain cap to 0
    Unstable(AttrVal),
}

impl Vast {
    pub(in crate::svc) fn get_stat_item_cap_sim(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<StatCapSimResult, StatItemCheckError> {
        let item = ctx.u_data.items.get(item_key);
        check_item_ship(item_key, item)?;
        Ok(StatCapSimResult::Stable(OF(0.25)))
    }
}
