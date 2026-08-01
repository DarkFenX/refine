use crate::{
    Value,
    svc::{Calc, SvcCtx, Vast, err::IntStatItemError, vast::stats::item_checks::check_ship_no_struct},
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_can_dock_citadel(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<bool, IntStatItemError<!>> {
        let ship = check_ship_no_struct(ctx.u_data, item_uid)?;
        // Citadel docking is blocked by either of:
        // - having any running effects which give weapons timer
        // - standard warp scram status attribute
        // - standard dock status attribute (scripted HIC ray)
        // - having any modules with effects which disable docking (cloaks, MJDs)
        let fit_data = self.get_fit_data(ship.get_fit_uid());
        if !fit_data.effects_weapons_timer.is_empty() {
            return Ok(false);
        }
        if !fit_data.mod_effects_disallow_dock.is_empty() {
            return Ok(false);
        }
        let warp_status = calc.get_item_oattr_ffb_extra(ctx, item_uid, ctx.ac().warp_scramble_status, Value::ZERO);
        if warp_status > Value::FLOAT_TOLERANCE {
            return Ok(false);
        }
        let dock_status = calc.get_item_oattr_ffb_extra(ctx, item_uid, ctx.ac().disallow_docking, Value::ZERO);
        if dock_status > Value::FLOAT_TOLERANCE {
            return Ok(false);
        }
        Ok(true)
    }
}
