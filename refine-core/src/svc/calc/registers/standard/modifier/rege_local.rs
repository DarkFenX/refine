use crate::{
    svc::calc::{
        AffecteeFilter, CtxModifier, Location, RawModifier,
        registers::standard::{
            StandardRegister,
            modifier::func::{add_cmod, remove_cmod},
        },
    },
    ud::UItem,
};

impl StandardRegister {
    pub(in crate::svc::calc) fn reg_local_mod(&mut self, item: &UItem, rmod: RawModifier) -> Option<CtxModifier> {
        let cmod = match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Item => {
                    let cmod = CtxModifier::new(rmod);
                    let key = (cmod.raw.affector_espec.item_uid, cmod.raw.affectee_attr_rid);
                    add_cmod(key, cmod, &mut self.cmods.direct, &mut self.cmods.by_aspec);
                    Some(cmod)
                }
                Location::Other => {
                    let cmod = CtxModifier::new(rmod);
                    let key = (cmod.raw.affector_espec.item_uid, cmod.raw.affectee_attr_rid);
                    add_cmod(key, cmod, &mut self.cmods.other, &mut self.cmods.by_aspec);
                    Some(cmod)
                }
                _ => {
                    let loc_kind = loc.try_into().ok()?;
                    let fit_uid = item.get_fit_uid()?;
                    let cmod = CtxModifier::with_fit(rmod, fit_uid);
                    let key = (fit_uid, loc_kind, cmod.raw.affectee_attr_rid);
                    add_cmod(key, cmod, &mut self.cmods.root, &mut self.cmods.by_aspec);
                    Some(cmod)
                }
            },
            AffecteeFilter::Loc(loc) => {
                let loc_kind = loc.try_into().ok()?;
                let fit_uid = item.get_fit_uid()?;
                let cmod = CtxModifier::with_fit(rmod, fit_uid);
                let key = (fit_uid, loc_kind, cmod.raw.affectee_attr_rid);
                add_cmod(key, cmod, &mut self.cmods.loc, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::LocGrp(loc, item_grp_id) => {
                let loc_kind = loc.try_into().ok()?;
                let fit_uid = item.get_fit_uid()?;
                let cmod = CtxModifier::with_fit(rmod, fit_uid);
                let key = (fit_uid, loc_kind, item_grp_id, cmod.raw.affectee_attr_rid);
                add_cmod(key, cmod, &mut self.cmods.loc_grp, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::LocSrq(loc, srq_type_aid) => {
                let loc_kind = loc.try_into().ok()?;
                let fit_uid = item.get_fit_uid()?;
                let cmod = CtxModifier::with_fit(rmod, fit_uid);
                let key = (fit_uid, loc_kind, srq_type_aid, cmod.raw.affectee_attr_rid);
                add_cmod(key, cmod, &mut self.cmods.loc_srq, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::OwnSrq(srq_type_aid) => {
                let fit_uid = item.get_fit_uid()?;
                let cmod = CtxModifier::with_fit(rmod, fit_uid);
                let key = (fit_uid, srq_type_aid, cmod.raw.affectee_attr_rid);
                add_cmod(key, cmod, &mut self.cmods.own_srq, &mut self.cmods.by_aspec);
                Some(cmod)
            }
        };
        // If we received a modifier with context, it means that raw modifier was valid
        if cmod.is_some() {
            self.rmods_all.add_entry(rmod.affector_espec, rmod);
        }
        cmod
    }
    pub(in crate::svc::calc) fn unreg_local_mod(&mut self, item: &UItem, rmod: RawModifier) -> Option<CtxModifier> {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Item => {
                    let cmod = CtxModifier::new(rmod);
                    let key = (cmod.raw.affector_espec.item_uid, cmod.raw.affectee_attr_rid);
                    remove_cmod(key, &cmod, &mut self.cmods.direct, &mut self.cmods.by_aspec);
                    Some(cmod)
                }
                Location::Other => {
                    let cmod = CtxModifier::new(rmod);
                    let key = (cmod.raw.affector_espec.item_uid, cmod.raw.affectee_attr_rid);
                    remove_cmod(key, &cmod, &mut self.cmods.other, &mut self.cmods.by_aspec);
                    Some(cmod)
                }
                _ => {
                    let loc_kind = loc.try_into().ok()?;
                    let fit_uid = item.get_fit_uid()?;
                    let cmod = CtxModifier::with_fit(rmod, fit_uid);
                    let key = (fit_uid, loc_kind, cmod.raw.affectee_attr_rid);
                    remove_cmod(key, &cmod, &mut self.cmods.root, &mut self.cmods.by_aspec);
                    Some(cmod)
                }
            },
            AffecteeFilter::Loc(loc) => {
                let loc_kind = loc.try_into().ok()?;
                let fit_uid = item.get_fit_uid()?;
                let cmod = CtxModifier::with_fit(rmod, fit_uid);
                let key = (fit_uid, loc_kind, cmod.raw.affectee_attr_rid);
                remove_cmod(key, &cmod, &mut self.cmods.loc, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::LocGrp(loc, item_grp_id) => {
                let loc_kind = loc.try_into().ok()?;
                let fit_uid = item.get_fit_uid()?;
                let cmod = CtxModifier::with_fit(rmod, fit_uid);
                let key = (fit_uid, loc_kind, item_grp_id, cmod.raw.affectee_attr_rid);
                remove_cmod(key, &cmod, &mut self.cmods.loc_grp, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::LocSrq(loc, srq_type_aid) => {
                let loc_kind = loc.try_into().ok()?;
                let fit_uid = item.get_fit_uid()?;
                let cmod = CtxModifier::with_fit(rmod, fit_uid);
                let key = (fit_uid, loc_kind, srq_type_aid, cmod.raw.affectee_attr_rid);
                remove_cmod(key, &cmod, &mut self.cmods.loc_srq, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::OwnSrq(srq_type_aid) => {
                let fit_uid = item.get_fit_uid()?;
                let cmod = CtxModifier::with_fit(rmod, fit_uid);
                let key = (fit_uid, srq_type_aid, cmod.raw.affectee_attr_rid);
                remove_cmod(key, &cmod, &mut self.cmods.own_srq, &mut self.cmods.by_aspec);
                Some(cmod)
            }
        }
    }
}
