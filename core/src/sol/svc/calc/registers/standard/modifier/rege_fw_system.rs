use crate::sol::{
    svc::calc::{SolAffecteeFilter, SolCtxModifier, SolRawModifier, registers::SolStandardRegister},
    uad::item::SolFwEffect,
};

use super::{add_ctx_modifier, remove_ctx_modifier};

impl SolStandardRegister {
    pub(in crate::sol::svc::calc) fn reg_fw_system_mod(
        &mut self,
        fw_effect: &SolFwEffect,
        raw_modifier: SolRawModifier,
    ) -> Option<SolCtxModifier> {
        let ctx_modifier = match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(loc) => match loc.try_into() {
                Ok(loc_kind) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fw_effect.get_fit_id());
                    add_ctx_modifier(
                        &mut self.cmods_root,
                        (fw_effect.get_fit_id(), loc_kind),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::Loc(loc) => match loc.try_into() {
                Ok(loc_kind) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fw_effect.get_fit_id());
                    add_ctx_modifier(
                        &mut self.cmods_loc,
                        (fw_effect.get_fit_id(), loc_kind),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::LocGrp(loc, grp_id) => match loc.try_into() {
                Ok(loc_kind) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fw_effect.get_fit_id());
                    add_ctx_modifier(
                        &mut self.cmods_loc_grp,
                        (fw_effect.get_fit_id(), loc_kind, grp_id),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::LocSrq(loc, srq_id) => match loc.try_into() {
                Ok(loc_kind) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fw_effect.get_fit_id());
                    add_ctx_modifier(
                        &mut self.cmods_loc_srq,
                        (fw_effect.get_fit_id(), loc_kind, srq_id),
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
    pub(in crate::sol::svc::calc) fn unreg_fw_system_mod(
        &mut self,
        fw_effect: &SolFwEffect,
        raw_modifier: SolRawModifier,
    ) -> Option<SolCtxModifier> {
        match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(loc) => match loc.try_into() {
                Ok(loc_kind) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fw_effect.get_fit_id());
                    remove_ctx_modifier(
                        &mut self.cmods_root,
                        &(fw_effect.get_fit_id(), loc_kind),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::Loc(loc) => match loc.try_into() {
                Ok(loc_kind) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fw_effect.get_fit_id());
                    remove_ctx_modifier(
                        &mut self.cmods_loc,
                        &(fw_effect.get_fit_id(), loc_kind),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::LocGrp(loc, grp_id) => match loc.try_into() {
                Ok(loc_kind) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fw_effect.get_fit_id());
                    remove_ctx_modifier(
                        &mut self.cmods_loc_grp,
                        &(fw_effect.get_fit_id(), loc_kind, grp_id),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            SolAffecteeFilter::LocSrq(loc, srq_id) => match loc.try_into() {
                Ok(loc_kind) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_fit(raw_modifier, fw_effect.get_fit_id());
                    remove_ctx_modifier(
                        &mut self.cmods_loc_srq,
                        &(fw_effect.get_fit_id(), loc_kind, srq_id),
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
