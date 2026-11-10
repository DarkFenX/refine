use crate::{
    rd::RItemListId,
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, CtxModifier, Location, RawModId, RawModTemplates, RawModifier,
            registers::{
                RawModStorage, RegOp, process_entry,
                standard::{
                    data::{StandardRegister, StandardRegisterCtxMods},
                    modifier::func::process_cmod,
                },
            },
        },
    },
    ud::{UFitId, UFleet, UItem, UItemId, UShip},
    util::RMapRSet,
};

impl StandardRegister {
    pub(in crate::svc::calc) fn process_fleet_buff_mod<const OP: RegOp>(
        &mut self,
        reuse_cmods: &mut Vec<CtxModifier>,
        ctx: &SvcCtx,
        item: &UItem,
        rmod_id: RawModId,
        rmod: RawModifier,
    ) -> bool {
        let Some(fit_uid) = item.get_fit_uid() else {
            return false;
        };
        // Check validity and handle buffs on fit which carries it
        let valid = process_fleet_mod_with_fit_uid::<OP>(reuse_cmods, ctx, &mut self.cmods, rmod_id, rmod, fit_uid);
        if valid {
            // If buff is valid, handle it on the rest of the fleet
            let fit = ctx.u_data.fits.get(fit_uid);
            if let Some(fleet_uid) = fit.fleet {
                let fleet = ctx.u_data.fleets.get(fleet_uid);
                for fleet_fit_uid in fleet.iter_fits() {
                    if fleet_fit_uid == fit_uid {
                        continue;
                    }
                    process_fleet_mod_with_fit_uid::<OP>(
                        reuse_cmods,
                        ctx,
                        &mut self.cmods,
                        rmod_id,
                        rmod,
                        fleet_fit_uid,
                    );
                }
            }
            process_entry::<OP, _, _>(&mut self.rmods_fleet, fit_uid, rmod_id);
        }
        valid
    }
    pub(in crate::svc::calc) fn process_fleet_for_fit<const OP: RegOp>(
        &mut self,
        reuse_cmods: &mut Vec<CtxModifier>,
        ctx: &SvcCtx,
        rmods: &RawModStorage,
        fleet: &UFleet,
        fit_uid: UFitId,
    ) {
        // Outgoing fleet boosts
        for &rmod_id in self.rmods_fleet.get(&fit_uid) {
            let rmod = rmods.get_rmod(ctx.rmt(), rmod_id);
            for fleet_fit_uid in fleet.iter_fits() {
                if fleet_fit_uid != fit_uid {
                    process_fleet_mod_with_fit_uid::<OP>(
                        reuse_cmods,
                        ctx,
                        &mut self.cmods,
                        rmod_id,
                        rmod,
                        fleet_fit_uid,
                    );
                }
            }
        }
        // Incoming fleet boosts
        for fleet_fit_uid in fleet.iter_fits() {
            if fleet_fit_uid == fit_uid {
                continue;
            }
            for &rmod_id in self.rmods_fleet.get(&fleet_fit_uid) {
                let rmod = rmods.get_rmod(ctx.rmt(), rmod_id);
                process_fleet_mod_with_fit_uid::<OP>(reuse_cmods, ctx, &mut self.cmods, rmod_id, rmod, fit_uid);
            }
        }
    }
    pub(in crate::svc::calc::registers::standard) fn process_affectee_for_fleet<const OP: RegOp>(
        &mut self,
        ctx: &SvcCtx,
        rmods: &RawModStorage,
        ship_uid: UItemId,
        ship: &UShip,
    ) {
        let fit_uid = ship.get_fit_uid();
        let fit = ctx.u_data.fits.get(fit_uid);
        match fit.fleet {
            Some(fleet_uid) => {
                let fleet = ctx.u_data.fleets.get(fleet_uid);
                for fleet_fit_uid in fleet.iter_fits() {
                    let fleet_rmods = get_rmods_with_ids(ctx.rmt(), rmods, &self.rmods_fleet, fleet_fit_uid);
                    process_fleet_mods_for_ship::<OP>(&mut self.cmods, fleet_rmods, ship_uid, ship);
                }
            }
            None => {
                let fleet_rmods = get_rmods_with_ids(ctx.rmt(), rmods, &self.rmods_fleet, fit_uid);
                process_fleet_mods_for_ship::<OP>(&mut self.cmods, fleet_rmods, ship_uid, ship);
            }
        }
    }
}

