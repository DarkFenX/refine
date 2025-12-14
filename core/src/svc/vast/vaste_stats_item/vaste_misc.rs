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
        // Warping is blocked by warp scram status and having no max velocity
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
    pub(in crate::svc) fn get_stat_item_can_gate_jump(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<bool, StatItemCheckError> {
        let ship = check_ship_no_struct(ctx.u_data, item_key)?;
        // Gating is blocked by having aggro modules active, gate scram status, and a special
        // docking attribute (which seems to be used by disruptive lances)
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
    pub(in crate::svc) fn get_stat_item_can_drive_jump(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<bool, StatItemCheckError> {
        check_ship_no_struct(ctx.u_data, item_key)?;
        // Jumping is blocked by warp scram status and attribute specific to drive jumping
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
    pub(in crate::svc) fn get_stat_item_can_dock(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<bool, StatItemCheckError> {
        let ship = check_ship_no_struct(ctx.u_data, item_key)?;
        // Docking is blocked by having any aggro effects active and a special attribute
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
    pub(in crate::svc) fn get_stat_item_can_tether(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<bool, StatItemCheckError> {
        let ship = check_ship_no_struct(ctx.u_data, item_key)?;
        // Tether is blocked by having any aggro effects active and a special attribute
        if self.is_fit_aggroed(ship.get_fit_key()) {
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
