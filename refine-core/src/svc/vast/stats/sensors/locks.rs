use crate::{
    Count, Value,
    svc::{Calc, SvcCtx, Vast, err::IntStatItemError, vast::stats::item_checks::check_drone_fighter_ship},
    ud::{UItem, UItemId, UShipKind},
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_locks(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<Count, IntStatItemError<!>> {
        check_drone_fighter_ship(ctx.u_data, item_uid)?;
        let attr_consts = ctx.ac();
        let mut item_locks = calc.get_item_oattr_ffb_extra(ctx, item_uid, attr_consts.max_locked_targets, Value::ZERO);
        // Ship (ship kind) locks are limited by character locks. Anything else, including
        // structures, drones and fighter are not limited by it
        let u_item = ctx.u_data.items.get(item_uid);
        if let UItem::Ship(u_ship) = u_item
            && let UShipKind::Ship = u_ship.get_ship_kind()
        {
            let u_fit = ctx.u_data.fits.get(u_ship.get_fit_uid());
            // No limit when no character or it is not loaded
            if let Some(character_uid) = u_fit.character
                && let Some(character_locks) =
                    calc.get_item_oattr_afb_oextra(ctx, character_uid, attr_consts.max_locked_targets, Value::ZERO)
            {
                item_locks = item_locks.min(character_locks)
            }
        }
        // Non-integer locks can happen in Pochven where locks are halved, halves are rounded up
        Ok(Count::from_value_rounded(item_locks))
    }
}
