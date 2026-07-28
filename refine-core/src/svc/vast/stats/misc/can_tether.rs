use crate::{
    Count, Value,
    svc::{Calc, SvcCtx, Vast, err::IntItemStatError, vast::stats::item_checks::check_ship_no_struct},
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_can_tether(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<bool, IntItemStatError<!>> {
        let ship = check_ship_no_struct(ctx.u_data, item_uid)?;
        // Tether is blocked by either of:
        // - having any running effects which give weapons timer
        // - any released fighters
        // - standard warp scram status attribute
        // - standard tether status attribute
        // - having any modules with effects which disable tethering (cloaks)
        let fit_data = self.get_fit_data(ship.get_fit_uid());
        if !fit_data.effects_weapons_timer.is_empty() {
            return Ok(false);
        }
        if !fit_data.mod_effects_disallow_tether.is_empty() {
            return Ok(false);
        }
        if fit_data.get_launched_fighter_count() > Count::ZERO {
            return Ok(false);
        }
        let tether_status = calc.get_item_oattr_ffb_extra(ctx, item_uid, ctx.ac().disallow_tethering, Value::ZERO);
        if tether_status > Value::FLOAT_TOLERANCE {
            return Ok(false);
        }
        // First, check if ship is scrambled (which is a relatively fast check, since it can use
        // cached attribute value). If ship is scrambled, calculate scram status value again, this
        // time excluding effects whose scram status modification is supposed to be excluded from
        // the tether check. This operation is more expensive, since it cannot use cached attribute
        // value.
        let warp_status = calc.get_item_oattr_ffb_extra(ctx, item_uid, ctx.ac().warp_scramble_status, Value::ZERO);
        if warp_status > Value::FLOAT_TOLERANCE {
            let warp_status = calc.get_item_oattr_ffb_extra_filtered(
                ctx,
                item_uid,
                ctx.ac().warp_scramble_status,
                |espec| {
                    !ctx.u_data
                        .r_data
                        .get_effect_by_rid(espec.effect_rid)
                        .do_not_prevent_tether
                },
                Value::ZERO,
            );
            if warp_status > Value::FLOAT_TOLERANCE {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
