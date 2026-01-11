use crate::{
    rd::RItemListId,
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, CtxModifier, Location, RawModifier,
            registers::standard::{
                data::{StandardRegister, StandardRegisterCtxMods},
                modifier::func::{add_cmod, remove_cmod},
            },
        },
    },
    ud::{UFitId, UFleet, UItem, UItemId, UShip},
};

impl StandardRegister {
    pub(in crate::svc::calc) fn reg_fleet_buff_mod(
        &mut self,
        reuse_cmods: &mut Vec<CtxModifier>,
        ctx: SvcCtx,
        item: &UItem,
        rmod: RawModifier,
    ) -> bool {
        reuse_cmods.clear();
        let fit_uid = match item.get_fit_uid() {
            Some(fit_uid) => fit_uid,
            None => return false,
        };
        // Check validity and apply buffs to fit which carries it
        let valid = apply_fleet_mod_with_fit_uid(reuse_cmods, ctx, &mut self.cmods, rmod, fit_uid);
        if valid {
            // If buff is valid, apply to the rest of the fleet
            let fit = ctx.u_data.fits.get(fit_uid);
            if let Some(fleet_uid) = fit.fleet {
                let fleet = ctx.u_data.fleets.get(fleet_uid);
                for fleet_fit_uid in fleet.iter_fits() {
                    if fleet_fit_uid == fit_uid {
                        continue;
                    }
                    apply_fleet_mod_with_fit_uid(reuse_cmods, ctx, &mut self.cmods, rmod, fleet_fit_uid);
                }
            }
            self.rmods_fleet.add_entry(fit_uid, rmod);
            self.rmods_all.add_entry(rmod.affector_espec, rmod);
        }
        valid
    }
    pub(in crate::svc::calc) fn unreg_fleet_buff_mod(
        &mut self,
        reuse_cmods: &mut Vec<CtxModifier>,
        ctx: SvcCtx,
        item: &UItem,
        rmod: RawModifier,
    ) {
        reuse_cmods.clear();
        let fit_uid = match item.get_fit_uid() {
            Some(fit_uid) => fit_uid,
            None => return,
        };
        // Modifiers received by this function are assumed to be valid, so just unapply all
        // modifications and remove modifier from helper container
        let fit = ctx.u_data.fits.get(fit_uid);
        match fit.fleet {
            Some(fleet_uid) => {
                let fleet = ctx.u_data.fleets.get(fleet_uid);
                for fleet_fit_uid in fleet.iter_fits() {
                    unapply_fleet_mod_with_fit_uid(reuse_cmods, ctx, &mut self.cmods, rmod, fleet_fit_uid);
                }
            }
            None => {
                unapply_fleet_mod_with_fit_uid(reuse_cmods, ctx, &mut self.cmods, rmod, fit_uid);
            }
        }
        self.rmods_fleet.remove_entry(fit_uid, &rmod);
    }
    pub(in crate::svc::calc) fn reg_fleet_for_fit(
        &mut self,
        ctx: SvcCtx,
        fleet: &UFleet,
        fit_uid: UFitId,
    ) -> Vec<CtxModifier> {
        let mut cmods = Vec::new();
        // Outgoing fleet boosts
        for rmod in self.rmods_fleet.get(&fit_uid) {
            for fleet_fit_uid in fleet.iter_fits() {
                if fleet_fit_uid != fit_uid {
                    apply_fleet_mod_with_fit_uid(&mut cmods, ctx, &mut self.cmods, *rmod, fleet_fit_uid);
                }
            }
        }
        // Incoming fleet boosts
        for fleet_fit_uid in fleet.iter_fits() {
            if fleet_fit_uid == fit_uid {
                continue;
            }
            for &rmod in self.rmods_fleet.get(&fleet_fit_uid) {
                apply_fleet_mod_with_fit_uid(&mut cmods, ctx, &mut self.cmods, rmod, fit_uid);
            }
        }
        cmods
    }
    pub(in crate::svc::calc) fn unreg_fleet_for_fit(
        &mut self,
        ctx: SvcCtx,
        fleet: &UFleet,
        fit_uid: UFitId,
    ) -> Vec<CtxModifier> {
        let mut cmods = Vec::new();
        // Outgoing fleet boosts
        for rmod in self.rmods_fleet.get(&fit_uid) {
            for fleet_fit_uid in fleet.iter_fits() {
                if fleet_fit_uid != fit_uid {
                    unapply_fleet_mod_with_fit_uid(&mut cmods, ctx, &mut self.cmods, *rmod, fleet_fit_uid);
                }
            }
        }
        // Incoming fleet boosts
        for fleet_fit_uid in fleet.iter_fits() {
            if fleet_fit_uid == fit_uid {
                continue;
            }
            for rmod in self.rmods_fleet.get(&fleet_fit_uid) {
                unapply_fleet_mod_with_fit_uid(&mut cmods, ctx, &mut self.cmods, *rmod, fit_uid);
            }
        }
        cmods
    }
    pub(in crate::svc::calc::registers::standard) fn load_affectee_for_fleet(
        &mut self,
        ctx: SvcCtx,
        ship_uid: UItemId,
        ship: &UShip,
    ) {
        let fit_uid = ship.get_fit_uid();
        let fit = ctx.u_data.fits.get(fit_uid);
        match fit.fleet {
            Some(fleet_uid) => {
                let fleet = ctx.u_data.fleets.get(fleet_uid);
                for fleet_fit_uid in fleet.iter_fits() {
                    let fleet_rmods = self.rmods_fleet.get(&fleet_fit_uid);
                    apply_fleet_mods_to_ship_fit(&mut self.cmods, fleet_rmods, ship_uid, ship);
                }
            }
            None => {
                let fleet_rmods = self.rmods_fleet.get(&fit_uid);
                apply_fleet_mods_to_ship_fit(&mut self.cmods, fleet_rmods, ship_uid, ship);
            }
        }
    }
    pub(in crate::svc::calc::registers::standard) fn unload_affectee_for_fleet(
        &mut self,
        ctx: SvcCtx,
        ship_uid: UItemId,
        ship: &UShip,
    ) {
        let fit_uid = ship.get_fit_uid();
        let fit = ctx.u_data.fits.get(fit_uid);
        match fit.fleet {
            Some(fleet_uid) => {
                let fleet = ctx.u_data.fleets.get(fleet_uid);
                for fleet_fit_uid in fleet.iter_fits() {
                    let fleet_rmods = self.rmods_fleet.get(&fleet_fit_uid);
                    unapply_fleet_mods_from_ship_fit(&mut self.cmods, fleet_rmods, ship_uid, ship);
                }
            }
            None => {
                let fleet_rmods = self.rmods_fleet.get(&fit_uid);
                unapply_fleet_mods_from_ship_fit(&mut self.cmods, fleet_rmods, ship_uid, ship);
            }
        }
    }
}

