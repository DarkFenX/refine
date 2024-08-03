use crate::sol::{
    item::SolFwEffect,
    svc::svce_calc::{registers::SolStandardRegister, SolAffecteeFilter, SolCtxModifier, SolRawModifier},
};

use super::{add_ctx_modifier, remove_ctx_modifier};

impl SolStandardRegister {
    pub(in crate::sol::svc::svce_calc) fn reg_fw_system_mod(
        &mut self,
        fw_effect: &SolFwEffect,
        raw_modifier: SolRawModifier,
    ) -> Option<SolCtxModifier> {
        let ctx_modifier = match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom.try_into() {
                Ok(loc) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fw_effect.get_fit_id());
                    add_ctx_modifier(
                        &mut self.cmods_root,
                        (fw_effect.get_fit_id(), loc),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::Loc(dom) => match dom.try_into() {
                Ok(loc) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fw_effect.get_fit_id());
                    add_ctx_modifier(
                        &mut self.cmods_loc,
                        (fw_effect.get_fit_id(), loc),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom.try_into() {
                Ok(loc) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fw_effect.get_fit_id());
                    add_ctx_modifier(
                        &mut self.cmods_loc_grp,
                        (fw_effect.get_fit_id(), loc, grp_id),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom.try_into() {
                Ok(loc) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fw_effect.get_fit_id());
                    add_ctx_modifier(
                        &mut self.cmods_loc_srq,
                        (fw_effect.get_fit_id(), loc, srq_id),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::OwnSrq(srq_id) => {
                let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fw_effect.get_fit_id());
                add_ctx_modifier(
                    &mut self.cmods_own_srq,
                    (fw_effect.get_fit_id(), srq_id),
                    ctx_modifier,
                    &mut self.cmods_by_attr_spec,
                );
                Some(ctx_modifier)
            }
        };
        // If context modifier was returned = raw was valid
        if ctx_modifier.is_some() {
            self.rmods_nonproj
                .add_entry((raw_modifier.affector_item_id, raw_modifier.effect_id), raw_modifier);
        }
        ctx_modifier
    }
    pub(in crate::sol::svc::svce_calc) fn unreg_fw_system_mod(
        &mut self,
        fw_effect: &SolFwEffect,
        raw_modifier: SolRawModifier,
    ) -> Option<SolCtxModifier> {
        match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom.try_into() {
                Ok(loc) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fw_effect.get_fit_id());
                    remove_ctx_modifier(
                        &mut self.cmods_root,
                        &(fw_effect.get_fit_id(), loc),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::Loc(dom) => match dom.try_into() {
                Ok(loc) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fw_effect.get_fit_id());
                    remove_ctx_modifier(
                        &mut self.cmods_loc,
                        &(fw_effect.get_fit_id(), loc),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom.try_into() {
                Ok(loc) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fw_effect.get_fit_id());
                    remove_ctx_modifier(
                        &mut self.cmods_loc_grp,
                        &(fw_effect.get_fit_id(), loc, grp_id),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom.try_into() {
                Ok(loc) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fw_effect.get_fit_id());
                    remove_ctx_modifier(
                        &mut self.cmods_loc_srq,
                        &(fw_effect.get_fit_id(), loc, srq_id),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::OwnSrq(srq_id) => {
                let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fw_effect.get_fit_id());
                remove_ctx_modifier(
                    &mut self.cmods_own_srq,
                    &(fw_effect.get_fit_id(), srq_id),
                    &ctx_modifier,
                    &mut self.cmods_by_attr_spec,
                );
                Some(ctx_modifier)
            }
        }
    }
}
