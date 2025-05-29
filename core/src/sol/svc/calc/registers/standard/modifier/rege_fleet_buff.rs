use super::{add_ctx_modifier, remove_ctx_modifier};
use crate::sol::{
    FitKey,
    svc::calc::{AffecteeFilter, CtxModifier, Location, LocationKind, RawModifier, registers::StandardRegister},
    uad::{Uad, fleet::UadFleet, item::UadItem},
};

impl StandardRegister {
    pub(in crate::sol::svc::calc) fn reg_fleet_buff_mod(
        &mut self,
        ctx_modifiers: &mut Vec<CtxModifier>,
        uad: &Uad,
        item: &UadItem,
        raw_modifier: RawModifier,
    ) -> bool {
        ctx_modifiers.clear();
        let fit_key = match item.get_fit_key() {
            Some(fit_key) => fit_key,
            None => return false,
        };
        let affector_fit = uad.fits.get(fit_key);
        match affector_fit.fleet {
            Some(fleet_key) => {
                let fleet = uad.fleets.get(fleet_key);
                for &fleet_fit_key in fleet.iter_fits() {
                    if let Some(ctx_modifier) = self.apply_fleet_mod(raw_modifier, fleet_fit_key) {
                        ctx_modifiers.push(ctx_modifier);
                    }
                }
            }
            None => {
                if let Some(ctx_modifier) = self.apply_fleet_mod(raw_modifier, fit_key) {
                    ctx_modifiers.push(ctx_modifier);
                }
            }
        }
        // Here, we can rely on presence of ctx modifiers, because there is always a fit we will go
        // through when adding them; if raw modifier is valid, there will always be a ctx one
        if !ctx_modifiers.is_empty() {
            self.rmods_fleet.add_entry(fit_key, raw_modifier);
            self.rmods_nonproj.add_entry(raw_modifier.affector_espec, raw_modifier);
        }
        !ctx_modifiers.is_empty()
    }
    pub(in crate::sol::svc::calc) fn unreg_fleet_buff_mod(
        &mut self,
        ctx_modifiers: &mut Vec<CtxModifier>,
        uad: &Uad,
        item: &UadItem,
        raw_modifier: RawModifier,
    ) {
        ctx_modifiers.clear();
        let fit_key = match item.get_fit_key() {
            Some(fit_key) => fit_key,
            None => return,
        };
        let affector_fit = uad.fits.get(fit_key);
        match affector_fit.fleet {
            Some(fleet_key) => {
                let fleet = uad.fleets.get(fleet_key);
                for &fleet_fit_key in fleet.iter_fits() {
                    if let Some(ctx_modifier) = self.unapply_fleet_mod(raw_modifier, fleet_fit_key) {
                        ctx_modifiers.push(ctx_modifier);
                    }
                }
            }
            None => {
                if let Some(ctx_modifier) = self.unapply_fleet_mod(raw_modifier, fit_key) {
                    ctx_modifiers.push(ctx_modifier);
                }
            }
        }
        self.rmods_fleet.remove_entry(&fit_key, &raw_modifier);
    }
    pub(in crate::sol::svc::calc) fn reg_fleet_for_fit(
        &mut self,
        fleet: &UadFleet,
        fit_key: &FitKey,
    ) -> Vec<CtxModifier> {
        let mut raw_modifiers = Vec::new();
        let mut ctx_modifiers = Vec::new();
        // Outgoing fleet boosts
        raw_modifiers.extend(self.rmods_fleet.get(fit_key).copied());
        for raw_modifier in raw_modifiers.iter() {
            for fleet_fit_key in fleet.iter_fits() {
                if fleet_fit_key == fit_key {
                    continue;
                }
                if let Some(ctx_modifier) = self.apply_fleet_mod(*raw_modifier, *fleet_fit_key) {
                    ctx_modifiers.push(ctx_modifier);
                }
            }
        }
        // Incoming fleet boosts
        for fleet_fit_key in fleet.iter_fits() {
            if fleet_fit_key == fit_key {
                continue;
            }
            raw_modifiers.clear();
            raw_modifiers.extend(self.rmods_fleet.get(fleet_fit_key).copied());
            for raw_modifier in raw_modifiers.iter() {
                if let Some(ctx_modifier) = self.apply_fleet_mod(*raw_modifier, *fit_key) {
                    ctx_modifiers.push(ctx_modifier);
                }
            }
        }
        ctx_modifiers
    }
    pub(in crate::sol::svc::calc) fn unreg_fleet_for_fit(
        &mut self,
        fleet: &UadFleet,
        fit_key: &FitKey,
    ) -> Vec<CtxModifier> {
        let mut raw_modifiers = Vec::new();
        let mut ctx_modifiers = Vec::new();
        // Outgoing fleet boosts
        raw_modifiers.extend(self.rmods_fleet.get(fit_key).copied());
        for raw_modifier in raw_modifiers.iter() {
            for fleet_fit_key in fleet.iter_fits() {
                if fleet_fit_key == fit_key {
                    continue;
                }
                if let Some(ctx_modifier) = self.unapply_fleet_mod(*raw_modifier, *fleet_fit_key) {
                    ctx_modifiers.push(ctx_modifier);
                }
            }
        }
        // Incoming fleet boosts
        for fleet_fit_key in fleet.iter_fits() {
            if fleet_fit_key == fit_key {
                continue;
            }
            raw_modifiers.clear();
            raw_modifiers.extend(self.rmods_fleet.get(fleet_fit_key).copied());
            for raw_modifier in raw_modifiers.iter() {
                if let Some(ctx_modifier) = self.unapply_fleet_mod(*raw_modifier, *fit_key) {
                    ctx_modifiers.push(ctx_modifier);
                }
            }
        }
        ctx_modifiers
    }
    // Private methods
    fn apply_fleet_mod(&mut self, raw_modifier: RawModifier, fit_key: FitKey) -> Option<CtxModifier> {
        match raw_modifier.affectee_filter {
            AffecteeFilter::Direct(Location::Ship) => {
                let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fit_key);
                add_ctx_modifier(
                    &mut self.cmods_root,
                    (fit_key, LocationKind::Ship),
                    ctx_modifier,
                    &mut self.cmods_by_attr_spec,
                );
                Some(ctx_modifier)
            }
            AffecteeFilter::Loc(Location::Ship) => {
                let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fit_key);
                add_ctx_modifier(
                    &mut self.cmods_loc,
                    (fit_key, LocationKind::Ship),
                    ctx_modifier,
                    &mut self.cmods_by_attr_spec,
                );
                Some(ctx_modifier)
            }
            AffecteeFilter::LocGrp(Location::Ship, a_item_grp_id) => {
                let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fit_key);
                add_ctx_modifier(
                    &mut self.cmods_loc_grp,
                    (fit_key, LocationKind::Ship, a_item_grp_id),
                    ctx_modifier,
                    &mut self.cmods_by_attr_spec,
                );
                Some(ctx_modifier)
            }
            AffecteeFilter::LocSrq(Location::Ship, srq_a_item_id) => {
                let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fit_key);
                add_ctx_modifier(
                    &mut self.cmods_loc_srq,
                    (fit_key, LocationKind::Ship, srq_a_item_id),
                    ctx_modifier,
                    &mut self.cmods_by_attr_spec,
                );
                Some(ctx_modifier)
            }
            _ => None,
        }
    }
    fn unapply_fleet_mod(&mut self, raw_modifier: RawModifier, fit_key: FitKey) -> Option<CtxModifier> {
        // We don't check location here, since logic on layers above ensures we receive only
        // modifiers which passed checks when they were added, and location check is part of those
        match raw_modifier.affectee_filter {
            AffecteeFilter::Direct(_) => {
                let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fit_key);
                remove_ctx_modifier(
                    &mut self.cmods_root,
                    &(fit_key, LocationKind::Ship),
                    &ctx_modifier,
                    &mut self.cmods_by_attr_spec,
                );
                Some(ctx_modifier)
            }
            AffecteeFilter::Loc(_) => {
                let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fit_key);
                remove_ctx_modifier(
                    &mut self.cmods_loc,
                    &(fit_key, LocationKind::Ship),
                    &ctx_modifier,
                    &mut self.cmods_by_attr_spec,
                );
                Some(ctx_modifier)
            }
            AffecteeFilter::LocGrp(_, a_item_grp_id) => {
                let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fit_key);
                remove_ctx_modifier(
                    &mut self.cmods_loc_grp,
                    &(fit_key, LocationKind::Ship, a_item_grp_id),
                    &ctx_modifier,
                    &mut self.cmods_by_attr_spec,
                );
                Some(ctx_modifier)
            }
            AffecteeFilter::LocSrq(_, srq_a_item_id) => {
                let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fit_key);
                remove_ctx_modifier(
                    &mut self.cmods_loc_srq,
                    &(fit_key, LocationKind::Ship, srq_a_item_id),
                    &ctx_modifier,
                    &mut self.cmods_by_attr_spec,
                );
                Some(ctx_modifier)
            }
            _ => None,
        }
    }
}
