use crate::{
    Value,
    svc::{Calc, SvcCtx, Vast, err::IntStatItemError, vast::stats::item_checks::check_fighter_ship_no_struct},
    ud::{UItem, UItemId},
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_can_warp(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<bool, IntStatItemError<!>> {
        let item = check_fighter_ship_no_struct(ctx.u_data, item_uid)?;
        // Warping is blocked by either of:
        // - standard warp scram status attribute (points, HIC scripted points)
        // - custom warp status attribute (bubbles)
        // - having no max velocity
        // - having any modules with effects which disable warp, if stat is fetched for a ship (non-covops
        //   cloaks)
        if let UItem::Ship(ship) = item {
            let fit_data = self.get_fit_data(ship.get_fit_uid());
            if !fit_data.mod_effects_disallow_warp.is_empty() {
                return Ok(false);
            }
        }
        let warp_status = calc.get_item_oattr_ffb_extra(ctx, item_uid, ctx.ac().warp_scramble_status, Value::ZERO);
        if warp_status > Value::FLOAT_TOLERANCE {
            return Ok(false);
        }
        // Do not block by velocity requirement if attribute is not defined
        if let Some(max_speed) = calc.get_item_oattr_oextra(ctx, item_uid, ctx.ac().max_velocity)
            && max_speed < Value::FLOAT_TOLERANCE
        {
            return Ok(false);
        }
        let warp_jump_status = calc.get_item_oattr_ffb_extra(ctx, item_uid, ctx.ac().disallow_warping, Value::ZERO);
        if warp_jump_status > Value::FLOAT_TOLERANCE {
            return Ok(false);
        }
        Ok(true)
    }
}
