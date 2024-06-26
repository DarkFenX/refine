use std::convert::TryInto;

use crate::{
    sol::{
        svc::svce_calc::{registers::SolStandardRegister, SolAffecteeFilter, SolCtxModifier, SolRawModifier},
        SolView,
    },
    SolFitId,
};

use super::{reg_cmod, unreg_cmod};

impl SolStandardRegister {
    pub(in crate::sol::svc::svce_calc) fn reg_sw_system_mod(
        &mut self,
        ctx_modifiers: &mut Vec<SolCtxModifier>,
        sol_view: &SolView,
        raw_modifier: SolRawModifier,
    ) {
        ctx_modifiers.clear();
        let valid = match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom.try_into() {
                Ok(loc) => {
                    ctx_modifiers.reserve_exact(sol_view.fits.len());
                    for fit_id in sol_view.fits.iter_fit_ids() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, *fit_id);
                        ctx_modifiers.push(ctx_modifier);
                        self.cmods_root.add_entry((*fit_id, loc), ctx_modifier);
                    }
                    true
                }
                _ => false,
            },
            SolAffecteeFilter::Loc(dom) => match dom.try_into() {
                Ok(loc) => {
                    ctx_modifiers.reserve_exact(sol_view.fits.len());
                    for fit_id in sol_view.fits.iter_fit_ids() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, *fit_id);
                        ctx_modifiers.push(ctx_modifier);
                        self.cmods_loc.add_entry((*fit_id, loc), ctx_modifier);
                    }
                    true
                }
                _ => false,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom.try_into() {
                Ok(loc) => {
                    ctx_modifiers.reserve_exact(sol_view.fits.len());
                    for fit_id in sol_view.fits.iter_fit_ids() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, *fit_id);
                        ctx_modifiers.push(ctx_modifier);
                        self.cmods_loc_grp.add_entry((*fit_id, loc, grp_id), ctx_modifier);
                    }
                    true
                }
                _ => false,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom.try_into() {
                Ok(loc) => {
                    ctx_modifiers.reserve_exact(sol_view.fits.len());
                    for fit_id in sol_view.fits.iter_fit_ids() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, *fit_id);
                        ctx_modifiers.push(ctx_modifier);
                        self.cmods_loc_srq.add_entry((*fit_id, loc, srq_id), ctx_modifier);
                    }
                    true
                }
                _ => false,
            },
            SolAffecteeFilter::OwnSrq(srq_id) => {
                ctx_modifiers.reserve_exact(sol_view.fits.len());
                for fit_id in sol_view.fits.iter_fit_ids() {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, *fit_id);
                    ctx_modifiers.push(ctx_modifier);
                    self.cmods_own_srq.add_entry((*fit_id, srq_id), ctx_modifier);
                }
                true
            }
        };
        if valid {
            self.rmods_sw_system.insert(raw_modifier);
            self.rmods_nonproj
                .add_entry((raw_modifier.affector_item_id, raw_modifier.effect_id), raw_modifier);
        }
        for ctx_modifier in ctx_modifiers.iter() {
            reg_cmod(&mut self.cmods_by_attr_spec, *ctx_modifier);
        }
    }
    pub(in crate::sol::svc::svce_calc) fn unreg_sw_system_mod(
        &mut self,
        ctx_modifiers: &mut Vec<SolCtxModifier>,
        sol_view: &SolView,
        raw_modifier: SolRawModifier,
    ) {
        ctx_modifiers.clear();
        match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => {
                if let Ok(loc) = dom.try_into() {
                    ctx_modifiers.reserve_exact(sol_view.fits.len());
                    for fit_id in sol_view.fits.iter_fit_ids() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, *fit_id);
                        ctx_modifiers.push(ctx_modifier);
                        self.cmods_root.remove_entry(&(*fit_id, loc), &ctx_modifier)
                    }
                }
            }
            SolAffecteeFilter::Loc(dom) => {
                if let Ok(loc) = dom.try_into() {
                    ctx_modifiers.reserve_exact(sol_view.fits.len());
                    for fit_id in sol_view.fits.iter_fit_ids() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, *fit_id);
                        ctx_modifiers.push(ctx_modifier);
                        self.cmods_loc.remove_entry(&(*fit_id, loc), &ctx_modifier)
                    }
                }
            }
            SolAffecteeFilter::LocGrp(dom, grp_id) => {
                if let Ok(loc) = dom.try_into() {
                    ctx_modifiers.reserve_exact(sol_view.fits.len());
                    for fit_id in sol_view.fits.iter_fit_ids() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, *fit_id);
                        ctx_modifiers.push(ctx_modifier);
                        self.cmods_loc_grp.remove_entry(&(*fit_id, loc, grp_id), &ctx_modifier)
                    }
                }
            }
            SolAffecteeFilter::LocSrq(dom, srq_id) => {
                if let Ok(loc) = dom.try_into() {
                    ctx_modifiers.reserve_exact(sol_view.fits.len());
                    for fit_id in sol_view.fits.iter_fit_ids() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, *fit_id);
                        ctx_modifiers.push(ctx_modifier);
                        self.cmods_loc_srq.remove_entry(&(*fit_id, loc, srq_id), &ctx_modifier)
                    }
                }
            }
            SolAffecteeFilter::OwnSrq(srq_id) => {
                ctx_modifiers.reserve_exact(sol_view.fits.len());
                for fit_id in sol_view.fits.iter_fit_ids() {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, *fit_id);
                    ctx_modifiers.push(ctx_modifier);
                    self.cmods_own_srq.remove_entry(&(*fit_id, srq_id), &ctx_modifier)
                }
            }
        }
        self.rmods_sw_system.remove(&raw_modifier);
        for ctx_modifier in ctx_modifiers.iter() {
            unreg_cmod(&mut self.cmods_by_attr_spec, ctx_modifier);
        }
    }
    // No need to return any ctx modifiers here, since fits being added have no items
    pub(in crate::sol::svc::svce_calc) fn reg_fit_for_sw(&mut self, fit_id: &SolFitId) {
        for raw_modifier in self.rmods_sw_system.iter() {
            match raw_modifier.affectee_filter {
                SolAffecteeFilter::Direct(dom) => {
                    if let Ok(loc) = dom.try_into() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(*raw_modifier, *fit_id);
                        self.cmods_root.add_entry((*fit_id, loc), ctx_modifier);
                        reg_cmod(&mut self.cmods_by_attr_spec, ctx_modifier);
                    }
                }
                SolAffecteeFilter::Loc(dom) => {
                    if let Ok(loc) = dom.try_into() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(*raw_modifier, *fit_id);
                        self.cmods_loc.add_entry((*fit_id, loc), ctx_modifier);
                        reg_cmod(&mut self.cmods_by_attr_spec, ctx_modifier);
                    }
                }
                SolAffecteeFilter::LocGrp(dom, grp_id) => {
                    if let Ok(loc) = dom.try_into() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(*raw_modifier, *fit_id);
                        self.cmods_loc_grp.add_entry((*fit_id, loc, grp_id), ctx_modifier);
                        reg_cmod(&mut self.cmods_by_attr_spec, ctx_modifier);
                    }
                }
                SolAffecteeFilter::LocSrq(dom, srq_id) => {
                    if let Ok(loc) = dom.try_into() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(*raw_modifier, *fit_id);
                        self.cmods_loc_srq.add_entry((*fit_id, loc, srq_id), ctx_modifier);
                        reg_cmod(&mut self.cmods_by_attr_spec, ctx_modifier);
                    }
                }
                SolAffecteeFilter::OwnSrq(srq_id) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(*raw_modifier, *fit_id);
                    self.cmods_own_srq.add_entry((*fit_id, srq_id), ctx_modifier);
                    reg_cmod(&mut self.cmods_by_attr_spec, ctx_modifier);
                }
            }
        }
    }
    // No need to return any ctx modifiers here, since fits being removed have no items
    pub(in crate::sol::svc::svce_calc) fn unreg_fit_for_sw(&mut self, fit_id: &SolFitId) {
        for raw_modifier in self.rmods_sw_system.iter() {
            match raw_modifier.affectee_filter {
                SolAffecteeFilter::Direct(dom) => {
                    if let Ok(loc) = dom.try_into() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(*raw_modifier, *fit_id);
                        self.cmods_root.remove_entry(&(*fit_id, loc), &ctx_modifier);
                        unreg_cmod(&mut self.cmods_by_attr_spec, &ctx_modifier);
                    }
                }
                SolAffecteeFilter::Loc(dom) => {
                    if let Ok(loc) = dom.try_into() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(*raw_modifier, *fit_id);
                        self.cmods_loc.remove_entry(&(*fit_id, loc), &ctx_modifier);
                        unreg_cmod(&mut self.cmods_by_attr_spec, &ctx_modifier);
                    }
                }
                SolAffecteeFilter::LocGrp(dom, grp_id) => {
                    if let Ok(loc) = dom.try_into() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(*raw_modifier, *fit_id);
                        self.cmods_loc_grp.remove_entry(&(*fit_id, loc, grp_id), &ctx_modifier);
                        unreg_cmod(&mut self.cmods_by_attr_spec, &ctx_modifier);
                    }
                }
                SolAffecteeFilter::LocSrq(dom, srq_id) => {
                    if let Ok(loc) = dom.try_into() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(*raw_modifier, *fit_id);
                        self.cmods_loc_srq.remove_entry(&(*fit_id, loc, srq_id), &ctx_modifier);
                        unreg_cmod(&mut self.cmods_by_attr_spec, &ctx_modifier);
                    }
                }
                SolAffecteeFilter::OwnSrq(srq_id) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(*raw_modifier, *fit_id);
                    self.cmods_own_srq.remove_entry(&(*fit_id, srq_id), &ctx_modifier);
                    unreg_cmod(&mut self.cmods_by_attr_spec, &ctx_modifier);
                }
            }
        }
    }
}
