use crate::{
    ad::AItemListId,
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
    ud::{UFitKey, UFleet, UItem, UItemKey, UShip},
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
        let fit_key = match item.get_fit_key() {
            Some(fit_key) => fit_key,
            None => return false,
        };
        // Check validity and apply buffs to fit which carries it
        let valid = apply_fleet_mod_with_fit_key(reuse_cmods, ctx, &mut self.cmods, rmod, fit_key);
        if valid {
            // If buff is valid, apply to the rest of the fleet
            let fit = ctx.u_data.fits.get(fit_key);
            if let Some(fleet_key) = fit.fleet {
                let fleet = ctx.u_data.fleets.get(fleet_key);
                for fleet_fit_key in fleet.iter_fits() {
                    if fleet_fit_key == fit_key {
                        continue;
                    }
                    apply_fleet_mod_with_fit_key(reuse_cmods, ctx, &mut self.cmods, rmod, fleet_fit_key);
                }
            }
            self.rmods_fleet.add_entry(fit_key, rmod);
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
        let fit_key = match item.get_fit_key() {
            Some(fit_key) => fit_key,
            None => return,
        };
        // Modifiers received by this function are assumed to be valid, so just unapply all
        // modifications and remove modifier from helper container
        let fit = ctx.u_data.fits.get(fit_key);
        match fit.fleet {
            Some(fleet_key) => {
                let fleet = ctx.u_data.fleets.get(fleet_key);
                for fleet_fit_key in fleet.iter_fits() {
                    unapply_fleet_mod_with_fit_key(reuse_cmods, ctx, &mut self.cmods, rmod, fleet_fit_key);
                }
            }
            None => {
                unapply_fleet_mod_with_fit_key(reuse_cmods, ctx, &mut self.cmods, rmod, fit_key);
            }
        }
        self.rmods_fleet.remove_entry(fit_key, &rmod);
    }
    pub(in crate::svc::calc) fn reg_fleet_for_fit(
        &mut self,
        ctx: SvcCtx,
        fleet: &UFleet,
        fit_key: UFitKey,
    ) -> Vec<CtxModifier> {
        let mut cmods = Vec::new();
        // Outgoing fleet boosts
        for rmod in self.rmods_fleet.get(&fit_key) {
            for fleet_fit_key in fleet.iter_fits() {
                if fleet_fit_key != fit_key {
                    apply_fleet_mod_with_fit_key(&mut cmods, ctx, &mut self.cmods, *rmod, fleet_fit_key);
                }
            }
        }
        // Incoming fleet boosts
        for fleet_fit_key in fleet.iter_fits() {
            if fleet_fit_key == fit_key {
                continue;
            }
            for &rmod in self.rmods_fleet.get(&fleet_fit_key) {
                apply_fleet_mod_with_fit_key(&mut cmods, ctx, &mut self.cmods, rmod, fit_key);
            }
        }
        cmods
    }
    pub(in crate::svc::calc) fn unreg_fleet_for_fit(
        &mut self,
        ctx: SvcCtx,
        fleet: &UFleet,
        fit_key: UFitKey,
    ) -> Vec<CtxModifier> {
        let mut cmods = Vec::new();
        // Outgoing fleet boosts
        for rmod in self.rmods_fleet.get(&fit_key) {
            for fleet_fit_key in fleet.iter_fits() {
                if fleet_fit_key != fit_key {
                    unapply_fleet_mod_with_fit_key(&mut cmods, ctx, &mut self.cmods, *rmod, fleet_fit_key);
                }
            }
        }
        // Incoming fleet boosts
        for fleet_fit_key in fleet.iter_fits() {
            if fleet_fit_key == fit_key {
                continue;
            }
            for rmod in self.rmods_fleet.get(&fleet_fit_key) {
                unapply_fleet_mod_with_fit_key(&mut cmods, ctx, &mut self.cmods, *rmod, fit_key);
            }
        }
        cmods
    }
    pub(in crate::svc::calc::registers::standard) fn load_affectee_for_fleet(
        &mut self,
        ctx: SvcCtx,
        ship_key: UItemKey,
        ship: &UShip,
    ) {
        let fit_key = ship.get_fit_key();
        let fit = ctx.u_data.fits.get(fit_key);
        match fit.fleet {
            Some(fleet_key) => {
                let fleet = ctx.u_data.fleets.get(fleet_key);
                for fleet_fit_key in fleet.iter_fits() {
                    let fleet_rmods = self.rmods_fleet.get(&fleet_fit_key);
                    apply_fleet_mods_to_ship_fit(&mut self.cmods, fleet_rmods, ship_key, ship);
                }
            }
            None => {
                let fleet_rmods = self.rmods_fleet.get(&fit_key);
                apply_fleet_mods_to_ship_fit(&mut self.cmods, fleet_rmods, ship_key, ship);
            }
        }
    }
    pub(in crate::svc::calc::registers::standard) fn unload_affectee_for_fleet(
        &mut self,
        ctx: SvcCtx,
        ship_key: UItemKey,
        ship: &UShip,
    ) {
        let fit_key = ship.get_fit_key();
        let fit = ctx.u_data.fits.get(fit_key);
        match fit.fleet {
            Some(fleet_key) => {
                let fleet = ctx.u_data.fleets.get(fleet_key);
                for fleet_fit_key in fleet.iter_fits() {
                    let fleet_rmods = self.rmods_fleet.get(&fleet_fit_key);
                    unapply_fleet_mods_from_ship_fit(&mut self.cmods, fleet_rmods, ship_key, ship);
                }
            }
            None => {
                let fleet_rmods = self.rmods_fleet.get(&fit_key);
                unapply_fleet_mods_from_ship_fit(&mut self.cmods, fleet_rmods, ship_key, ship);
            }
        }
    }
}

