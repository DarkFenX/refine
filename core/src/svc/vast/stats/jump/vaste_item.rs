use super::{
    option::StatJumpRange,
    stat::{StatJump, StatJumpConduit, StatJumpPassenger, StatJumpPortal, StatJumpSelf},
};
use crate::{
    ad::AAttrId,
    api::ItemTypeId,
    num::{Count, PValue, Value},
    rd::RAttrId,
    svc::{
        SvcCtx,
        calc::Calc,
        err::StatItemCheckError,
        funcs,
        vast::{Vast, stats::item_checks::check_ship},
    },
    ud::{UFit, UFitId, UItemId, UShip, UShipKind},
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_jump(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        range: StatJumpRange,
        psg_fit_uids: &[UFitId],
    ) -> Result<Option<StatJump>, StatItemCheckError> {
        let ship = check_ship(ctx.u_data, item_uid)?;
        Ok(self.internal_get_stat_item_jump_unchecked(ctx, calc, item_uid, ship, range, psg_fit_uids))
    }
    fn internal_get_stat_item_jump_unchecked(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        ship_uid: UItemId,
        ship: &UShip,
        range: StatJumpRange,
        psg_fit_uids: &[UFitId],
    ) -> Option<StatJump> {
        let fuel_type_id = ItemTypeId::from_aid(ship.get_axt().unwrap().jump_fuel_type_id?);
        let max_range = PValue::from_value_clamped(calc.get_item_oattr_ffb_extra(
            ctx,
            ship_uid,
            ctx.ac().jump_drive_range,
            Value::ZERO,
        ));
        if max_range < PValue::FLOAT_TOLERANCE {
            return None;
        }
        let mut stat = StatJump {
            max_range,
            fuel_type_id,
            jump_self: None,
            jump_conduit: None,
            jump_portals: Vec::new(),
        };
        let range = match range {
            // If requested range was higher than item allows, exclude all the jump stats besides
            // basic ones
            StatJumpRange::LightYears(range) => match range > max_range + PValue::FLOAT_TOLERANCE {
                true => return Some(stat),
                false => range,
            },
            StatJumpRange::Max => max_range,
        };
        // Make self-jump stat for all ships with jump drive, except of structure kind, not to
        // expose self-jump stats for ansiblexes
        if !matches!(ship.get_kind(), UShipKind::Structure) {
            stat.jump_self = Some(get_stat_jump_self(ctx, calc, ship_uid, range));
        }
        let fit_data = self.get_fit_data(ship.get_fit_uid());
        // Expose conduit stats only if fit has any conduit enablers (ship or online portals)
        if !fit_data.conduit_enablers.is_empty() {
            stat.jump_conduit = Some(get_stat_jump_conduit(ctx, calc, ship_uid, range, psg_fit_uids));
        }
        // Expose portal stats, one entry per portal enabler, since some properties depend on portal
        for &portal_uid in fit_data.portal_enablers.iter() {
            stat.jump_portals.push(get_stat_jump_portal(
                ctx,
                calc,
                ship_uid,
                portal_uid,
                range,
                psg_fit_uids,
            ));
        }
        Some(stat)
    }
}

