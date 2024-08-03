use crate::sol::{
    item::SolItem,
    svc::svce_calc::{registers::SolStandardRegister, SolAffecteeFilter, SolCtxModifier, SolDomain, SolRawModifier},
};

use super::{add_ctx_modifier, remove_ctx_modifier};

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
                    add_ctx_modifier(
                        &mut self.cmods_direct,
                        ctx_modifier.raw.affector_item_id,
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                SolDomain::Other => {
                    let ctx_modifier = SolCtxModifier::from_raw(raw_modifier);
                    add_ctx_modifier(
                        &mut self.cmods_other,
                        ctx_modifier.raw.affector_item_id,
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => match dom.try_into() {
                    Ok(loc) => match item.get_fit_id() {
                        Some(fit_id) => {
                            let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                            add_ctx_modifier(
                                &mut self.cmods_root,
                                (fit_id, loc),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
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
                        add_ctx_modifier(
                            &mut self.cmods_loc,
                            (fit_id, loc),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
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
                        add_ctx_modifier(
                            &mut self.cmods_loc_grp,
                            (fit_id, loc, grp_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
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
                        add_ctx_modifier(
                            &mut self.cmods_loc_srq,
                            (fit_id, loc, srq_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    None => None,
                },
                _ => None,
            },
            SolAffecteeFilter::OwnSrq(srq_id) => match item.get_fit_id() {
                Some(fit_id) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                    add_ctx_modifier(
                        &mut self.cmods_own_srq,
                        (fit_id, srq_id),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                None => None,
            },
        };
        // If we received a modifier with context, it means that raw modifier was valid
        if ctx_modifier.is_some() {
            self.rmods_nonproj
                .add_entry((raw_modifier.affector_item_id, raw_modifier.effect_id), raw_modifier);
        }
        ctx_modifier
    }
    pub(in crate::sol::svc::svce_calc) fn unreg_local_mod(
        &mut self,
        item: &SolItem,
        raw_modifier: SolRawModifier,
    ) -> Option<SolCtxModifier> {
        match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolDomain::Item => {
                    let ctx_modifier = SolCtxModifier::from_raw(raw_modifier);
                    remove_ctx_modifier(
                        &mut self.cmods_direct,
                        &ctx_modifier.raw.affector_item_id,
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                SolDomain::Other => {
                    let ctx_modifier = SolCtxModifier::from_raw(raw_modifier);
                    remove_ctx_modifier(
                        &mut self.cmods_other,
                        &ctx_modifier.raw.affector_item_id,
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => match dom.try_into() {
                    Ok(loc) => match item.get_fit_id() {
                        Some(fit_id) => {
                            let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                            remove_ctx_modifier(
                                &mut self.cmods_root,
                                &(fit_id, loc),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
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
                        remove_ctx_modifier(
                            &mut self.cmods_loc,
                            &(fit_id, loc),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
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
                        remove_ctx_modifier(
                            &mut self.cmods_loc_grp,
                            &(fit_id, loc, grp_id),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
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
                        remove_ctx_modifier(
                            &mut self.cmods_loc_srq,
                            &(fit_id, loc, srq_id),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    None => None,
                },
                _ => None,
            },
            SolAffecteeFilter::OwnSrq(srq_id) => match item.get_fit_id() {
                Some(fit_id) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fit_id);
                    remove_ctx_modifier(
                        &mut self.cmods_own_srq,
                        &(fit_id, srq_id),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                None => None,
            },
        }
    }
}
