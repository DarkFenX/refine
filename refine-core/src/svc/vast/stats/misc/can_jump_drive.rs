use crate::{
    Value,
    svc::{Calc, SvcCtx, Vast, err::IntItemStatError, vast::stats::item_checks::check_ship_no_struct},
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_can_jump_drive(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<bool, IntItemStatError<!>> {
        let ship = check_ship_no_struct(ctx.u_data, item_uid)?;
        // Jumping (with a jump drive) is blocked by either of:
        // - standard warp scram status attribute (points, HIC scripted points)
        // - standard drive jump status attribute (disruptive lance, it controls both drive jumps and gate
        //   jumps)
        // - custom drive jump status attribute (bubbles)
        // - having any modules with effects which disable drive jumping (cloaks, MJDs)
        let fit_data = self.get_fit_data(ship.get_fit_uid());
        if !fit_data.mod_effects_disallow_jump_drive.is_empty() {
            return Ok(false);
        }
        let warp_status = calc.get_item_oattr_ffb_extra(ctx, item_uid, ctx.ac().warp_scramble_status, Value::ZERO);
        if warp_status > Value::FLOAT_TOLERANCE {
            return Ok(false);
        }
        let jump_status = calc.get_item_oattr_ffb_extra(ctx, item_uid, ctx.ac().disallow_drive_jumping, Value::ZERO);
        if jump_status > Value::FLOAT_TOLERANCE {
            return Ok(false);
        }
        let jump_status =
            calc.get_item_oattr_ffb_extra(ctx, item_uid, ctx.ac().disallow_drive_jumping_only, Value::ZERO);
        if jump_status > Value::FLOAT_TOLERANCE {
            return Ok(false);
        }
        Ok(true)
    }
}
