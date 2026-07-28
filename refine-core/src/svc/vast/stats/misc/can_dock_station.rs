use crate::{
    Value,
    svc::{Calc, SvcCtx, Vast, err::IntItemStatError, vast::stats::item_checks::check_ship_no_struct},
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_can_dock_station(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<bool, IntItemStatError<!>> {
        let ship = check_ship_no_struct(ctx.u_data, item_uid)?;
        // Station docking is blocked by either of:
        // - having any aggro effects active
        // - standard dock status attribute (scripted HIC ray)
        // - having any modules with effects which disable docking (cloaks, MJDs)
        let fit_data = self.get_fit_data(ship.get_fit_uid());
        if !fit_data.effects_aggro.is_empty() {
            return Ok(false);
        }
        if !fit_data.mod_effects_disallow_dock.is_empty() {
            return Ok(false);
        }
        let dock_status = calc.get_item_oattr_ffb_extra(ctx, item_uid, ctx.ac().disallow_docking, Value::ZERO);
        if dock_status > Value::FLOAT_TOLERANCE {
            return Ok(false);
        }
        Ok(true)
    }
}
