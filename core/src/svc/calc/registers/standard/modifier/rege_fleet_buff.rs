use crate::{
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, CtxModifier, Location, RawModifier,
            registers::standard::{
                StandardRegister,
                modifier::func::{add_cmod, is_fit_ship_on_item_list, remove_cmod},
            },
        },
    },
    ud::{UFitKey, UFleet, UItem},
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
        let mut valid = false;
        let affector_fit = ctx.u_data.fits.get(fit_key);
        match affector_fit.fleet {
            Some(fleet_key) => {
                let fleet = ctx.u_data.fleets.get(fleet_key);
                for fleet_fit_key in fleet.iter_fits() {
                    if self.apply_fleet_mod(reuse_cmods, ctx, rmod, fleet_fit_key) {
                        valid = true;
                    }
                }
            }
            None => {
                if self.apply_fleet_mod(reuse_cmods, ctx, rmod, fit_key) {
                    valid = true;
                }
            }
        }
        if valid {
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
        let affector_fit = ctx.u_data.fits.get(fit_key);
        match affector_fit.fleet {
            Some(fleet_key) => {
                let fleet = ctx.u_data.fleets.get(fleet_key);
                for fleet_fit_key in fleet.iter_fits() {
                    self.unapply_fleet_mod(reuse_cmods, ctx, rmod, fleet_fit_key);
                }
            }
            None => {
                self.unapply_fleet_mod(reuse_cmods, ctx, rmod, fit_key);
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
        let mut rmods = Vec::new();
        let mut cmods = Vec::new();
        // Outgoing fleet boosts
        rmods.extend(self.rmods_fleet.get(&fit_key).copied());
        for rmod in rmods.iter() {
            for fleet_fit_key in fleet.iter_fits() {
                if fleet_fit_key != fit_key {
                    self.apply_fleet_mod(&mut cmods, ctx, *rmod, fleet_fit_key);
                }
            }
        }
        // Incoming fleet boosts
        for fleet_fit_key in fleet.iter_fits() {
            if fleet_fit_key == fit_key {
                continue;
            }
            rmods.clear();
            rmods.extend(self.rmods_fleet.get(&fleet_fit_key).copied());
            for rmod in rmods.iter() {
                self.apply_fleet_mod(&mut cmods, ctx, *rmod, fit_key);
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
        let mut rmods = Vec::new();
        let mut cmods = Vec::new();
        // Outgoing fleet boosts
        rmods.extend(self.rmods_fleet.get(&fit_key).copied());
        for rmod in rmods.iter() {
            for fleet_fit_key in fleet.iter_fits() {
                if fleet_fit_key != fit_key {
                    self.unapply_fleet_mod(&mut cmods, ctx, *rmod, fleet_fit_key)
                }
            }
        }
        // Incoming fleet boosts
        for fleet_fit_key in fleet.iter_fits() {
            if fleet_fit_key == fit_key {
                continue;
            }
            rmods.clear();
            rmods.extend(self.rmods_fleet.get(&fleet_fit_key).copied());
            for rmod in rmods.iter() {
                self.unapply_fleet_mod(&mut cmods, ctx, *rmod, fit_key)
            }
        }
        cmods
    }
    // Private methods
    fn apply_fleet_mod(
        &mut self,
        reuse_cmods: &mut Vec<CtxModifier>,
        ctx: SvcCtx,
        rmod: RawModifier,
        fit_key: UFitKey,
    ) -> bool {
        // Assume all fleet buffs affect ships. This is controlled by the lib, so whenever
        // item-specific buffs are added to EVE, this implementation has to be changed
        match rmod.affectee_filter {
            AffecteeFilter::Direct(Location::ItemList(item_list_id)) => {
                if let Some((_, ship)) = is_fit_ship_on_item_list(ctx, fit_key, &item_list_id)
                    && let Ok(loc_kind) = ship.get_kind().try_into()
                {
                    let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                    let key = (fit_key, loc_kind);
                    add_cmod(&mut self.cmods.root, key, cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            AffecteeFilter::Loc(Location::ItemList(item_list_id)) => {
                if let Some((_, ship)) = is_fit_ship_on_item_list(ctx, fit_key, &item_list_id)
                    && let Ok(loc_kind) = ship.get_kind().try_into()
                {
                    let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                    let key = (fit_key, loc_kind);
                    add_cmod(&mut self.cmods.loc, key, cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            AffecteeFilter::LocGrp(Location::ItemList(item_list_id), item_grp_id) => {
                if let Some((_, ship)) = is_fit_ship_on_item_list(ctx, fit_key, &item_list_id)
                    && let Ok(loc_kind) = ship.get_kind().try_into()
                {
                    let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                    let key = (fit_key, loc_kind, item_grp_id);
                    add_cmod(&mut self.cmods.loc_grp, key, cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_type_id) => {
                if let Some((_, ship)) = is_fit_ship_on_item_list(ctx, fit_key, &item_list_id)
                    && let Ok(loc_kind) = ship.get_kind().try_into()
                {
                    let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                    let key = (fit_key, loc_kind, srq_type_id);
                    add_cmod(&mut self.cmods.loc_srq, key, cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            _ => false,
        }
    }
    fn unapply_fleet_mod(
        &mut self,
        reuse_cmods: &mut Vec<CtxModifier>,
        ctx: SvcCtx,
        rmod: RawModifier,
        fit_key: UFitKey,
    ) {
        // We don't check location here, since logic on layers above ensures we receive only
        // modifiers which passed checks when they were added, and location check is part of those
        match rmod.affectee_filter {
            AffecteeFilter::Direct(Location::ItemList(item_list_id)) => {
                if let Some((_, ship)) = is_fit_ship_on_item_list(ctx, fit_key, &item_list_id)
                    && let Ok(loc_kind) = ship.get_kind().try_into()
                {
                    let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                    let key = (fit_key, loc_kind);
                    remove_cmod(&mut self.cmods.root, key, &cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
            }
            AffecteeFilter::Loc(Location::ItemList(item_list_id)) => {
                if let Some((_, ship)) = is_fit_ship_on_item_list(ctx, fit_key, &item_list_id)
                    && let Ok(loc_kind) = ship.get_kind().try_into()
                {
                    let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                    let key = (fit_key, loc_kind);
                    remove_cmod(&mut self.cmods.loc, key, &cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
            }
            AffecteeFilter::LocGrp(Location::ItemList(item_list_id), item_grp_id) => {
                if let Some((_, ship)) = is_fit_ship_on_item_list(ctx, fit_key, &item_list_id)
                    && let Ok(loc_kind) = ship.get_kind().try_into()
                {
                    let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                    let key = (fit_key, loc_kind, item_grp_id);
                    remove_cmod(&mut self.cmods.loc_grp, key, &cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
            }
            AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_type_id) => {
                if let Some((_, ship)) = is_fit_ship_on_item_list(ctx, fit_key, &item_list_id)
                    && let Ok(loc_kind) = ship.get_kind().try_into()
                {
                    let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                    let key = (fit_key, loc_kind, srq_type_id);
                    remove_cmod(&mut self.cmods.loc_srq, key, &cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
            }
            _ => (),
        }
    }
}
