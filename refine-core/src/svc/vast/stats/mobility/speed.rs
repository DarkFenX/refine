use crate::{
    PValue,
    svc::{
        Calc, SvcCtx, Vast, err::IntItemStatError, funcs, vast::stats::item_checks::check_drone_fighter_ship_no_struct,
    },
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_speed(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<PValue, IntItemStatError<!>> {
        check_drone_fighter_ship_no_struct(ctx.u_data, item_uid)?;
        Ok(funcs::get_speed(ctx, calc, item_uid))
    }
}
