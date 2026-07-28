use crate::{
    svc::{SvcCtx, Vast, err::IntItemStatError, vast::stats::item_checks::check_ship_no_struct},
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_can_jump_wormhole(
        &self,
        ctx: SvcCtx,
        item_uid: UItemId,
    ) -> Result<bool, IntItemStatError<!>> {
        let ship = check_ship_no_struct(ctx.u_data, item_uid)?;
        // WH jumping is blocked by:
        // - type ID being on type list 245 WH jump black list (supercapitals)
        // - having any modules with effects which disable WH jumping (cloaks, MJDs, sieges)
        if ship.get_disallowed_in_wspace() == Some(true) {
            return Ok(false);
        }
        let fit_data = self.get_fit_data(ship.get_fit_uid());
        if !fit_data.mod_effects_disallow_jump_wh.is_empty() {
            return Ok(false);
        }
        Ok(true)
    }
}
