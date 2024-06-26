use std::convert::TryInto;

use crate::sol::{
    item::SolItem,
    svc::svce_calc::{registers::SolStandardRegister, SolAffecteeFilter, SolCtxModifier, SolDomain, SolRawModifier},
};

use super::{reg_cmod, unreg_cmod};

impl SolStandardRegister {
    pub(in crate::sol::svc::svce_calc) fn reg_local_mod(
        &mut self,
        item: &SolItem,
        raw_modifier: SolRawModifier,
    ) -> Option<SolCtxModifier> {
        let ctx_modifier = match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolDomain::Item => {
                    let ctx_modifier = SolCtxModifier::from_raw(raw_modifier);
                    self.cmods_direct
                        .add_entry(ctx_modifier.raw.affector_item_id, ctx_modifier);
                    Some(ctx_modifier)
                }
                SolDomain::Other => {
                    let ctx_modifier = SolCtxModifier::from_raw(raw_modifier);
                    self.cmods_other
                        .add_entry(ctx_modifier.raw.affector_item_id, ctx_modifier);
                    Some(ctx_modifier)
                }
                _ => match dom.try_into() {
                    Ok(loc) => match item.get_fit_id() {
                        Some(fit_id) => {
                            let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                            self.cmods_root.add_entry((fit_id, loc), ctx_modifier);
                            Some(ctx_modifier)
                        }
                        None => None,
                    },
                    _ => None,
                },
            },
            SolAffecteeFilter::Loc(dom) => match dom.try_into() {
                Ok(loc) => match item.get_fit_id() {
                    Some(fit_id) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                        self.cmods_loc.add_entry((fit_id, loc), ctx_modifier);
                        Some(ctx_modifier)
                    }
                    None => None,
                },
                _ => None,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom.try_into() {
                Ok(loc) => match item.get_fit_id() {
                    Some(fit_id) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                        self.cmods_loc_grp.add_entry((fit_id, loc, grp_id), ctx_modifier);
                        Some(ctx_modifier)
                    }
                    None => None,
                },
                _ => None,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom.try_into() {
                Ok(loc) => match item.get_fit_id() {
                    Some(fit_id) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                        self.cmods_loc_srq.add_entry((fit_id, loc, srq_id), ctx_modifier);
                        Some(ctx_modifier)
                    }
                    None => None,
                },
                _ => None,
            },
            SolAffecteeFilter::OwnSrq(srq_id) => match item.get_fit_id() {
                Some(fit_id) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                    self.cmods_own_srq.add_entry((fit_id, srq_id), ctx_modifier);
                    Some(ctx_modifier)
                }
                None => None,
            },
        };
        // If we received a modifier with context, it means that raw modifier was valid
        if let Some(ctx_modifier) = ctx_modifier {
            self.rmods_nonproj
                .add_entry((raw_modifier.affector_item_id, raw_modifier.effect_id), raw_modifier);
            reg_cmod(&mut self.cmods_by_attr_spec, ctx_modifier);
        }
        ctx_modifier
    }
    pub(in crate::sol::svc::svce_calc) fn unreg_local_mod(
        &mut self,
        item: &SolItem,
        raw_modifier: SolRawModifier,
    ) -> Option<SolCtxModifier> {
        let ctx_modifier = match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolDomain::Item => {
                    let ctx_modifier = SolCtxModifier::from_raw(raw_modifier);
                    self.cmods_direct
                        .remove_entry(&ctx_modifier.raw.affector_item_id, &ctx_modifier);
                    Some(ctx_modifier)
                }
                SolDomain::Other => {
                    let ctx_modifier = SolCtxModifier::from_raw(raw_modifier);
                    self.cmods_other
                        .remove_entry(&ctx_modifier.raw.affector_item_id, &ctx_modifier);
                    Some(ctx_modifier)
                }
                _ => match dom.try_into() {
                    Ok(loc) => match item.get_fit_id() {
                        Some(fit_id) => {
                            let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                            self.cmods_root.remove_entry(&(fit_id, loc), &ctx_modifier);
                            Some(ctx_modifier)
                        }
                        None => None,
                    },
                    _ => None,
                },
            },
            SolAffecteeFilter::Loc(dom) => match dom.try_into() {
                Ok(loc) => match item.get_fit_id() {
                    Some(fit_id) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                        self.cmods_loc.remove_entry(&(fit_id, loc), &ctx_modifier);
                        Some(ctx_modifier)
                    }
                    None => None,
                },
                _ => None,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom.try_into() {
                Ok(loc) => match item.get_fit_id() {
                    Some(fit_id) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                        self.cmods_loc_grp.remove_entry(&(fit_id, loc, grp_id), &ctx_modifier);
                        Some(ctx_modifier)
                    }
                    None => None,
                },
                _ => None,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom.try_into() {
                Ok(loc) => match item.get_fit_id() {
                    Some(fit_id) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                        self.cmods_loc_srq.remove_entry(&(fit_id, loc, srq_id), &ctx_modifier);
                        Some(ctx_modifier)
                    }
                    None => None,
                },
                _ => None,
            },
            SolAffecteeFilter::OwnSrq(srq_id) => match item.get_fit_id() {
                Some(fit_id) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                    self.cmods_own_srq.remove_entry(&(fit_id, srq_id), &ctx_modifier);
                    Some(ctx_modifier)
                }
                None => None,
            },
        };
        if let Some(ctx_modifier) = ctx_modifier {
            unreg_cmod(&mut self.cmods_by_attr_spec, &ctx_modifier);
        }
        ctx_modifier
    }
}
