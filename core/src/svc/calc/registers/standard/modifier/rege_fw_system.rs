use super::{add_ctx_modifier, remove_ctx_modifier};
use crate::{
    svc::calc::{AffecteeFilter, CtxModifier, RawModifier, registers::StandardRegister},
    uad::UadFwEffect,
};

impl StandardRegister {
    pub(in crate::svc::calc) fn reg_fw_system_mod(
        &mut self,
        fw_effect: &UadFwEffect,
        raw_modifier: RawModifier,
    ) -> Option<CtxModifier> {
        let ctx_modifier = match raw_modifier.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc.try_into() {
                Ok(loc_kind) => {
                    let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fw_effect.get_fit_key());
                    add_ctx_modifier(
                        &mut self.cmods_root,
                        (fw_effect.get_fit_key(), loc_kind),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            AffecteeFilter::Loc(loc) => match loc.try_into() {
                Ok(loc_kind) => {
                    let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fw_effect.get_fit_key());
                    add_ctx_modifier(
                        &mut self.cmods_loc,
                        (fw_effect.get_fit_key(), loc_kind),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            AffecteeFilter::LocGrp(loc, a_item_grp_id) => match loc.try_into() {
                Ok(loc_kind) => {
                    let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fw_effect.get_fit_key());
                    add_ctx_modifier(
                        &mut self.cmods_loc_grp,
                        (fw_effect.get_fit_key(), loc_kind, a_item_grp_id),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            AffecteeFilter::LocSrq(loc, srq_a_item_id) => match loc.try_into() {
                Ok(loc_kind) => {
                    let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fw_effect.get_fit_key());
                    add_ctx_modifier(
                        &mut self.cmods_loc_srq,
                        (fw_effect.get_fit_key(), loc_kind, srq_a_item_id),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            AffecteeFilter::OwnSrq(srq_a_item_id) => {
                let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fw_effect.get_fit_key());
                add_ctx_modifier(
                    &mut self.cmods_own_srq,
                    (fw_effect.get_fit_key(), srq_a_item_id),
                    ctx_modifier,
                    &mut self.cmods_by_attr_spec,
                );
                Some(ctx_modifier)
            }
        };
        // If context modifier was returned = raw was valid
        if ctx_modifier.is_some() {
            self.rmods_all.add_entry(raw_modifier.affector_espec, raw_modifier);
        }
        ctx_modifier
    }
    pub(in crate::svc::calc) fn unreg_fw_system_mod(
        &mut self,
        fw_effect: &UadFwEffect,
        raw_modifier: RawModifier,
    ) -> Option<CtxModifier> {
        match raw_modifier.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc.try_into() {
                Ok(loc_kind) => {
                    let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fw_effect.get_fit_key());
                    remove_ctx_modifier(
                        &mut self.cmods_root,
                        &(fw_effect.get_fit_key(), loc_kind),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            AffecteeFilter::Loc(loc) => match loc.try_into() {
                Ok(loc_kind) => {
                    let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fw_effect.get_fit_key());
                    remove_ctx_modifier(
                        &mut self.cmods_loc,
                        &(fw_effect.get_fit_key(), loc_kind),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            AffecteeFilter::LocGrp(loc, a_item_grp_id) => match loc.try_into() {
                Ok(loc_kind) => {
                    let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fw_effect.get_fit_key());
                    remove_ctx_modifier(
                        &mut self.cmods_loc_grp,
                        &(fw_effect.get_fit_key(), loc_kind, a_item_grp_id),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            AffecteeFilter::LocSrq(loc, srq_a_item_id) => match loc.try_into() {
                Ok(loc_kind) => {
                    let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fw_effect.get_fit_key());
                    remove_ctx_modifier(
                        &mut self.cmods_loc_srq,
                        &(fw_effect.get_fit_key(), loc_kind, srq_a_item_id),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            AffecteeFilter::OwnSrq(srq_a_item_id) => {
                let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fw_effect.get_fit_key());
                remove_ctx_modifier(
                    &mut self.cmods_own_srq,
                    &(fw_effect.get_fit_key(), srq_a_item_id),
                    &ctx_modifier,
                    &mut self.cmods_by_attr_spec,
                );
                Some(ctx_modifier)
            }
        }
    }
}
