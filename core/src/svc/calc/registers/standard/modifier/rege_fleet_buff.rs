use crate::{
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, CtxModifier, Location, LocationKind, RawModifier,
            registers::standard::{
                StandardRegister,
                modifier::func::{add_cmod, remove_cmod},
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
        let affector_fit = ctx.u_data.fits.get(fit_key);
        match affector_fit.fleet {
            Some(fleet_key) => {
                let fleet = ctx.u_data.fleets.get(fleet_key);
                for fleet_fit_key in fleet.iter_fits() {
                    if let Some(cmod) = self.apply_fleet_mod(rmod, fleet_fit_key) {
                        reuse_cmods.push(cmod);
                    }
                }
            }
            None => {
                if let Some(cmod) = self.apply_fleet_mod(rmod, fit_key) {
                    reuse_cmods.push(cmod);
                }
            }
        }
        // Here, we can rely on presence of ctx modifiers, because there is always a fit we will go
        // through when adding them; if raw modifier is valid, there will always be a ctx one
        if !reuse_cmods.is_empty() {
            self.rmods_fleet.add_entry(fit_key, rmod);
            self.rmods_all.add_entry(rmod.affector_espec, rmod);
        }
        !reuse_cmods.is_empty()
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
                    if let Some(cmod) = self.unapply_fleet_mod(rmod, fleet_fit_key) {
                        reuse_cmods.push(cmod);
                    }
                }
            }
            None => {
                if let Some(cmod) = self.unapply_fleet_mod(rmod, fit_key) {
                    reuse_cmods.push(cmod);
                }
            }
        }
        self.rmods_fleet.remove_entry(fit_key, &rmod);
    }
    pub(in crate::svc::calc) fn reg_fleet_for_fit(&mut self, fleet: &UFleet, fit_key: UFitKey) -> Vec<CtxModifier> {
        let mut rmods = Vec::new();
        let mut cmods = Vec::new();
        // Outgoing fleet boosts
        rmods.extend(self.rmods_fleet.get(&fit_key).copied());
        for rmod in rmods.iter() {
            for fleet_fit_key in fleet.iter_fits() {
                if fleet_fit_key != fit_key
                    && let Some(cmod) = self.apply_fleet_mod(*rmod, fleet_fit_key)
                {
                    cmods.push(cmod);
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
                if let Some(cmod) = self.apply_fleet_mod(*rmod, fit_key) {
                    cmods.push(cmod);
                }
            }
        }
        cmods
    }
    pub(in crate::svc::calc) fn unreg_fleet_for_fit(&mut self, fleet: &UFleet, fit_key: UFitKey) -> Vec<CtxModifier> {
        let mut rmods = Vec::new();
        let mut cmods = Vec::new();
        // Outgoing fleet boosts
        rmods.extend(self.rmods_fleet.get(&fit_key).copied());
        for rmod in rmods.iter() {
            for fleet_fit_key in fleet.iter_fits() {
                if fleet_fit_key != fit_key
                    && let Some(cmod) = self.unapply_fleet_mod(*rmod, fleet_fit_key)
                {
                    cmods.push(cmod);
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
                if let Some(cmod) = self.unapply_fleet_mod(*rmod, fit_key) {
                    cmods.push(cmod);
                }
            }
        }
        cmods
    }
    // Private methods
    fn apply_fleet_mod(&mut self, rmod: RawModifier, fit_key: UFitKey) -> Option<CtxModifier> {
        // Assume all fleet buffs affect ships. This is controlled by the lib, so whenever
        // item-specific buffs are added to EVE, this implementation has to be changed
        match rmod.affectee_filter {
            AffecteeFilter::Direct(Location::ItemList(_)) => {
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, LocationKind::Ship);
                add_cmod(&mut self.cmods.root, key, cmod, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::Loc(Location::ItemList(_)) => {
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, LocationKind::Ship);
                add_cmod(&mut self.cmods.loc, key, cmod, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::LocGrp(Location::ItemList(_), item_grp_id) => {
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, LocationKind::Ship, item_grp_id);
                add_cmod(&mut self.cmods.loc_grp, key, cmod, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::LocSrq(Location::ItemList(_), srq_type_id) => {
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, LocationKind::Ship, srq_type_id);
                add_cmod(&mut self.cmods.loc_srq, key, cmod, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            _ => None,
        }
    }
    fn unapply_fleet_mod(&mut self, rmod: RawModifier, fit_key: UFitKey) -> Option<CtxModifier> {
        // We don't check location here, since logic on layers above ensures we receive only
        // modifiers which passed checks when they were added, and location check is part of those
        match rmod.affectee_filter {
            AffecteeFilter::Direct(_) => {
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, LocationKind::Ship);
                remove_cmod(&mut self.cmods.root, key, &cmod, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::Loc(_) => {
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, LocationKind::Ship);
                remove_cmod(&mut self.cmods.loc, key, &cmod, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::LocGrp(_, item_grp_id) => {
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, LocationKind::Ship, item_grp_id);
                remove_cmod(&mut self.cmods.loc_grp, key, &cmod, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::LocSrq(_, srq_type_id) => {
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, LocationKind::Ship, srq_type_id);
                remove_cmod(&mut self.cmods.loc_srq, key, &cmod, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            _ => None,
        }
    }
}