fn get_rmods_with_ids(
    templates: &RawModTemplates,
    rmods: &RawModStorage,
    rmods_fleet: &RMapRSet<UFitId, RawModId>,
    fit_uid: UFitId,
) -> impl ExactSizeIterator<Item = (RawModId, RawModifier)> {
    rmods_fleet
        .get(&fit_uid)
        .map(|&rmod_id| (rmod_id, rmods.get_rmod(templates, rmod_id)))
}
fn is_fit_ship_on_fleet_item_list<'u>(
    ctx: &SvcCtx<'u, '_>,
    fit_uid: UFitId,
    item_list_rid: RItemListId,
) -> Option<(UItemId, &'u UShip)> {
    let fit = ctx.u_data.fits.get(fit_uid);
    let ship_uid = fit.ship?;
    let ship = ctx.u_data.items.get(ship_uid).dc_ship().unwrap();
    match is_ship_on_fleet_item_list(ship, item_list_rid) {
        true => Some((ship_uid, ship)),
        false => None,
    }
}
fn is_ship_on_fleet_item_list(ship: &UShip, item_list_rid: RItemListId) -> bool {
    match ship.get_r_item_base() {
        Some(ship_rib) => ship_rib.fleet_buff_item_list_rids.contains(&item_list_rid),
        None => false,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Regular registration functions
////////////////////////////////////////////////////////////////////////////////////////////////////
fn process_fleet_mod_with_fit_uid<const OP: RegOp>(
    reuse_cmods: &mut Vec<CtxModifier>,
    ctx: &SvcCtx,
    reg_cmods: &mut StandardRegisterCtxMods,
    rmod_id: RawModId,
    rmod: RawModifier,
    fit_uid: UFitId,
) -> bool {
    match rmod.affectee_filter {
        AffecteeFilter::Direct(Location::ItemList(item_list_rid)) => {
            if let Some((ship_uid, _)) = is_fit_ship_on_fleet_item_list(ctx, fit_uid, item_list_rid) {
                let cmod = CtxModifier::with_item(rmod, ship_uid);
                let key = (ship_uid, cmod.rmod.affectee_attr_rid);
                process_cmod::<OP, _>(key, rmod_id, &cmod, &mut reg_cmods.direct, &mut reg_cmods.by_aspec);
                reuse_cmods.push(cmod);
            }
            true
        }
        AffecteeFilter::Loc(Location::ItemList(item_list_rid)) => {
            if let Some((ship_uid, ship)) = is_fit_ship_on_fleet_item_list(ctx, fit_uid, item_list_rid)
                && let Ok(loc_kind) = ship.get_ship_kind().try_into()
            {
                let cmod = CtxModifier::with_fit_item(rmod, fit_uid, ship_uid);
                let key = (fit_uid, loc_kind, cmod.rmod.affectee_attr_rid);
                process_cmod::<OP, _>(key, rmod_id, &cmod, &mut reg_cmods.loc, &mut reg_cmods.by_aspec);
                reuse_cmods.push(cmod);
            }
            true
        }
        AffecteeFilter::LocGrp(Location::ItemList(item_list_rid), item_grp_id) => {
            if let Some((ship_uid, ship)) = is_fit_ship_on_fleet_item_list(ctx, fit_uid, item_list_rid)
                && let Ok(loc_kind) = ship.get_ship_kind().try_into()
            {
                let cmod = CtxModifier::with_fit_item(rmod, fit_uid, ship_uid);
                let key = (fit_uid, loc_kind, item_grp_id, cmod.rmod.affectee_attr_rid);
                process_cmod::<OP, _>(key, rmod_id, &cmod, &mut reg_cmods.loc_grp, &mut reg_cmods.by_aspec);
                reuse_cmods.push(cmod);
            }
            true
        }
        AffecteeFilter::LocSrq(Location::ItemList(item_list_rid), srq_type_aid) => {
            if let Some((ship_uid, ship)) = is_fit_ship_on_fleet_item_list(ctx, fit_uid, item_list_rid)
                && let Ok(loc_kind) = ship.get_ship_kind().try_into()
            {
                let cmod = CtxModifier::with_fit_item(rmod, fit_uid, ship_uid);
                let key = (fit_uid, loc_kind, srq_type_aid, cmod.rmod.affectee_attr_rid);
                process_cmod::<OP, _>(key, rmod_id, &cmod, &mut reg_cmods.loc_srq, &mut reg_cmods.by_aspec);
                reuse_cmods.push(cmod);
            }
            true
        }
        _ => false,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Apply-unapply functions for ship changes
////////////////////////////////////////////////////////////////////////////////////////////////////
fn process_fleet_mods_for_ship<const OP: RegOp>(
    reg_cmods: &mut StandardRegisterCtxMods,
    fleet_rmods: impl Iterator<Item = (RawModId, RawModifier)>,
    ship_uid: UItemId,
    ship: &UShip,
) {
    let fit_uid = ship.get_fit_uid();
    for (rmod_id, rmod) in fleet_rmods {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(Location::ItemList(item_list_rid))
                if is_ship_on_fleet_item_list(ship, item_list_rid) =>
            {
                let cmod = CtxModifier::with_item(rmod, ship_uid);
                let key = (ship_uid, cmod.rmod.affectee_attr_rid);
                process_cmod::<OP, _>(key, rmod_id, &cmod, &mut reg_cmods.direct, &mut reg_cmods.by_aspec);
            }
            AffecteeFilter::Loc(Location::ItemList(item_list_rid))
                if is_ship_on_fleet_item_list(ship, item_list_rid)
                    && let Ok(loc_kind) = ship.get_ship_kind().try_into() =>
            {
                let cmod = CtxModifier::with_fit_item(rmod, fit_uid, ship_uid);
                let key = (fit_uid, loc_kind, cmod.rmod.affectee_attr_rid);
                process_cmod::<OP, _>(key, rmod_id, &cmod, &mut reg_cmods.loc, &mut reg_cmods.by_aspec);
            }
            AffecteeFilter::LocGrp(Location::ItemList(item_list_rid), item_grp_id)
                if is_ship_on_fleet_item_list(ship, item_list_rid)
                    && let Ok(loc_kind) = ship.get_ship_kind().try_into() =>
            {
                let cmod = CtxModifier::with_fit_item(rmod, fit_uid, ship_uid);
                let key = (fit_uid, loc_kind, item_grp_id, cmod.rmod.affectee_attr_rid);
                process_cmod::<OP, _>(key, rmod_id, &cmod, &mut reg_cmods.loc_grp, &mut reg_cmods.by_aspec);
            }
            AffecteeFilter::LocSrq(Location::ItemList(item_list_rid), srq_type_aid)
                if is_ship_on_fleet_item_list(ship, item_list_rid)
                    && let Ok(loc_kind) = ship.get_ship_kind().try_into() =>
            {
                let cmod = CtxModifier::with_fit_item(rmod, fit_uid, ship_uid);
                let key = (fit_uid, loc_kind, srq_type_aid, cmod.rmod.affectee_attr_rid);
                process_cmod::<OP, _>(key, rmod_id, &cmod, &mut reg_cmods.loc_srq, &mut reg_cmods.by_aspec);
            }
            _ => (),
        }
    }
}