fn is_fit_ship_on_fleet_item_list<'u>(
    ctx: SvcCtx<'u, '_>,
    fit_key: UFitKey,
    item_list_id: &AItemListId,
) -> Option<(UItemKey, &'u UShip)> {
    let fit = ctx.u_data.fits.get(fit_key);
    let ship_key = fit.ship?;
    let ship = ctx.u_data.items.get(ship_key).dc_ship().unwrap();
    match is_ship_on_fleet_item_list(ship, item_list_id) {
        true => Some((ship_key, ship)),
        false => None,
    }
}
fn is_ship_on_fleet_item_list(ship: &UShip, item_list_id: &AItemListId) -> bool {
    match ship.get_fleet_buff_item_lists() {
        Some(item_list_ids) => item_list_ids.contains(item_list_id),
        None => false,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Regular apply-unapply functions
////////////////////////////////////////////////////////////////////////////////////////////////////
fn apply_fleet_mod_with_fit_key(
    reuse_cmods: &mut Vec<CtxModifier>,
    ctx: SvcCtx,
    reg_cmods: &mut StandardRegisterCtxMods,
    rmod: RawModifier,
    fit_key: UFitKey,
) -> bool {
    match rmod.affectee_filter {
        AffecteeFilter::Direct(Location::ItemList(item_list_id)) => {
            if let Some((ship_key, _)) = is_fit_ship_on_fleet_item_list(ctx, fit_key, &item_list_id) {
                let cmod = CtxModifier::new_with_projectee_item(rmod, ship_key);
                add_cmod(&mut reg_cmods.direct, ship_key, cmod, &mut reg_cmods.by_aspec);
                reuse_cmods.push(cmod);
            }
            true
        }
        AffecteeFilter::Loc(Location::ItemList(item_list_id)) => {
            if let Some((_, ship)) = is_fit_ship_on_fleet_item_list(ctx, fit_key, &item_list_id)
                && let Ok(loc_kind) = ship.get_kind().try_into()
            {
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, loc_kind);
                add_cmod(&mut reg_cmods.loc, key, cmod, &mut reg_cmods.by_aspec);
                reuse_cmods.push(cmod);
            }
            true
        }
        AffecteeFilter::LocGrp(Location::ItemList(item_list_id), item_grp_id) => {
            if let Some((_, ship)) = is_fit_ship_on_fleet_item_list(ctx, fit_key, &item_list_id)
                && let Ok(loc_kind) = ship.get_kind().try_into()
            {
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, loc_kind, item_grp_id);
                add_cmod(&mut reg_cmods.loc_grp, key, cmod, &mut reg_cmods.by_aspec);
                reuse_cmods.push(cmod);
            }
            true
        }
        AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_type_id) => {
            if let Some((_, ship)) = is_fit_ship_on_fleet_item_list(ctx, fit_key, &item_list_id)
                && let Ok(loc_kind) = ship.get_kind().try_into()
            {
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, loc_kind, srq_type_id);
                add_cmod(&mut reg_cmods.loc_srq, key, cmod, &mut reg_cmods.by_aspec);
                reuse_cmods.push(cmod);
            }
            true
        }
        _ => false,
    }
}
fn unapply_fleet_mod_with_fit_key(
    reuse_cmods: &mut Vec<CtxModifier>,
    ctx: SvcCtx,
    reg_cmods: &mut StandardRegisterCtxMods,
    rmod: RawModifier,
    fit_key: UFitKey,
) {
    // We don't check location here, since logic on layers above ensures we receive only
    // modifiers which passed checks when they were added, and location check is part of those
    match rmod.affectee_filter {
        AffecteeFilter::Direct(Location::ItemList(item_list_id))
            if let Some((ship_key, _)) = is_fit_ship_on_fleet_item_list(ctx, fit_key, &item_list_id) =>
        {
            let cmod = CtxModifier::new_with_projectee_item(rmod, ship_key);
            remove_cmod(&mut reg_cmods.direct, ship_key, &cmod, &mut reg_cmods.by_aspec);
            reuse_cmods.push(cmod);
        }
        AffecteeFilter::Loc(Location::ItemList(item_list_id))
            if let Some((_, ship)) = is_fit_ship_on_fleet_item_list(ctx, fit_key, &item_list_id)
                && let Ok(loc_kind) = ship.get_kind().try_into() =>
        {
            let cmod = CtxModifier::new_with_fit(rmod, fit_key);
            let key = (fit_key, loc_kind);
            remove_cmod(&mut reg_cmods.loc, key, &cmod, &mut reg_cmods.by_aspec);
            reuse_cmods.push(cmod);
        }
        AffecteeFilter::LocGrp(Location::ItemList(item_list_id), item_grp_id)
            if let Some((_, ship)) = is_fit_ship_on_fleet_item_list(ctx, fit_key, &item_list_id)
                && let Ok(loc_kind) = ship.get_kind().try_into() =>
        {
            let cmod = CtxModifier::new_with_fit(rmod, fit_key);
            let key = (fit_key, loc_kind, item_grp_id);
            remove_cmod(&mut reg_cmods.loc_grp, key, &cmod, &mut reg_cmods.by_aspec);
            reuse_cmods.push(cmod);
        }
        AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_type_id)
            if let Some((_, ship)) = is_fit_ship_on_fleet_item_list(ctx, fit_key, &item_list_id)
                && let Ok(loc_kind) = ship.get_kind().try_into() =>
        {
            let cmod = CtxModifier::new_with_fit(rmod, fit_key);
            let key = (fit_key, loc_kind, srq_type_id);
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
    ship_key: UItemKey,
    ship: &UShip,
) {
    let fit_key = ship.get_fit_key();
    for rmod in fleet_rmods {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(Location::ItemList(item_list_id))
                if is_ship_on_fleet_item_list(ship, &item_list_id) =>
            {
                let cmod = CtxModifier::new_with_projectee_item(*rmod, ship_key);
                add_cmod(&mut reg_cmods.direct, ship_key, cmod, &mut reg_cmods.by_aspec);
            }
            AffecteeFilter::Loc(Location::ItemList(item_list_id))
                if is_ship_on_fleet_item_list(ship, &item_list_id)
                    && let Ok(loc_kind) = ship.get_kind().try_into() =>
            {
                let cmod = CtxModifier::new_with_fit(*rmod, fit_key);
                let key = (fit_key, loc_kind);
                add_cmod(&mut reg_cmods.loc, key, cmod, &mut reg_cmods.by_aspec);
            }
            AffecteeFilter::LocGrp(Location::ItemList(item_list_id), item_grp_id)
                if is_ship_on_fleet_item_list(ship, &item_list_id)
                    && let Ok(loc_kind) = ship.get_kind().try_into() =>
            {
                let cmod = CtxModifier::new_with_fit(*rmod, fit_key);
                let key = (fit_key, loc_kind, item_grp_id);
                add_cmod(&mut reg_cmods.loc_grp, key, cmod, &mut reg_cmods.by_aspec);
            }
            AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_type_id)
                if is_ship_on_fleet_item_list(ship, &item_list_id)
                    && let Ok(loc_kind) = ship.get_kind().try_into() =>
            {
                let cmod = CtxModifier::new_with_fit(*rmod, fit_key);
                let key = (fit_key, loc_kind, srq_type_id);
                add_cmod(&mut reg_cmods.loc_srq, key, cmod, &mut reg_cmods.by_aspec);
            }
            _ => (),
        }
    }
}
fn unapply_fleet_mods_from_ship_fit<'a>(
    reg_cmods: &mut StandardRegisterCtxMods,
    fleet_rmods: impl Iterator<Item = &'a RawModifier>,
    ship_key: UItemKey,
    ship: &UShip,
) {
    let fit_key = ship.get_fit_key();
    for rmod in fleet_rmods {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(Location::ItemList(item_list_id))
                if is_ship_on_fleet_item_list(ship, &item_list_id) =>
            {
                let cmod = CtxModifier::new_with_projectee_item(*rmod, ship_key);
                remove_cmod(&mut reg_cmods.direct, ship_key, &cmod, &mut reg_cmods.by_aspec);
            }
            AffecteeFilter::Loc(Location::ItemList(item_list_id))
                if is_ship_on_fleet_item_list(ship, &item_list_id)
                    && let Ok(loc_kind) = ship.get_kind().try_into() =>
            {
                let cmod = CtxModifier::new_with_fit(*rmod, fit_key);
                let key = (fit_key, loc_kind);
                remove_cmod(&mut reg_cmods.loc, key, &cmod, &mut reg_cmods.by_aspec);
            }
            AffecteeFilter::LocGrp(Location::ItemList(item_list_id), item_grp_id)
                if is_ship_on_fleet_item_list(ship, &item_list_id)
                    && let Ok(loc_kind) = ship.get_kind().try_into() =>
            {
                let cmod = CtxModifier::new_with_fit(*rmod, fit_key);
                let key = (fit_key, loc_kind, item_grp_id);
                remove_cmod(&mut reg_cmods.loc_grp, key, &cmod, &mut reg_cmods.by_aspec);
            }
            AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_type_id)
                if is_ship_on_fleet_item_list(ship, &item_list_id)
                    && let Ok(loc_kind) = ship.get_kind().try_into() =>
            {
                let cmod = CtxModifier::new_with_fit(*rmod, fit_key);
                let key = (fit_key, loc_kind, srq_type_id);
                remove_cmod(&mut reg_cmods.loc_srq, key, &cmod, &mut reg_cmods.by_aspec);
            }
            _ => (),
        }
    }
}
