use std::convert::TryInto;

use crate::sol::{
    item::SolFwEffect,
    svc::svce_calc::{registers::SolStandardRegister, SolAffecteeFilter, SolCtxModifier, SolRawModifier},
};

use super::{reg_cmod, unreg_cmod};

impl SolStandardRegister {
    pub(in crate::sol::svc::svce_calc) fn reg_fw_system_mod(
        &mut self,
        fw_effect: &SolFwEffect,
        raw_modifier: SolRawModifier,
    ) -> Option<SolCtxModifier> {
        let ctx_modifier = match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom.try_into() {
                Ok(loc) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fw_effect.fit_id);
                    self.cmods_root.add_entry((fw_effect.fit_id, loc), ctx_modifier);
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::Loc(dom) => match dom.try_into() {
                Ok(loc) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fw_effect.fit_id);
                    self.cmods_loc.add_entry((fw_effect.fit_id, loc), ctx_modifier);
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom.try_into() {
                Ok(loc) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fw_effect.fit_id);
                    self.cmods_loc_grp
                        .add_entry((fw_effect.fit_id, loc, grp_id), ctx_modifier);
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom.try_into() {
                Ok(loc) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fw_effect.fit_id);
                    self.cmods_loc_srq
                        .add_entry((fw_effect.fit_id, loc, srq_id), ctx_modifier);
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::OwnSrq(srq_id) => {
                let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fw_effect.fit_id);
                self.cmods_own_srq.add_entry((fw_effect.fit_id, srq_id), ctx_modifier);
                Some(ctx_modifier)
            }
        };
        if let Some(ctx_modifier) = ctx_modifier {
            self.rmods_nonproj
                .add_entry((raw_modifier.affector_item_id, raw_modifier.effect_id), raw_modifier);
            reg_cmod(&mut self.cmods_by_attr_spec, ctx_modifier);
        }
        ctx_modifier
    }
    pub(in crate::sol::svc::svce_calc) fn unreg_fw_system_mod(
        &mut self,
        fw_effect: &SolFwEffect,
        raw_modifier: SolRawModifier,
    ) -> Option<SolCtxModifier> {
        let ctx_modifier = match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom.try_into() {
                Ok(loc) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fw_effect.fit_id);
                    self.cmods_root.remove_entry(&(fw_effect.fit_id, loc), &ctx_modifier);
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::Loc(dom) => match dom.try_into() {
                Ok(loc) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fw_effect.fit_id);
                    self.cmods_loc.remove_entry(&(fw_effect.fit_id, loc), &ctx_modifier);
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom.try_into() {
                Ok(loc) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fw_effect.fit_id);
                    self.cmods_loc_grp
                        .remove_entry(&(fw_effect.fit_id, loc, grp_id), &ctx_modifier);
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom.try_into() {
                Ok(loc) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fw_effect.fit_id);
                    self.cmods_loc_srq
                        .remove_entry(&(fw_effect.fit_id, loc, srq_id), &ctx_modifier);
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::OwnSrq(srq_id) => {
                let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fw_effect.fit_id);
                self.cmods_own_srq
                    .remove_entry(&(fw_effect.fit_id, srq_id), &ctx_modifier);
                Some(ctx_modifier)
            }
        };
        if let Some(ctx_modifier) = ctx_modifier {
            unreg_cmod(&mut self.cmods_by_attr_spec, &ctx_modifier);
        }
        ctx_modifier
    }
}
