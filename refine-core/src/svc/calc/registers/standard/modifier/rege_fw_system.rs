use crate::{
    svc::calc::{
        AffecteeFilter, CtxModifier, RawModId, RawModifier,
        registers::{
            RegOp,
            standard::{StandardRegister, modifier::func::process_cmod},
        },
    },
    ud::UFwEffect,
};

impl StandardRegister {
    pub(in crate::svc::calc) fn process_fw_system_mod<const OP: RegOp>(
        &mut self,
        fw_effect: &UFwEffect,
        rmod_id: RawModId,
        rmod: RawModifier,
    ) -> Option<CtxModifier> {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) => {
                let loc_kind = loc.try_into().ok()?;
                let fit_uid = fw_effect.get_fit_uid();
                let cmod = CtxModifier::with_fit(rmod, fit_uid);
                let key = (fit_uid, loc_kind, cmod.rmod.affectee_attr_rid);
                process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.root, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::Loc(loc) => {
                let loc_kind = loc.try_into().ok()?;
                let fit_uid = fw_effect.get_fit_uid();
                let cmod = CtxModifier::with_fit(rmod, fit_uid);
                let key = (fit_uid, loc_kind, cmod.rmod.affectee_attr_rid);
                process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.loc, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::LocGrp(loc, item_grp_id) => {
                let loc_kind = loc.try_into().ok()?;
                let fit_uid = fw_effect.get_fit_uid();
                let cmod = CtxModifier::with_fit(rmod, fit_uid);
                let key = (fit_uid, loc_kind, item_grp_id, cmod.rmod.affectee_attr_rid);
                process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.loc_grp, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::LocSrq(loc, srq_type_aid) => {
                let loc_kind = loc.try_into().ok()?;
                let fit_uid = fw_effect.get_fit_uid();
                let cmod = CtxModifier::with_fit(rmod, fit_uid);
                let key = (fit_uid, loc_kind, srq_type_aid, cmod.rmod.affectee_attr_rid);
                process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.loc_srq, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::OwnSrq(srq_type_aid) => {
                let fit_uid = fw_effect.get_fit_uid();
                let cmod = CtxModifier::with_fit(rmod, fit_uid);
                let key = (fit_uid, srq_type_aid, cmod.rmod.affectee_attr_rid);
                process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.own_srq, &mut self.cmods.by_aspec);
                Some(cmod)
            }
        }
    }
}
