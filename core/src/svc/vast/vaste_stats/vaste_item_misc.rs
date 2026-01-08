use super::item_checks::{check_character, check_fighter_ship_no_struct, check_ship_no_struct};
use crate::{
    misc::{Count, PValue, Value},
    svc::{SvcCtx, calc::Calc, err::StatItemCheckError, vast::Vast},
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
        // - warp scram status
        // - special attribute which disallows warping and jumping
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
            .get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().disallow_warping_jumping, Value::ZERO)
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
        // - gate scram status (scripted HIC ray)
        // - special attribute which disallows docking (disruptive lance)
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
            .get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().disallow_docking, Value::ZERO)
            .unwrap();
        if dock_status > Value::FLOAT_TOLERANCE {
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
        // - warp scram status
        // - special attribute which disallows jumping
        // - special attribute which disallows warping and jumping
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
        let warp_jump_status = calc
            .get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().disallow_warping_jumping, Value::ZERO)
            .unwrap();
        if warp_jump_status > Value::FLOAT_TOLERANCE {
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
        // - special attribute which disallows docking (scripted HIC ray)
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
        // - scramble status
        // - special attribute which disallows docking
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
        // - warp scram status
        // - special attribute which disallows tethering
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
