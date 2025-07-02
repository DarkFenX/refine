use super::{add_cmod, remove_cmod};
use crate::{
    svc::calc::{AffecteeFilter, CtxModifier, RawModifier, registers::StandardRegister},
    uad::UadFwEffect,
};

impl StandardRegister {
    pub(in crate::svc::calc) fn reg_fw_system_mod(
        &mut self,
        fw_effect: &UadFwEffect,
        rmod: RawModifier,
    ) -> Option<CtxModifier> {
        let cmod = match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc.try_into() {
                Ok(loc_kind) => {
                    let cmod = CtxModifier::from_raw_with_fit(rmod, fw_effect.get_fit_key());
                    add_cmod(
                        &mut self.cmods_root,
                        (fw_effect.get_fit_key(), loc_kind),
                        cmod,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(cmod)
                }
                _ => None,
            },
            AffecteeFilter::Loc(loc) => match loc.try_into() {
                Ok(loc_kind) => {
                    let cmod = CtxModifier::from_raw_with_fit(rmod, fw_effect.get_fit_key());
                    add_cmod(
                        &mut self.cmods_loc,
                        (fw_effect.get_fit_key(), loc_kind),
                        cmod,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(cmod)
                }
                _ => None,
            },
            AffecteeFilter::LocGrp(loc, a_item_grp_id) => match loc.try_into() {
                Ok(loc_kind) => {
                    let cmod = CtxModifier::from_raw_with_fit(rmod, fw_effect.get_fit_key());
                    add_cmod(
                        &mut self.cmods_loc_grp,
                        (fw_effect.get_fit_key(), loc_kind, a_item_grp_id),
                        cmod,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(cmod)
                }
                _ => None,
            },
            AffecteeFilter::LocSrq(loc, srq_a_item_id) => match loc.try_into() {
                Ok(loc_kind) => {
                    let cmod = CtxModifier::from_raw_with_fit(rmod, fw_effect.get_fit_key());
                    add_cmod(
                        &mut self.cmods_loc_srq,
                        (fw_effect.get_fit_key(), loc_kind, srq_a_item_id),
                        cmod,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(cmod)
                }
                _ => None,
            },
            AffecteeFilter::OwnSrq(srq_a_item_id) => {
                let cmod = CtxModifier::from_raw_with_fit(rmod, fw_effect.get_fit_key());
                add_cmod(
                    &mut self.cmods_own_srq,
                    (fw_effect.get_fit_key(), srq_a_item_id),
                    cmod,
                    &mut self.cmods_by_attr_spec,
                );
                Some(cmod)
            }
        };
        // If context modifier was returned = raw was valid
        if cmod.is_some() {
            self.rmods_all.add_entry(rmod.affector_espec, rmod);
        }
        cmod
    }
    pub(in crate::svc::calc) fn unreg_fw_system_mod(
        &mut self,
        fw_effect: &UadFwEffect,
        rmod: RawModifier,
    ) -> Option<CtxModifier> {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc.try_into() {
                Ok(loc_kind) => {
                    let cmod = CtxModifier::from_raw_with_fit(rmod, fw_effect.get_fit_key());
                    remove_cmod(
                        &mut self.cmods_root,
                        &(fw_effect.get_fit_key(), loc_kind),
                        &cmod,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(cmod)
                }
                _ => None,
            },
            AffecteeFilter::Loc(loc) => match loc.try_into() {
                Ok(loc_kind) => {
                    let cmod = CtxModifier::from_raw_with_fit(rmod, fw_effect.get_fit_key());
                    remove_cmod(
                        &mut self.cmods_loc,
                        &(fw_effect.get_fit_key(), loc_kind),
                        &cmod,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(cmod)
                }
                _ => None,
            },
            AffecteeFilter::LocGrp(loc, a_item_grp_id) => match loc.try_into() {
                Ok(loc_kind) => {
                    let cmod = CtxModifier::from_raw_with_fit(rmod, fw_effect.get_fit_key());
                    remove_cmod(
                        &mut self.cmods_loc_grp,
                        &(fw_effect.get_fit_key(), loc_kind, a_item_grp_id),
                        &cmod,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(cmod)
                }
                _ => None,
            },
            AffecteeFilter::LocSrq(loc, srq_a_item_id) => match loc.try_into() {
                Ok(loc_kind) => {
                    let cmod = CtxModifier::from_raw_with_fit(rmod, fw_effect.get_fit_key());
                    remove_cmod(
                        &mut self.cmods_loc_srq,
                        &(fw_effect.get_fit_key(), loc_kind, srq_a_item_id),
                        &cmod,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(cmod)
                }
                _ => None,
            },
            AffecteeFilter::OwnSrq(srq_a_item_id) => {
                let cmod = CtxModifier::from_raw_with_fit(rmod, fw_effect.get_fit_key());
                remove_cmod(
                    &mut self.cmods_own_srq,
                    &(fw_effect.get_fit_key(), srq_a_item_id),
                    &cmod,
                    &mut self.cmods_by_attr_spec,
                );
                Some(cmod)
            }
        }
    }
}
