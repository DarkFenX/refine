use crate::{
    sol::{
        svc::calc::{registers::SolStandardRegister, SolAffecteeFilter, SolCtxModifier, SolRawModifier},
        uad::SolUad,
    },
    SolFitId,
};

use super::{add_ctx_modifier, remove_ctx_modifier};

impl SolStandardRegister {
    pub(in crate::sol::svc::calc) fn reg_sw_system_mod(
        &mut self,
        ctx_modifiers: &mut Vec<SolCtxModifier>,
        uad: &SolUad,
        raw_modifier: SolRawModifier,
    ) {
        ctx_modifiers.clear();
        let valid = match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(loc) => match loc.try_into() {
                Ok(loc_kind) => {
                    ctx_modifiers.reserve(uad.fits.len());
                    for fit_id in uad.fits.iter_fit_ids() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, *fit_id);
                        add_ctx_modifier(
                            &mut self.cmods_root,
                            (*fit_id, loc_kind),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                    true
                }
                _ => false,
            },
            SolAffecteeFilter::Loc(loc) => match loc.try_into() {
                Ok(loc_kind) => {
                    ctx_modifiers.reserve(uad.fits.len());
                    for fit_id in uad.fits.iter_fit_ids() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, *fit_id);
                        add_ctx_modifier(
                            &mut self.cmods_loc,
                            (*fit_id, loc_kind),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                    true
                }
                _ => false,
            },
            SolAffecteeFilter::LocGrp(loc, grp_id) => match loc.try_into() {
                Ok(loc_kind) => {
                    ctx_modifiers.reserve(uad.fits.len());
                    for fit_id in uad.fits.iter_fit_ids() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, *fit_id);
                        add_ctx_modifier(
                            &mut self.cmods_loc_grp,
                            (*fit_id, loc_kind, grp_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                    true
                }
                _ => false,
            },
            SolAffecteeFilter::LocSrq(loc, srq_id) => match loc.try_into() {
                Ok(loc_kind) => {
                    ctx_modifiers.reserve(uad.fits.len());
                    for fit_id in uad.fits.iter_fit_ids() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, *fit_id);
                        add_ctx_modifier(
                            &mut self.cmods_loc_srq,
                            (*fit_id, loc_kind, srq_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                    true
                }
                _ => false,
            },
            SolAffecteeFilter::OwnSrq(srq_id) => {
                ctx_modifiers.reserve(uad.fits.len());
                for fit_id in uad.fits.iter_fit_ids() {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, *fit_id);
                    add_ctx_modifier(
                        &mut self.cmods_own_srq,
                        (*fit_id, srq_id),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    ctx_modifiers.push(ctx_modifier);
                }
                true
            }
        };
        if valid {
            self.rmods_sw_system.insert(raw_modifier);
            self.rmods_nonproj
                .add_entry((raw_modifier.affector_item_id, raw_modifier.effect_id), raw_modifier);
        }
    }
    pub(in crate::sol::svc::calc) fn unreg_sw_system_mod(
        &mut self,
        ctx_modifiers: &mut Vec<SolCtxModifier>,
        uad: &SolUad,
        raw_modifier: SolRawModifier,
    ) {
        ctx_modifiers.clear();
        match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(loc) => {
                if let Ok(loc_kind) = loc.try_into() {
                    ctx_modifiers.reserve(uad.fits.len());
                    for fit_id in uad.fits.iter_fit_ids() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, *fit_id);
                        remove_ctx_modifier(
                            &mut self.cmods_root,
                            &(*fit_id, loc_kind),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                }
            }
            SolAffecteeFilter::Loc(loc) => {
                if let Ok(loc_kind) = loc.try_into() {
                    ctx_modifiers.reserve(uad.fits.len());
                    for fit_id in uad.fits.iter_fit_ids() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, *fit_id);
                        remove_ctx_modifier(
                            &mut self.cmods_loc,
                            &(*fit_id, loc_kind),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                }
            }
            SolAffecteeFilter::LocGrp(loc, grp_id) => {
                if let Ok(loc_kind) = loc.try_into() {
                    ctx_modifiers.reserve(uad.fits.len());
                    for fit_id in uad.fits.iter_fit_ids() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, *fit_id);
                        remove_ctx_modifier(
                            &mut self.cmods_loc_grp,
                            &(*fit_id, loc_kind, grp_id),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                }
            }
            SolAffecteeFilter::LocSrq(loc, srq_id) => {
                if let Ok(loc_kind) = loc.try_into() {
                    ctx_modifiers.reserve(uad.fits.len());
                    for fit_id in uad.fits.iter_fit_ids() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, *fit_id);
                        remove_ctx_modifier(
                            &mut self.cmods_loc_srq,
                            &(*fit_id, loc_kind, srq_id),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                }
            }
            SolAffecteeFilter::OwnSrq(srq_id) => {
                ctx_modifiers.reserve(uad.fits.len());
                for fit_id in uad.fits.iter_fit_ids() {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, *fit_id);
                    remove_ctx_modifier(
                        &mut self.cmods_own_srq,
                        &(*fit_id, srq_id),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    ctx_modifiers.push(ctx_modifier);
                }
            }
        }
        self.rmods_sw_system.remove(&raw_modifier);
    }
    // No need to return any ctx modifiers here, since fits being added have no items
    pub(in crate::sol::svc::calc) fn reg_fit_for_sw(&mut self, fit_id: &SolFitId) {
        for raw_modifier in self.rmods_sw_system.iter() {
            match raw_modifier.affectee_filter {
                SolAffecteeFilter::Direct(loc) => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(*raw_modifier, *fit_id);
                        add_ctx_modifier(
                            &mut self.cmods_root,
                            (*fit_id, loc_kind),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                SolAffecteeFilter::Loc(loc) => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(*raw_modifier, *fit_id);
                        add_ctx_modifier(
                            &mut self.cmods_loc,
                            (*fit_id, loc_kind),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                SolAffecteeFilter::LocGrp(loc, grp_id) => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(*raw_modifier, *fit_id);
                        add_ctx_modifier(
                            &mut self.cmods_loc_grp,
                            (*fit_id, loc_kind, grp_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                SolAffecteeFilter::LocSrq(loc, srq_id) => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(*raw_modifier, *fit_id);
                        add_ctx_modifier(
                            &mut self.cmods_loc_srq,
                            (*fit_id, loc_kind, srq_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                SolAffecteeFilter::OwnSrq(srq_id) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(*raw_modifier, *fit_id);
                    add_ctx_modifier(
                        &mut self.cmods_own_srq,
                        (*fit_id, srq_id),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                }
            }
        }
    }
    // No need to return any ctx modifiers here, since fits being removed have no items
    pub(in crate::sol::svc::calc) fn unreg_fit_for_sw(&mut self, fit_id: &SolFitId) {
        for raw_modifier in self.rmods_sw_system.iter() {
            match raw_modifier.affectee_filter {
                SolAffecteeFilter::Direct(loc) => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(*raw_modifier, *fit_id);
                        remove_ctx_modifier(
                            &mut self.cmods_root,
                            &(*fit_id, loc_kind),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                SolAffecteeFilter::Loc(loc) => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(*raw_modifier, *fit_id);
                        remove_ctx_modifier(
                            &mut self.cmods_loc,
                            &(*fit_id, loc_kind),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                SolAffecteeFilter::LocGrp(loc, grp_id) => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(*raw_modifier, *fit_id);
                        remove_ctx_modifier(
                            &mut self.cmods_loc_grp,
                            &(*fit_id, loc_kind, grp_id),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                SolAffecteeFilter::LocSrq(loc, srq_id) => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(*raw_modifier, *fit_id);
                        remove_ctx_modifier(
                            &mut self.cmods_loc_srq,
                            &(*fit_id, loc_kind, srq_id),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                SolAffecteeFilter::OwnSrq(srq_id) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(*raw_modifier, *fit_id);
                    remove_ctx_modifier(
                        &mut self.cmods_own_srq,
                        &(*fit_id, srq_id),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                }
            }
        }
    }
}
