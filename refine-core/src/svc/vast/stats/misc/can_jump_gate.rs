use crate::{
    Value,
    svc::{Calc, SvcCtx, Vast, err::IntStatItemError, vast::stats::item_checks::check_ship_no_struct},
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_can_jump_gate(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<bool, IntStatItemError<!>> {
        let ship = check_ship_no_struct(ctx.u_data, item_uid)?;
        // Gating is blocked by either of:
        // - having any running effects which give weapons timer
        // - standard gate scram status attribute (scripted HIC ray)
        // - standard drive jump status attribute (disruptive lance, it controls both drive jumps
        //   and gate jumps)
        // - having any modules with effects which disable gate jumping
        let fit_data = self.get_fit_data(ship.get_fit_uid());
        if !fit_data.effects_weapons_timer.is_empty() {
            return Ok(false);
        }
        if !fit_data.mod_effects_disallow_jump_gate.is_empty() {
            return Ok(false);
        }
        let gate_status = calc.get_item_oattr_ffb_extra(ctx, item_uid, ctx.ac().gate_scramble_status, Value::ZERO);
        if gate_status > Value::FLOAT_TOLERANCE {
            return Ok(false);
        }
        let dock_status = calc.get_item_oattr_ffb_extra(ctx, item_uid, ctx.ac().disallow_drive_jumping, Value::ZERO);
        Ok(dock_status <= Value::FLOAT_TOLERANCE)
    }
}
