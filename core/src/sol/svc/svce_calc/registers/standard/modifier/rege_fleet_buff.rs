use crate::{
    defs::SolFitId,
    sol::{
        fleet::SolFleet,
        item::SolItem,
        svc::svce_calc::{
            registers::SolStandardRegister, SolAffecteeFilter, SolCtxModifier, SolDomain, SolLocationKind,
            SolRawModifier,
        },
        SolView,
    },
};

use super::{reg_cmod, unreg_cmod};

impl SolStandardRegister {
    pub(in crate::sol::svc::svce_calc) fn reg_fleet_buff_mod(
        &mut self,
        ctx_modifiers: &mut Vec<SolCtxModifier>,
        sol_view: &SolView,
        item: &SolItem,
        raw_modifier: SolRawModifier,
    ) {
        ctx_modifiers.clear();
        let fit_id = match item.get_fit_id() {
            Some(fit_id) => fit_id,
            None => return,
        };
        let affector_fit = sol_view.fits.get_fit(&fit_id).unwrap();
        match affector_fit.fleet {
            Some(fleet_id) => {
                let fleet = sol_view.fleets.get_fleet(&fleet_id).unwrap();
                for fleet_fit_id in fleet.iter_fits() {
                    if let Some(ctx_modifier) = self.apply_fleet_mod(raw_modifier, *fleet_fit_id) {
                        ctx_modifiers.push(ctx_modifier);
                    }
                }
            }
            None => {
                if let Some(ctx_modifier) = self.apply_fleet_mod(raw_modifier, fit_id) {
                    ctx_modifiers.push(ctx_modifier);
                }
            }
        }
        // Here, we can rely on presence of ctx modifiers, because there is always a fit we will go
        // through when adding them; if raw modifier was valid, there will always be ctx one
        if !ctx_modifiers.is_empty() {
            self.rmods_fleet.add_entry(fit_id, raw_modifier);
            self.rmods_nonproj
                .add_entry((raw_modifier.affector_item_id, raw_modifier.effect_id), raw_modifier);
            for ctx_modifier in ctx_modifiers.iter() {
                reg_cmod(&mut self.cmods_by_attr_spec, *ctx_modifier);
            }
        }
    }
    pub(in crate::sol::svc::svce_calc) fn unreg_fleet_buff_mod(
        &mut self,
        ctx_modifiers: &mut Vec<SolCtxModifier>,
        sol_view: &SolView,
        item: &SolItem,
        raw_modifier: SolRawModifier,
    ) {
        ctx_modifiers.clear();
        let fit_id = match item.get_fit_id() {
            Some(fit_id) => fit_id,
            None => return,
        };
        let affector_fit = sol_view.fits.get_fit(&fit_id).unwrap();
        match affector_fit.fleet {
            Some(fleet_id) => {
                let fleet = sol_view.fleets.get_fleet(&fleet_id).unwrap();
                for fleet_fit_id in fleet.iter_fits() {
                    if let Some(ctx_modifier) = self.unapply_fleet_mod(raw_modifier, *fleet_fit_id) {
                        ctx_modifiers.push(ctx_modifier);
                    }
                }
            }
            None => {
                if let Some(ctx_modifier) = self.unapply_fleet_mod(raw_modifier, fit_id) {
                    ctx_modifiers.push(ctx_modifier);
                }
            }
        }
        self.rmods_fleet.remove_entry(&fit_id, &raw_modifier);
        for ctx_modifier in ctx_modifiers.iter() {
            unreg_cmod(&mut self.cmods_by_attr_spec, ctx_modifier);
        }
    }
    pub(in crate::sol::svc::svce_calc) fn reg_fleet_for_fit(
        &mut self,
        fleet: &SolFleet,
        fit_id: &SolFitId,
    ) -> Vec<SolCtxModifier> {
        let mut rmods = Vec::new();
        let mut cmods = Vec::new();
        // Outgoing fleet boosts
        rmods.extend(self.rmods_fleet.get(fit_id).map(|v| *v));
        for raw_modifier in rmods.iter() {
            for fleet_fit_id in fleet.iter_fits() {
                if fleet_fit_id == fit_id {
                    continue;
                }
                if let Some(ctx_modifier) = self.apply_fleet_mod(*raw_modifier, *fleet_fit_id) {
                    cmods.push(ctx_modifier);
                }
            }
        }
        // Incoming fleet boosts
        for fleet_fit_id in fleet.iter_fits() {
            if fleet_fit_id == fit_id {
                continue;
            }
            rmods.clear();
            rmods.extend(self.rmods_fleet.get(fleet_fit_id).map(|v| *v));
            for raw_modifier in rmods.iter() {
                if let Some(ctx_modifier) = self.apply_fleet_mod(*raw_modifier, *fit_id) {
                    cmods.push(ctx_modifier);
                }
            }
        }
        for ctx_modifier in cmods.iter() {
            reg_cmod(&mut self.cmods_by_attr_spec, *ctx_modifier);
        }
        cmods
    }
    pub(in crate::sol::svc::svce_calc) fn unreg_fleet_for_fit(
        &mut self,
        fleet: &SolFleet,
        fit_id: &SolFitId,
    ) -> Vec<SolCtxModifier> {
        let mut rmods = Vec::new();
        let mut cmods = Vec::new();
        // Outgoing fleet boosts
        rmods.extend(self.rmods_fleet.get(fit_id).map(|v| *v));
        for raw_modifier in rmods.iter() {
            for fleet_fit_id in fleet.iter_fits() {
                if fleet_fit_id == fit_id {
                    continue;
                }
                if let Some(ctx_modifier) = self.unapply_fleet_mod(*raw_modifier, *fleet_fit_id) {
                    cmods.push(ctx_modifier);
                }
            }
        }
        // Incoming fleet boosts
        for fleet_fit_id in fleet.iter_fits() {
            if fleet_fit_id == fit_id {
                continue;
            }
            rmods.clear();
            rmods.extend(self.rmods_fleet.get(fleet_fit_id).map(|v| *v));
            for raw_modifier in rmods.iter() {
                if let Some(ctx_modifier) = self.unapply_fleet_mod(*raw_modifier, *fit_id) {
                    cmods.push(ctx_modifier);
                }
            }
        }
        for ctx_modifier in cmods.iter() {
            unreg_cmod(&mut self.cmods_by_attr_spec, ctx_modifier);
        }
        cmods
    }
    // Private methods
    fn apply_fleet_mod(&mut self, raw_modifier: SolRawModifier, fit_id: SolFitId) -> Option<SolCtxModifier> {
        match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolDomain::Ship => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                    self.cmods_root.add_entry((fit_id, SolLocationKind::Ship), ctx_modifier);
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolDomain::Ship => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                    self.cmods_loc.add_entry((fit_id, SolLocationKind::Ship), ctx_modifier);
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolDomain::Ship => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                    self.cmods_loc_grp
                        .add_entry((fit_id, SolLocationKind::Ship, grp_id), ctx_modifier);
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolDomain::Ship => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                    self.cmods_loc_srq
                        .add_entry((fit_id, SolLocationKind::Ship, srq_id), ctx_modifier);
                    Some(ctx_modifier)
                }
                _ => None,
            },
            _ => None,
        }
    }
    fn unapply_fleet_mod(&mut self, raw_modifier: SolRawModifier, fit_id: SolFitId) -> Option<SolCtxModifier> {
        // We don't check domain here, since logic on layers above ensures we receive only modifiers
        // which passed checks when they were added, and domain check is part of those
        match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(_) => {
                let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                self.cmods_root
                    .remove_entry(&(fit_id, SolLocationKind::Ship), &ctx_modifier);
                Some(ctx_modifier)
            }
            SolAffecteeFilter::Loc(_) => {
                let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                self.cmods_loc
                    .remove_entry(&(fit_id, SolLocationKind::Ship), &ctx_modifier);
                Some(ctx_modifier)
            }
            SolAffecteeFilter::LocGrp(_, grp_id) => {
                let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                self.cmods_loc_grp
                    .remove_entry(&(fit_id, SolLocationKind::Ship, grp_id), &ctx_modifier);
                Some(ctx_modifier)
            }
            SolAffecteeFilter::LocSrq(_, srq_id) => {
                let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                self.cmods_loc_srq
                    .remove_entry(&(fit_id, SolLocationKind::Ship, srq_id), &ctx_modifier);
                Some(ctx_modifier)
            }
            _ => None,
        }
    }
}
