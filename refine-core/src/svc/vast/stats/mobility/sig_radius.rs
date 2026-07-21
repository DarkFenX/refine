use crate::{
    PValue,
    svc::{Calc, SvcCtx, Vast, err::IntItemStatError, funcs, vast::stats::item_checks::check_drone_fighter_ship},
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_sig_radius(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<PValue, IntItemStatError<!>> {
        check_drone_fighter_ship(ctx.u_data, item_uid)?;
        Ok(Self::internal_get_stat_item_sig_radius_unchecked(ctx, calc, item_uid))
    }
    pub(in crate::svc::vast::stats) fn internal_get_stat_item_sig_radius_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> PValue {
        funcs::get_sig_radius(ctx, calc, item_uid)
    }
}
