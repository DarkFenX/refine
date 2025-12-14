use super::checks::{
    check_drone_fighter_ship, check_drone_fighter_ship_no_struct, check_fighter_ship_no_struct, check_ship_no_struct,
};
use crate::{
    def::{AttrVal, OF},
    svc::{SvcCtx, calc::Calc, err::StatItemCheckError, item_funcs, vast::Vast},
    ud::UItemKey,
    util::ceil_tick,
};

// Result of calculation of -math.log(0.25) / 1000000 using 64-bit python 2.7
pub(super) const AGILITY_CONST: AttrVal = OF(f64::from_bits(0x3eb74216c502a54f));

impl Vast {
    pub(in crate::svc) fn get_stat_item_speed(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        check_drone_fighter_ship_no_struct(ctx.u_data, item_key)?;
        let speed = item_funcs::get_speed(ctx, calc, item_key);
        Ok(speed)
    }
    pub(in crate::svc) fn get_stat_item_agility(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<Option<AttrVal>, StatItemCheckError> {
        check_drone_fighter_ship_no_struct(ctx.u_data, item_key)?;
        Ok(Vast::internal_get_stat_item_agility_unchecked(ctx, calc, item_key))
    }
    fn internal_get_stat_item_agility_unchecked(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> Option<AttrVal> {
        let attr_consts = ctx.ac();
        let agility = calc
            .get_item_oattr_afb_oextra(ctx, item_key, attr_consts.agility, OF(0.0))
            .unwrap();
        if agility <= OF(0.0) {
            return None;
        }
        let mass = calc
            .get_item_oattr_afb_oextra(ctx, item_key, attr_consts.mass, OF(0.0))
            .unwrap();
        if mass <= OF(0.0) {
            return None;
        }
        Some(AGILITY_CONST * agility * mass)
    }
    pub(in crate::svc) fn get_stat_item_align_time(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<Option<AttrVal>, StatItemCheckError> {
        check_drone_fighter_ship_no_struct(ctx.u_data, item_key)?;
        let agility = Vast::internal_get_stat_item_agility_unchecked(ctx, calc, item_key);
        let align_time = agility.map(ceil_tick);
        Ok(align_time)
    }
    pub(in crate::svc) fn get_stat_item_sig_radius(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        check_drone_fighter_ship(ctx.u_data, item_key)?;
        Ok(Vast::internal_get_stat_item_sig_radius_unchecked(ctx, calc, item_key))
    }
    pub(super) fn internal_get_stat_item_sig_radius_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> AttrVal {
        item_funcs::get_sig_radius(ctx, calc, item_key)
    }
    pub(in crate::svc) fn get_stat_item_mass(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        check_drone_fighter_ship(ctx.u_data, item_key)?;
        Ok(Vast::internal_get_stat_item_mass_unchecked(ctx, calc, item_key))
    }
    fn internal_get_stat_item_mass_unchecked(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> AttrVal {
        calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().mass, OF(0.0))
            .unwrap()
    }
    pub(in crate::svc) fn get_stat_item_warp_speed(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<Option<AttrVal>, StatItemCheckError> {
        check_fighter_ship_no_struct(ctx.u_data, item_key)?;
        let warp_speed = calc
            .get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().warp_speed_mult, OF(0.0))
            .unwrap();
        let warp_speed = match warp_speed > OF(0.0) {
            true => Some(warp_speed),
            false => None,
        };
        Ok(warp_speed)
    }
    pub(in crate::svc) fn get_stat_item_max_warp_range(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<Option<AttrVal>, StatItemCheckError> {
        check_ship_no_struct(ctx.u_data, item_key)?;
        let cap = Vast::internal_get_stat_item_cap_unchecked(ctx, calc, item_key);
        let mass = Vast::internal_get_stat_item_mass_unchecked(ctx, calc, item_key);
        let cap_need = calc
            .get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().warp_capacitor_need, OF(0.0))
            .unwrap();
        let warp_range = cap / mass / cap_need;
        let warp_range = match warp_range.is_finite() && warp_range > OF(0.0) {
            true => Some(warp_range),
            false => None,
        };
        Ok(warp_range)
    }
}
