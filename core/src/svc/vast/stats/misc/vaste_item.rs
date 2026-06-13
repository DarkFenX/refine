use crate::{
    num::{Count, PValue, Value},
    svc::{
        SvcCtx,
        calc::Calc,
        err::StatItemCheckError,
        vast::{
            Vast,
            stats::item_checks::{check_character, check_fighter_ship_no_struct, check_ship_no_struct},
        },
    },
    ud::{UFitId, UItemId},
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_drone_control_range(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<PValue, StatItemCheckError> {
        check_character(ctx.u_data, item_uid)?;
        let drone_control_range = calc
            .get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().drone_control_distance, Value::ZERO)
            .unwrap();
        Ok(PValue::from_value_clamped(drone_control_range))
    }
    pub(in crate::svc) fn get_stat_item_can_warp(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<bool, StatItemCheckError> {
        check_fighter_ship_no_struct(ctx.u_data, item_uid)?;
        // Warping is blocked by either of:
        // - standard warp scram status attribute
        // - custom warp status attribute
        // - having no max velocity
        let warp_status = calc
            .get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().warp_scramble_status, Value::ZERO)
            .unwrap();
        if warp_status > Value::FLOAT_TOLERANCE {
            return Ok(false);
        }
        // Do not block by velocity requirement if attribute is not defined
        if let Some(max_speed) = calc.get_item_oattr_oextra(ctx, item_uid, ctx.ac().max_velocity)
            && max_speed < Value::FLOAT_TOLERANCE
        {
            return Ok(false);
        }
        let warp_jump_status = calc
            .get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().disallow_warping, Value::ZERO)
            .unwrap();
        if warp_jump_status > Value::FLOAT_TOLERANCE {
            return Ok(false);
        }
        Ok(true)
    }
    pub(in crate::svc) fn get_stat_item_can_jump_gate(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<bool, StatItemCheckError> {
        let ship = check_ship_no_struct(ctx.u_data, item_uid)?;
        // Gating is blocked by either of:
        // - having aggro modules active
        // - standard gate scram status attribute (scripted HIC ray)
        // - standard drive jump status attribute (disruptive lance, it controls both drive jumps and gate
        //   jumps)
        if self.is_fit_aggroed(ship.get_fit_uid()) {
            return Ok(false);
        }
        let gate_status = calc
            .get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().gate_scramble_status, Value::ZERO)
            .unwrap();
        if gate_status > Value::FLOAT_TOLERANCE {
            return Ok(false);
        }
        let dock_status = calc
            .get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().disallow_drive_jumping, Value::ZERO)
            .unwrap();
        if dock_status > Value::FLOAT_TOLERANCE {
            return Ok(false);
        }
        Ok(true)
    }
    pub(in crate::svc) fn get_stat_item_can_jump_wormhole(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<bool, StatItemCheckError> {
        let ship = check_ship_no_struct(ctx.u_data, item_uid)?;
        // WH jumping is blocked by:
        // - type ID being on type list 245 WH jump black list (supercapitals)
        // - custom WH jump status attribute (MJDs, sieges)
        if ship.get_disallowed_in_wspace() == Some(true) {
            return Ok(false);
        }
        let wh_status = calc
            .get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().disallow_wormhole_jumping, Value::ZERO)
            .unwrap();
        if wh_status > Value::FLOAT_TOLERANCE {
            return Ok(false);
        }
        Ok(true)
    }
    pub(in crate::svc) fn get_stat_item_can_jump_drive(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<bool, StatItemCheckError> {
        check_ship_no_struct(ctx.u_data, item_uid)?;
        // Jumping (with a jump drive) is blocked by either of:
        // - standard warp scram status attribute
        // - standard drive jump status attribute (disruptive lance, it controls both drive jumps and gate
        //   jumps)
        // - custom drive jump status attribute (bubbles)
        let warp_status = calc
            .get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().warp_scramble_status, Value::ZERO)
            .unwrap();
        if warp_status > Value::FLOAT_TOLERANCE {
            return Ok(false);
        }
        let jump_status = calc
            .get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().disallow_drive_jumping, Value::ZERO)
            .unwrap();
        if jump_status > Value::FLOAT_TOLERANCE {
            return Ok(false);
        }
        let jump_status = calc
            .get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().disallow_drive_jumping_only, Value::ZERO)
            .unwrap();
        if jump_status > Value::FLOAT_TOLERANCE {
            return Ok(false);
        }
        Ok(true)
    }
    pub(in crate::svc) fn get_stat_item_can_dock_station(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<bool, StatItemCheckError> {
        let ship = check_ship_no_struct(ctx.u_data, item_uid)?;
        // Station docking is blocked by either of:
        // - having any aggro effects active
        // - standard dock status attribute (scripted HIC ray)
        if self.is_fit_aggroed(ship.get_fit_uid()) {
            return Ok(false);
        }
        let dock_status = calc
            .get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().disallow_docking, Value::ZERO)
            .unwrap();
        if dock_status > Value::FLOAT_TOLERANCE {
            return Ok(false);
        }
        Ok(true)
    }
    pub(in crate::svc) fn get_stat_item_can_dock_citadel(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<bool, StatItemCheckError> {
        let ship = check_ship_no_struct(ctx.u_data, item_uid)?;
        // Citadel docking is blocked by either of:
        // - having any aggro effects active
        // - standard warp scram status attribute
        // - standard dock status attribute (scripted HIC ray)
        if self.is_fit_aggroed(ship.get_fit_uid()) {
            return Ok(false);
        }
        let warp_status = calc
            .get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().warp_scramble_status, Value::ZERO)
            .unwrap();
        if warp_status > Value::FLOAT_TOLERANCE {
            return Ok(false);
        }
        let dock_status = calc
            .get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().disallow_docking, Value::ZERO)
            .unwrap();
        if dock_status > Value::FLOAT_TOLERANCE {
            return Ok(false);
        }
        Ok(true)
    }
    pub(in crate::svc) fn get_stat_item_can_tether(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<bool, StatItemCheckError> {
        let ship = check_ship_no_struct(ctx.u_data, item_uid)?;
        // Tether is blocked by either of:
        // - having any aggro effects active
        // - any drones or fighters being outside
        // - standard warp scram status attribute
        // - standard tether status attribute
        let fit_data = self.fit_datas.get(&ship.get_fit_uid()).unwrap();
        if !fit_data.aggro_effects.is_empty() {
            return Ok(false);
        }
        if fit_data.get_launched_drone_count() > Count::ZERO || fit_data.get_launched_fighter_count() > Count::ZERO {
            return Ok(false);
        }
        let warp_status = calc
            .get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().warp_scramble_status, Value::ZERO)
            .unwrap();
        if warp_status > Value::FLOAT_TOLERANCE {
            return Ok(false);
        }
        let tether_status = calc
            .get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().disallow_tethering, Value::ZERO)
            .unwrap();
        if tether_status > Value::FLOAT_TOLERANCE {
            return Ok(false);
        }
        Ok(true)
    }
    fn is_fit_aggroed(&self, fit_uid: UFitId) -> bool {
        !self.fit_datas.get(&fit_uid).unwrap().aggro_effects.is_empty()
    }
}
