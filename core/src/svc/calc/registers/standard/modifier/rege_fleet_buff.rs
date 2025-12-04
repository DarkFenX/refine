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
        let valid = apply_fleet_mod(reuse_cmods, ctx, &mut self.cmods, rmod, fit_key);
        if valid {
            // If buff is valid, apply to the rest of the fleet
            if let Some(fleet_key) = ctx.u_data.fits.get(fit_key).fleet {
                for fleet_fit_key in ctx.u_data.fleets.get(fleet_key).iter_fits() {
                    if fleet_fit_key == fit_key {
                        continue;
                    }
                    apply_fleet_mod(reuse_cmods, ctx, &mut self.cmods, rmod, fleet_fit_key);
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
        match ctx.u_data.fits.get(fit_key).fleet {
            Some(fleet_key) => {
                for fleet_fit_key in ctx.u_data.fleets.get(fleet_key).iter_fits() {
                    unapply_fleet_mod(reuse_cmods, ctx, &mut self.cmods, rmod, fleet_fit_key);
                }
            }
            None => {
                unapply_fleet_mod(reuse_cmods, ctx, &mut self.cmods, rmod, fit_key);
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
                    apply_fleet_mod(&mut cmods, ctx, &mut self.cmods, *rmod, fleet_fit_key);
                }
            }
        }
        // Incoming fleet boosts
        for fleet_fit_key in fleet.iter_fits() {
            if fleet_fit_key == fit_key {
                continue;
            }
            for &rmod in self.rmods_fleet.get(&fleet_fit_key) {
                apply_fleet_mod(&mut cmods, ctx, &mut self.cmods, rmod, fit_key);
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
                    unapply_fleet_mod(&mut cmods, ctx, &mut self.cmods, *rmod, fleet_fit_key);
                }
            }
        }
        // Incoming fleet boosts
        for fleet_fit_key in fleet.iter_fits() {
            if fleet_fit_key == fit_key {
                continue;
            }
            for rmod in self.rmods_fleet.get(&fleet_fit_key) {
                unapply_fleet_mod(&mut cmods, ctx, &mut self.cmods, *rmod, fit_key);
            }
        }
        cmods
    }
}

fn apply_fleet_mod(
    reuse_cmods: &mut Vec<CtxModifier>,
    ctx: SvcCtx,
    reg_cmods: &mut StandardRegisterCtxMods,
    rmod: RawModifier,
    fit_key: UFitKey,
) -> bool {
    // Assume all fleet buffs affect ships. This is controlled by the lib, so whenever
    // item-specific buffs are added to EVE, this implementation has to be changed
    match rmod.affectee_filter {
        AffecteeFilter::Direct(Location::ItemList(item_list_id)) => {
            if let Some((_, ship)) = is_fit_ship_on_fleet_item_list(ctx, fit_key, &item_list_id)
                && let Ok(loc_kind) = ship.get_kind().try_into()
            {
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, loc_kind);
                add_cmod(&mut reg_cmods.root, key, cmod, &mut reg_cmods.by_aspec);
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
fn unapply_fleet_mod(
    reuse_cmods: &mut Vec<CtxModifier>,
    ctx: SvcCtx,
    reg_cmods: &mut StandardRegisterCtxMods,
    rmod: RawModifier,
    fit_key: UFitKey,
) {
    // We don't check location here, since logic on layers above ensures we receive only
    // modifiers which passed checks when they were added, and location check is part of those
    match rmod.affectee_filter {
        AffecteeFilter::Direct(Location::ItemList(item_list_id)) => {
            if let Some((_, ship)) = is_fit_ship_on_fleet_item_list(ctx, fit_key, &item_list_id)
                && let Ok(loc_kind) = ship.get_kind().try_into()
            {
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, loc_kind);
                remove_cmod(&mut reg_cmods.root, key, &cmod, &mut reg_cmods.by_aspec);
                reuse_cmods.push(cmod);
            }
        }
        AffecteeFilter::Loc(Location::ItemList(item_list_id)) => {
            if let Some((_, ship)) = is_fit_ship_on_fleet_item_list(ctx, fit_key, &item_list_id)
                && let Ok(loc_kind) = ship.get_kind().try_into()
            {
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, loc_kind);
                remove_cmod(&mut reg_cmods.loc, key, &cmod, &mut reg_cmods.by_aspec);
                reuse_cmods.push(cmod);
            }
        }
        AffecteeFilter::LocGrp(Location::ItemList(item_list_id), item_grp_id) => {
            if let Some((_, ship)) = is_fit_ship_on_fleet_item_list(ctx, fit_key, &item_list_id)
                && let Ok(loc_kind) = ship.get_kind().try_into()
            {
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, loc_kind, item_grp_id);
                remove_cmod(&mut reg_cmods.loc_grp, key, &cmod, &mut reg_cmods.by_aspec);
                reuse_cmods.push(cmod);
            }
        }
        AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_type_id) => {
            if let Some((_, ship)) = is_fit_ship_on_fleet_item_list(ctx, fit_key, &item_list_id)
                && let Ok(loc_kind) = ship.get_kind().try_into()
            {
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, loc_kind, srq_type_id);
                remove_cmod(&mut reg_cmods.loc_srq, key, &cmod, &mut reg_cmods.by_aspec);
                reuse_cmods.push(cmod);
            }
        }
        _ => (),
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
    match ship.get_fleet_buff_item_lists()?.contains(item_list_id) {
        true => Some((ship_key, ship)),
        false => None,
    }
}
