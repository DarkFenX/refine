use crate::{
    svc::calc::{
        AffecteeFilter, CtxModifier, RawModifier,
        registers::standard::{
            StandardRegister,
            func::{add_cmod, remove_cmod},
        },
    },
    ud::UFwEffect,
};

impl StandardRegister {
    pub(in crate::svc::calc) fn reg_fw_system_mod(
        &mut self,
        fw_effect: &UFwEffect,
        rmod: RawModifier,
    ) -> Option<CtxModifier> {
        let cmod = match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) => {
                let loc_kind = loc.try_into().ok()?;
                let fit_key = fw_effect.get_fit_key();
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, loc_kind);
                add_cmod(&mut self.cmods.root, key, cmod, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::Loc(loc) => {
                let loc_kind = loc.try_into().ok()?;
                let fit_key = fw_effect.get_fit_key();
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, loc_kind);
                add_cmod(&mut self.cmods.loc, key, cmod, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::LocGrp(loc, item_grp_id) => {
                let loc_kind = loc.try_into().ok()?;
                let fit_key = fw_effect.get_fit_key();
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, loc_kind, item_grp_id);
                add_cmod(&mut self.cmods.loc_grp, key, cmod, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::LocSrq(loc, srq_type_id) => {
                let loc_kind = loc.try_into().ok()?;
                let fit_key = fw_effect.get_fit_key();
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, loc_kind, srq_type_id);
                add_cmod(&mut self.cmods.loc_srq, key, cmod, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::OwnSrq(srq_type_id) => {
                let fit_key = fw_effect.get_fit_key();
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, srq_type_id);
                add_cmod(&mut self.cmods.own_srq, key, cmod, &mut self.cmods.by_aspec);
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
        fw_effect: &UFwEffect,
        rmod: RawModifier,
    ) -> Option<CtxModifier> {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) => {
                let loc_kind = loc.try_into().ok()?;
                let fit_key = fw_effect.get_fit_key();
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, loc_kind);
                remove_cmod(&mut self.cmods.root, key, &cmod, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::Loc(loc) => {
                let loc_kind = loc.try_into().ok()?;
                let fit_key = fw_effect.get_fit_key();
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, loc_kind);
                remove_cmod(&mut self.cmods.loc, key, &cmod, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::LocGrp(loc, item_grp_id) => {
                let loc_kind = loc.try_into().ok()?;
                let fit_key = fw_effect.get_fit_key();
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, loc_kind, item_grp_id);
                remove_cmod(&mut self.cmods.loc_grp, key, &cmod, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::LocSrq(loc, srq_type_id) => {
                let loc_kind = loc.try_into().ok()?;
                let fit_key = fw_effect.get_fit_key();
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, loc_kind, srq_type_id);
                remove_cmod(&mut self.cmods.loc_srq, key, &cmod, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::OwnSrq(srq_type_id) => {
                let fit_key = fw_effect.get_fit_key();
                let cmod = CtxModifier::new_with_fit(rmod, fit_key);
                let key = (fit_key, srq_type_id);
                remove_cmod(&mut self.cmods.own_srq, key, &cmod, &mut self.cmods.by_aspec);
                Some(cmod)
            }
        }
    }
}
