use crate::{
    defs::SolFitId,
    sol::{
        svc::calc::{
            registers::SolStandardRegister, SolAffecteeFilter, SolCtxModifier, SolDomain, SolLocationKind,
            SolRawModifier,
        },
        uad::{fleet::SolFleet, item::SolItem, SolUad},
    },
};

use super::{add_ctx_modifier, remove_ctx_modifier};

impl SolStandardRegister {
    pub(in crate::sol::svc::calc) fn reg_fleet_buff_mod(
        &mut self,
        ctx_modifiers: &mut Vec<SolCtxModifier>,
        uad: &SolUad,
        item: &SolItem,
        raw_modifier: SolRawModifier,
    ) -> bool {
        ctx_modifiers.clear();
        let fit_id = match item.get_fit_id() {
            Some(fit_id) => fit_id,
            None => return false,
        };
        let affector_fit = uad.fits.get_fit(&fit_id).unwrap();
        match affector_fit.fleet {
            Some(fleet_id) => {
                let fleet = uad.fleets.get_fleet(&fleet_id).unwrap();
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
        // through when adding them; if raw modifier is valid, there will always be a ctx one
        if !ctx_modifiers.is_empty() {
            self.rmods_fleet.add_entry(fit_id, raw_modifier);
            self.rmods_nonproj
                .add_entry((raw_modifier.affector_item_id, raw_modifier.effect_id), raw_modifier);
        }
        !ctx_modifiers.is_empty()
    }
    pub(in crate::sol::svc::calc) fn unreg_fleet_buff_mod(
        &mut self,
        ctx_modifiers: &mut Vec<SolCtxModifier>,
        uad: &SolUad,
        item: &SolItem,
        raw_modifier: SolRawModifier,
    ) {
        ctx_modifiers.clear();
        let fit_id = match item.get_fit_id() {
            Some(fit_id) => fit_id,
            None => return,
        };
        let affector_fit = uad.fits.get_fit(&fit_id).unwrap();
        match affector_fit.fleet {
            Some(fleet_id) => {
                let fleet = uad.fleets.get_fleet(&fleet_id).unwrap();
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
    }
    pub(in crate::sol::svc::calc) fn reg_fleet_for_fit(
        &mut self,
        fleet: &SolFleet,
        fit_id: &SolFitId,
    ) -> Vec<SolCtxModifier> {
        let mut raw_modifiers = Vec::new();
        let mut ctx_modifiers = Vec::new();
        // Outgoing fleet boosts
        raw_modifiers.extend(self.rmods_fleet.get(fit_id).map(|v| *v));
        for raw_modifier in raw_modifiers.iter() {
            for fleet_fit_id in fleet.iter_fits() {
                if fleet_fit_id == fit_id {
                    continue;
                }
                if let Some(ctx_modifier) = self.apply_fleet_mod(*raw_modifier, *fleet_fit_id) {
                    ctx_modifiers.push(ctx_modifier);
                }
            }
        }
        // Incoming fleet boosts
        for fleet_fit_id in fleet.iter_fits() {
            if fleet_fit_id == fit_id {
                continue;
            }
            raw_modifiers.clear();
            raw_modifiers.extend(self.rmods_fleet.get(fleet_fit_id).map(|v| *v));
            for raw_modifier in raw_modifiers.iter() {
                if let Some(ctx_modifier) = self.apply_fleet_mod(*raw_modifier, *fit_id) {
                    ctx_modifiers.push(ctx_modifier);
                }
            }
        }
        ctx_modifiers
    }
    pub(in crate::sol::svc::calc) fn unreg_fleet_for_fit(
        &mut self,
        fleet: &SolFleet,
        fit_id: &SolFitId,
    ) -> Vec<SolCtxModifier> {
        let mut raw_modifiers = Vec::new();
        let mut ctx_modifiers = Vec::new();
        // Outgoing fleet boosts
        raw_modifiers.extend(self.rmods_fleet.get(fit_id).map(|v| *v));
        for raw_modifier in raw_modifiers.iter() {
            for fleet_fit_id in fleet.iter_fits() {
                if fleet_fit_id == fit_id {
                    continue;
                }
                if let Some(ctx_modifier) = self.unapply_fleet_mod(*raw_modifier, *fleet_fit_id) {
                    ctx_modifiers.push(ctx_modifier);
                }
            }
        }
        // Incoming fleet boosts
        for fleet_fit_id in fleet.iter_fits() {
            if fleet_fit_id == fit_id {
                continue;
            }
            raw_modifiers.clear();
            raw_modifiers.extend(self.rmods_fleet.get(fleet_fit_id).map(|v| *v));
            for raw_modifier in raw_modifiers.iter() {
                if let Some(ctx_modifier) = self.unapply_fleet_mod(*raw_modifier, *fit_id) {
                    ctx_modifiers.push(ctx_modifier);
                }
            }
        }
        ctx_modifiers
    }
    // Private methods
    fn apply_fleet_mod(&mut self, raw_modifier: SolRawModifier, fit_id: SolFitId) -> Option<SolCtxModifier> {
        match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolDomain::Ship => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                    add_ctx_modifier(
                        &mut self.cmods_root,
                        (fit_id, SolLocationKind::Ship),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolDomain::Ship => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                    add_ctx_modifier(
                        &mut self.cmods_loc,
                        (fit_id, SolLocationKind::Ship),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolDomain::Ship => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                    add_ctx_modifier(
                        &mut self.cmods_loc_grp,
                        (fit_id, SolLocationKind::Ship, grp_id),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolDomain::Ship => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                    add_ctx_modifier(
                        &mut self.cmods_loc_srq,
                        (fit_id, SolLocationKind::Ship, srq_id),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
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
                remove_ctx_modifier(
                    &mut self.cmods_root,
                    &(fit_id, SolLocationKind::Ship),
                    &ctx_modifier,
                    &mut self.cmods_by_attr_spec,
                );
                Some(ctx_modifier)
            }
            SolAffecteeFilter::Loc(_) => {
                let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                remove_ctx_modifier(
                    &mut self.cmods_loc,
                    &(fit_id, SolLocationKind::Ship),
                    &ctx_modifier,
                    &mut self.cmods_by_attr_spec,
                );
                Some(ctx_modifier)
            }
            SolAffecteeFilter::LocGrp(_, grp_id) => {
                let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                remove_ctx_modifier(
                    &mut self.cmods_loc_grp,
                    &(fit_id, SolLocationKind::Ship, grp_id),
                    &ctx_modifier,
                    &mut self.cmods_by_attr_spec,
                );
                Some(ctx_modifier)
            }
            SolAffecteeFilter::LocSrq(_, srq_id) => {
                let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                remove_ctx_modifier(
                    &mut self.cmods_loc_srq,
                    &(fit_id, SolLocationKind::Ship, srq_id),
                    &ctx_modifier,
                    &mut self.cmods_by_attr_spec,
                );
                Some(ctx_modifier)
            }
            _ => None,
        }
    }
}