// Higher level jump fetchers
fn get_stat_jump_self(ctx: SvcCtx, calc: &mut Calc, ship_uid: UItemId, range: PValue) -> StatJumpSelf {
    let self_fuel_need =
        calc.get_item_oattr_ffb_extra(ctx, ship_uid, ctx.ac().jump_drive_consumption_amount, Value::ZERO);
    StatJumpSelf {
        fuel_use: Count::from_value_ceiled(self_fuel_need * range),
    }
}
fn get_stat_jump_conduit(
    ctx: SvcCtx,
    calc: &mut Calc,
    ship_uid: UItemId,
    range: PValue,
    psg_fit_uids: &[UFitId],
) -> StatJumpConduit {
    let conduit_fuel_need = calc.get_item_oattr_ffb_extra(
        ctx,
        ship_uid,
        ctx.ac().conduit_jump_drive_consumption_amount,
        Value::ZERO,
    );
    let max_psgs = Count::from_value_rounded(calc.get_item_oattr_ffb_extra(
        ctx,
        ship_uid,
        ctx.ac().conduit_jump_passenger_count,
        Value::ZERO,
    ));
    let mut psgs = Vec::with_capacity(psg_fit_uids.len());
    if !psg_fit_uids.is_empty() {
        let psg_check = get_psg_check(ctx, calc, ship_uid, ctx.ac().jump_conduit_passenger_required_attr_id);
        for &psg_fit_uid in psg_fit_uids {
            let psg_u_fit = ctx.u_data.fits.get(psg_fit_uid);
            let fuel_use = is_psg(ctx, calc, psg_u_fit, psg_check).map(|_| Count::ZERO);
            psgs.push(StatJumpPassenger {
                fit_id: psg_u_fit.id,
                fuel_use,
            });
        }
    }
    StatJumpConduit {
        max_passengers: max_psgs,
        fuel_use_self: Count::from_value_ceiled(conduit_fuel_need * range),
        fuel_use_passengers: psgs,
    }
}
fn get_stat_jump_portal(
    ctx: SvcCtx,
    calc: &mut Calc,
    ship_uid: UItemId,
    portal_uid: UItemId,
    range: PValue,
    psg_fit_uids: &[UFitId],
) -> StatJumpPortal {
    let mut psgs = Vec::with_capacity(psg_fit_uids.len());
    if !psg_fit_uids.is_empty() {
        let mass_limit = match PValue::from_value_clamped(calc.get_item_oattr_ffb_extra(
            ctx,
            ship_uid,
            ctx.ac().gate_max_jump_mass,
            Value::ZERO,
        )) {
            PValue::ZERO => None,
            mass_limit => Some(mass_limit),
        };
        let ship_fuel_use = PValue::from_value_clamped(calc.get_item_oattr_ffb_extra(
            ctx,
            ship_uid,
            ctx.ac().jump_drive_consumption_amount,
            Value::ZERO,
        ));
        let mass_mult = PValue::from_value_clamped(calc.get_item_oattr_ffb_extra(
            ctx,
            ship_uid,
            ctx.ac().jump_portal_consumption_mass_factor,
            Value::ZERO,
        ));
        let fuel_use_add = PValue::from_value_clamped(calc.get_item_oattr_ffb_extra(
            ctx,
            ship_uid,
            ctx.ac().jump_portal_additional_consumption,
            Value::ZERO,
        ));
        let fuel_use_base = ship_fuel_use * range * mass_mult;
        let psg_check = get_psg_check(ctx, calc, ship_uid, ctx.ac().jump_conduit_passenger_required_attr_id);
        for &psg_fit_uid in psg_fit_uids {
            let psg_u_fit = ctx.u_data.fits.get(psg_fit_uid);
            let fuel_use = match is_psg(ctx, calc, psg_u_fit, psg_check) {
                Some(psg_ship_uid) => {
                    let psg_mass = Vast::internal_get_stat_item_mass_unchecked(ctx, calc, psg_ship_uid);
                    match mass_limit {
                        Some(mass_limit) if psg_mass > mass_limit => None,
                        _ => Some(Count::from_pvalue_ceiled(fuel_use_base.mul_add(psg_mass, fuel_use_add))),
                    }
                }
                None => None,
            };
            psgs.push(StatJumpPassenger {
                fit_id: psg_u_fit.id,
                fuel_use,
            });
        }
    }
    StatJumpPortal {
        item_id: ctx.u_data.items.get(portal_uid).get_item_id(),
        fuel_use_passengers: psgs,
    }
}

#[derive(Copy, Clone)]
enum PassengerCheck {
    Allow,
    Disallow,
    CheckFlag(RAttrId),
}
fn get_psg_check(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId, ref_attr_rid: Option<RAttrId>) -> PassengerCheck {
    // No reference attribute - no requirement - allow all passengers
    let Some(ref_attr_rid) = ref_attr_rid else {
        return PassengerCheck::Allow;
    };
    // Unwrap, since we already checked that ship is loaded
    let ref_val = calc.get_item_attr_oextra(ctx, item_uid, ref_attr_rid).unwrap();
    // Cannot build reference - no requirement - allow all passengers
    let Some(psg_attr_aid) = AAttrId::try_eve_from_f64_rounded(ref_val.into_f64()) else {
        return PassengerCheck::Allow;
    };
    match ctx.u_data.r_data.get_attr_rid_by_aid(&psg_attr_aid) {
        Some(psg_attr_rid) => PassengerCheck::CheckFlag(psg_attr_rid),
        // There is a requirement, but target attribute does not exist - disallow all passengers
        None => PassengerCheck::Disallow,
    }
}
fn is_psg(ctx: SvcCtx, calc: &mut Calc, psg_fit: &UFit, psg_check: PassengerCheck) -> Option<UItemId> {
    // No passenger ship - cannot be a passenger regardless of what check says
    let psg_ship_uid = psg_fit.ship?;
    // Passenger ship is not loaded - cannot be a passenger as well
    if !ctx.u_data.items.get(psg_ship_uid).is_loaded() {
        return None;
    }
    match psg_check {
        PassengerCheck::Allow => Some(psg_ship_uid),
        PassengerCheck::Disallow => None,
        PassengerCheck::CheckFlag(psg_attr_rid) => {
            match funcs::is_attr_flag_set(ctx, calc, psg_ship_uid, psg_attr_rid) {
                Some(true) => Some(psg_ship_uid),
                _ => None,
            }
        }
    }
}
