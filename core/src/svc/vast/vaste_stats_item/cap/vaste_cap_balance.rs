use super::super::checks::check_item_key_ship;
use crate::{
    ac,
    def::{AttrVal, OF},
    svc::{SvcCtx, calc::Calc, err::StatItemCheckError, vast::Vast},
    ud::UItemKey,
};

/// Capacitor change sources which will be considered for cap balance stats.
#[derive(Copy, Clone)]
pub struct StatCapSrcKinds {
    pub regen: bool,
    pub cap_boosters: bool,
    pub consumers: bool,
    pub incoming_transfers: bool,
    pub incoming_neuts: bool,
}
impl StatCapSrcKinds {
    /// Include all capacitor change sources.
    pub fn all_enabled() -> Self {
        Self {
            regen: true,
            cap_boosters: true,
            consumers: true,
            incoming_transfers: true,
            incoming_neuts: true,
        }
    }
    /// Exclude all capacitor change sources.
    pub fn all_disabled() -> Self {
        Self {
            regen: false,
            cap_boosters: false,
            consumers: false,
            incoming_transfers: false,
            incoming_neuts: false,
        }
    }
}

impl Vast {
    pub(in crate::svc) fn get_stat_item_cap_balance(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        src_kinds: StatCapSrcKinds,
        regen_perc: Option<AttrVal>,
    ) -> Result<AttrVal, StatItemCheckError> {
        check_item_key_ship(ctx, item_key)?;
        let mut balance = OF(0.0);
        if src_kinds.regen {
            balance += Vast::internal_get_stat_item_cap_regen_unchecked(ctx, calc, item_key, regen_perc);
        }
        Ok(balance)
    }
    fn internal_get_stat_item_cap_regen_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        regen_perc: Option<AttrVal>,
    ) -> AttrVal {
        let max_amount = Vast::internal_get_stat_item_cap_unchecked(ctx, calc, item_key);
        let cap_regen_time = calc
            .get_item_attr_val_extra(ctx, item_key, &ac::attrs::RECHARGE_RATE)
            .unwrap()
            / OF(1000.0);
        let cap_perc = match regen_perc {
            Some(cap_perc) => cap_perc.clamp(OF(0.0), OF(1.0)),
            None => OF(0.25),
        };
        let result = OF(10.0) * max_amount / cap_regen_time * (OF(cap_perc.sqrt()) - cap_perc);
        match result.is_finite() {
            true => result,
            false => OF(0.0),
        }
    }
}
