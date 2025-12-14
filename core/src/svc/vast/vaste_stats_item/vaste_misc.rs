use super::checks::{check_character, check_fighter_ship_no_struct, check_ship_no_struct};
use crate::{
    def::{AttrVal, OF},
    svc::{SvcCtx, calc::Calc, err::StatItemCheckError, vast::Vast},
    ud::{UFitKey, UItemKey},
    util::FLOAT_TOLERANCE,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_drone_control_range(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        check_character(ctx.u_data, item_key)?;
        let drone_control_range = calc
            .get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().drone_control_distance, OF(0.0))
            .unwrap();
        Ok(drone_control_range)
    }
    pub(in crate::svc) fn get_stat_item_can_warp(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<bool, StatItemCheckError> {
        check_fighter_ship_no_struct(ctx.u_data, item_key)?;
        // Warping is blocked by either of:
        // - warp scram status
        // - having no max velocity
        let warp_status = calc
            .get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().warp_scramble_status, OF(0.0))
            .unwrap();
        if warp_status > FLOAT_TOLERANCE {
            return Ok(false);
        }
        // Do not block by velocity requirement if attribute is not defined
        if let Some(max_speed) = calc.get_item_oattr_oextra(ctx, item_key, ctx.ac().max_velocity)
            && max_speed < FLOAT_TOLERANCE
        {
            return Ok(false);
        }
        Ok(true)
    }
    pub(in crate::svc) fn get_stat_item_can_jump_gate(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<bool, StatItemCheckError> {
        let ship = check_ship_no_struct(ctx.u_data, item_key)?;
        // Gating is blocked by either of:
        // - having aggro modules active
        // - gate scram status (scripted HIC ray)
        // - special attribute which disallows docking (disruptive lance)
        if self.is_fit_aggroed(ship.get_fit_key()) {
            return Ok(false);
        }
        let gate_status = calc
            .get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().gate_scramble_status, OF(0.0))
            .unwrap();
        if gate_status > FLOAT_TOLERANCE {
            return Ok(false);
        }
        let dock_status = calc
            .get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().disallow_docking, OF(0.0))
            .unwrap();
        if dock_status > FLOAT_TOLERANCE {
            return Ok(false);
        }
        Ok(true)
    }
    pub(in crate::svc) fn get_stat_item_can_jump_drive(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<bool, StatItemCheckError> {
        check_ship_no_struct(ctx.u_data, item_key)?;
        // Jumping (with a jump drive) is blocked by either of:
        // - warp scram status
        // - special attribute which disallows jumping
        let warp_status = calc
            .get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().warp_scramble_status, OF(0.0))
            .unwrap();
        if warp_status > FLOAT_TOLERANCE {
            return Ok(false);
        }
        let jump_status = calc
            .get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().disallow_drive_jumping, OF(0.0))
            .unwrap();
        if jump_status > FLOAT_TOLERANCE {
            return Ok(false);
        }
        Ok(true)
    }
    pub(in crate::svc) fn get_stat_item_can_dock_station(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<bool, StatItemCheckError> {
        let ship = check_ship_no_struct(ctx.u_data, item_key)?;
        // Station docking is blocked by either of:
        // - having any aggro effects active
        // - special attribute which disallows docking (scripted HIC ray)
        if self.is_fit_aggroed(ship.get_fit_key()) {
            return Ok(false);
        }
        let dock_status = calc
            .get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().disallow_docking, OF(0.0))
            .unwrap();
        if dock_status > FLOAT_TOLERANCE {
            return Ok(false);
        }
        Ok(true)
    }
    pub(in crate::svc) fn get_stat_item_can_dock_citadel(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<bool, StatItemCheckError> {
        let ship = check_ship_no_struct(ctx.u_data, item_key)?;
        // Citadel docking is blocked by either of:
        // - having any aggro effects active
        // - scramble status
        // - special attribute which disallows docking
        if self.is_fit_aggroed(ship.get_fit_key()) {
            return Ok(false);
        }
        let warp_status = calc
            .get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().warp_scramble_status, OF(0.0))
            .unwrap();
        if warp_status > FLOAT_TOLERANCE {
            return Ok(false);
        }
        let dock_status = calc
            .get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().disallow_docking, OF(0.0))
            .unwrap();
        if dock_status > FLOAT_TOLERANCE {
            return Ok(false);
        }
        Ok(true)
    }
    pub(in crate::svc) fn get_stat_item_can_tether(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<bool, StatItemCheckError> {
        let ship = check_ship_no_struct(ctx.u_data, item_key)?;
        // Tether is blocked by either of:
        // - having any aggro effects active
        // - any drones or fighters being outside
        // - warp scram status
        // - special attribute which disallows tethering
        let fit_data = self.fit_datas.get(&ship.get_fit_key()).unwrap();
        if !fit_data.aggro_effects.is_empty() {
            return Ok(false);
        }
        if fit_data.get_launched_drone_count() > 0 || fit_data.get_launched_fighter_count() > 0 {
            return Ok(false);
        }
        let warp_status = calc
            .get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().warp_scramble_status, OF(0.0))
            .unwrap();
        if warp_status > FLOAT_TOLERANCE {
            return Ok(false);
        }
        let tether_status = calc
            .get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().disallow_tethering, OF(0.0))
            .unwrap();
        if tether_status > FLOAT_TOLERANCE {
            return Ok(false);
        }
        Ok(true)
    }
    fn is_fit_aggroed(&self, fit_key: UFitKey) -> bool {
        !self.fit_datas.get(&fit_key).unwrap().aggro_effects.is_empty()
    }
}