fn is_fit_ship_on_fleet_item_list<'u>(
    ctx: SvcCtx<'u, '_>,
    fit_uid: UFitId,
    item_list_rid: &RItemListId,
) -> Option<(UItemId, &'u UShip)> {
    let fit = ctx.u_data.fits.get(fit_uid);
    let ship_uid = fit.ship?;
    let ship = ctx.u_data.items.get(ship_uid).dc_ship().unwrap();
    match is_ship_on_fleet_item_list(ship, item_list_rid) {
        true => Some((ship_uid, ship)),
        false => None,
    }
}
fn is_ship_on_fleet_item_list(ship: &UShip, item_list_rid: &RItemListId) -> bool {
    match ship.get_fleet_buff_item_lists() {
        Some(item_list_rids) => item_list_rids.contains(item_list_rid),
        None => false,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Regular apply-unapply functions
////////////////////////////////////////////////////////////////////////////////////////////////////
fn apply_fleet_mod_with_fit_uid(
    reuse_cmods: &mut Vec<CtxModifier>,
    ctx: SvcCtx,
    reg_cmods: &mut StandardRegisterCtxMods,
    rmod: RawModifier,
    fit_uid: UFitId,
) -> bool {
    match rmod.affectee_filter {
        AffecteeFilter::Direct(Location::ItemList(item_list_rid)) => {
            if let Some((ship_uid, _)) = is_fit_ship_on_fleet_item_list(ctx, fit_uid, &item_list_rid) {
                let cmod = CtxModifier::new_with_item(rmod, ship_uid);
                add_cmod(&mut reg_cmods.direct, ship_uid, cmod, &mut reg_cmods.by_aspec);
                reuse_cmods.push(cmod);
            }
            true
        }
        AffecteeFilter::Loc(Location::ItemList(item_list_rid)) => {
            if let Some((ship_uid, ship)) = is_fit_ship_on_fleet_item_list(ctx, fit_uid, &item_list_rid)
                && let Ok(loc_kind) = ship.get_kind().try_into()
            {
                let cmod = CtxModifier::new_with_fit_item(rmod, fit_uid, ship_uid);
                let key = (fit_uid, loc_kind);
                add_cmod(&mut reg_cmods.loc, key, cmod, &mut reg_cmods.by_aspec);
                reuse_cmods.push(cmod);
            }
            true
        }
        AffecteeFilter::LocGrp(Location::ItemList(item_list_rid), item_grp_id) => {
            if let Some((ship_uid, ship)) = is_fit_ship_on_fleet_item_list(ctx, fit_uid, &item_list_rid)
                && let Ok(loc_kind) = ship.get_kind().try_into()
            {
                let cmod = CtxModifier::new_with_fit_item(rmod, fit_uid, ship_uid);
                let key = (fit_uid, loc_kind, item_grp_id);
                add_cmod(&mut reg_cmods.loc_grp, key, cmod, &mut reg_cmods.by_aspec);
                reuse_cmods.push(cmod);
            }
            true
        }
        AffecteeFilter::LocSrq(Location::ItemList(item_list_rid), srq_type_aid) => {
            if let Some((ship_uid, ship)) = is_fit_ship_on_fleet_item_list(ctx, fit_uid, &item_list_rid)
                && let Ok(loc_kind) = ship.get_kind().try_into()
            {
                let cmod = CtxModifier::new_with_fit_item(rmod, fit_uid, ship_uid);
                let key = (fit_uid, loc_kind, srq_type_aid);
                add_cmod(&mut reg_cmods.loc_srq, key, cmod, &mut reg_cmods.by_aspec);
                reuse_cmods.push(cmod);
            }
            true
        }
        _ => false,
    }
}
fn unapply_fleet_mod_with_fit_uid(
    reuse_cmods: &mut Vec<CtxModifier>,
    ctx: SvcCtx,
    reg_cmods: &mut StandardRegisterCtxMods,
    rmod: RawModifier,
    fit_uid: UFitId,
) {
    // We don't check location here, since logic on layers above ensures we receive only
    // modifiers which passed checks when they were added, and location check is part of those
    match rmod.affectee_filter {
        AffecteeFilter::Direct(Location::ItemList(item_list_rid))
            if let Some((ship_uid, _)) = is_fit_ship_on_fleet_item_list(ctx, fit_uid, &item_list_rid) =>
        {
            let cmod = CtxModifier::new_with_item(rmod, ship_uid);
            remove_cmod(&mut reg_cmods.direct, ship_uid, &cmod, &mut reg_cmods.by_aspec);
            reuse_cmods.push(cmod);
        }
        AffecteeFilter::Loc(Location::ItemList(item_list_rid))
            if let Some((ship_uid, ship)) = is_fit_ship_on_fleet_item_list(ctx, fit_uid, &item_list_rid)
                && let Ok(loc_kind) = ship.get_kind().try_into() =>
        {
            let cmod = CtxModifier::new_with_fit_item(rmod, fit_uid, ship_uid);
            let key = (fit_uid, loc_kind);
            remove_cmod(&mut reg_cmods.loc, key, &cmod, &mut reg_cmods.by_aspec);
            reuse_cmods.push(cmod);
        }
        AffecteeFilter::LocGrp(Location::ItemList(item_list_rid), item_grp_id)
            if let Some((ship_uid, ship)) = is_fit_ship_on_fleet_item_list(ctx, fit_uid, &item_list_rid)
                && let Ok(loc_kind) = ship.get_kind().try_into() =>
        {
            let cmod = CtxModifier::new_with_fit_item(rmod, fit_uid, ship_uid);
            let key = (fit_uid, loc_kind, item_grp_id);
            remove_cmod(&mut reg_cmods.loc_grp, key, &cmod, &mut reg_cmods.by_aspec);
            reuse_cmods.push(cmod);
        }
        AffecteeFilter::LocSrq(Location::ItemList(item_list_rid), srq_type_aid)
            if let Some((ship_uid, ship)) = is_fit_ship_on_fleet_item_list(ctx, fit_uid, &item_list_rid)
                && let Ok(loc_kind) = ship.get_kind().try_into() =>
        {
            let cmod = CtxModifier::new_with_fit_item(rmod, fit_uid, ship_uid);
            let key = (fit_uid, loc_kind, srq_type_aid);
            remove_cmod(&mut reg_cmods.loc_srq, key, &cmod, &mut reg_cmods.by_aspec);
            reuse_cmods.push(cmod);
        }
        _ => (),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Apply-unapply functions for ship changes
////////////////////////////////////////////////////////////////////////////////////////////////////
fn apply_fleet_mods_to_ship_fit<'a>(
    reg_cmods: &mut StandardRegisterCtxMods,
    fleet_rmods: impl Iterator<Item = &'a RawModifier>,
    ship_uid: UItemId,
    ship: &UShip,
) {
    let fit_uid = ship.get_fit_uid();
    for rmod in fleet_rmods {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(Location::ItemList(item_list_rid))
                if is_ship_on_fleet_item_list(ship, &item_list_rid) =>
            {
                let cmod = CtxModifier::new_with_item(*rmod, ship_uid);
                add_cmod(&mut reg_cmods.direct, ship_uid, cmod, &mut reg_cmods.by_aspec);
            }
            AffecteeFilter::Loc(Location::ItemList(item_list_rid))
                if is_ship_on_fleet_item_list(ship, &item_list_rid)
                    && let Ok(loc_kind) = ship.get_kind().try_into() =>
            {
                let cmod = CtxModifier::new_with_fit_item(*rmod, fit_uid, ship_uid);
                let key = (fit_uid, loc_kind);
                add_cmod(&mut reg_cmods.loc, key, cmod, &mut reg_cmods.by_aspec);
            }
            AffecteeFilter::LocGrp(Location::ItemList(item_list_rid), item_grp_id)
                if is_ship_on_fleet_item_list(ship, &item_list_rid)
                    && let Ok(loc_kind) = ship.get_kind().try_into() =>
            {
                let cmod = CtxModifier::new_with_fit_item(*rmod, fit_uid, ship_uid);
                let key = (fit_uid, loc_kind, item_grp_id);
                add_cmod(&mut reg_cmods.loc_grp, key, cmod, &mut reg_cmods.by_aspec);
            }
            AffecteeFilter::LocSrq(Location::ItemList(item_list_rid), srq_type_aid)
                if is_ship_on_fleet_item_list(ship, &item_list_rid)
                    && let Ok(loc_kind) = ship.get_kind().try_into() =>
            {
                let cmod = CtxModifier::new_with_fit_item(*rmod, fit_uid, ship_uid);
                let key = (fit_uid, loc_kind, srq_type_aid);
                add_cmod(&mut reg_cmods.loc_srq, key, cmod, &mut reg_cmods.by_aspec);
            }
            _ => (),
        }
    }
}
fn unapply_fleet_mods_from_ship_fit<'a>(
    reg_cmods: &mut StandardRegisterCtxMods,
    fleet_rmods: impl Iterator<Item = &'a RawModifier>,
    ship_uid: UItemId,
    ship: &UShip,
) {
    let fit_uid = ship.get_fit_uid();
    for rmod in fleet_rmods {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(Location::ItemList(item_list_rid))
                if is_ship_on_fleet_item_list(ship, &item_list_rid) =>
            {
                let cmod = CtxModifier::new_with_item(*rmod, ship_uid);
                remove_cmod(&mut reg_cmods.direct, ship_uid, &cmod, &mut reg_cmods.by_aspec);
            }
            AffecteeFilter::Loc(Location::ItemList(item_list_rid))
                if is_ship_on_fleet_item_list(ship, &item_list_rid)
                    && let Ok(loc_kind) = ship.get_kind().try_into() =>
            {
                let cmod = CtxModifier::new_with_fit_item(*rmod, fit_uid, ship_uid);
                let key = (fit_uid, loc_kind);
                remove_cmod(&mut reg_cmods.loc, key, &cmod, &mut reg_cmods.by_aspec);
            }
            AffecteeFilter::LocGrp(Location::ItemList(item_list_rid), item_grp_id)
                if is_ship_on_fleet_item_list(ship, &item_list_rid)
                    && let Ok(loc_kind) = ship.get_kind().try_into() =>
            {
                let cmod = CtxModifier::new_with_fit_item(*rmod, fit_uid, ship_uid);
                let key = (fit_uid, loc_kind, item_grp_id);
                remove_cmod(&mut reg_cmods.loc_grp, key, &cmod, &mut reg_cmods.by_aspec);
            }
            AffecteeFilter::LocSrq(Location::ItemList(item_list_rid), srq_type_aid)
                if is_ship_on_fleet_item_list(ship, &item_list_rid)
                    && let Ok(loc_kind) = ship.get_kind().try_into() =>
            {
                let cmod = CtxModifier::new_with_fit_item(*rmod, fit_uid, ship_uid);
                let key = (fit_uid, loc_kind, srq_type_aid);
                remove_cmod(&mut reg_cmods.loc_srq, key, &cmod, &mut reg_cmods.by_aspec);
            }
            _ => (),
        }
    }
}
