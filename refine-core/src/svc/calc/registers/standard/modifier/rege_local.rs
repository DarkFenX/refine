use crate::{
    svc::calc::{
        AffecteeFilter, CtxModifier, Location, LocationKind, RawModId, RawModifier,
        registers::{
            RegOp,
            standard::{StandardRegister, modifier::func::process_cmod},
        },
    },
    ud::UItem,
};

impl StandardRegister {
    // Keep in sync with query_local_mod: affector attribute changes find local template modifiers
    // through it, not through a register, so both have to build the same context modifier and skip
    // the same raw modifiers
    pub(in crate::svc::calc) fn process_local_mod<const OP: RegOp>(
        &mut self,
        item: &UItem,
        rmod_id: RawModId,
        rmod: RawModifier,
    ) -> Option<CtxModifier> {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Item => {
                    let cmod = CtxModifier::new(rmod);
                    let key = (cmod.rmod.affector_espec.item_uid, cmod.rmod.affectee_attr_rid);
                    process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.direct, &mut self.cmods.by_aspec);
                    Some(cmod)
                }
                Location::Other => {
                    let cmod = CtxModifier::new(rmod);
                    let key = (cmod.rmod.affector_espec.item_uid, cmod.rmod.affectee_attr_rid);
                    process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.other, &mut self.cmods.by_aspec);
                    Some(cmod)
                }
                _ => {
                    let loc_kind = loc.try_into().ok()?;
                    let fit_uid = item.get_fit_uid()?;
                    let cmod = CtxModifier::with_fit(rmod, fit_uid);
                    let key = (fit_uid, loc_kind, cmod.rmod.affectee_attr_rid);
                    process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.root, &mut self.cmods.by_aspec);
                    Some(cmod)
                }
            },
            AffecteeFilter::Loc(loc) => {
                let loc_kind = loc.try_into().ok()?;
                let fit_uid = item.get_fit_uid()?;
                let cmod = CtxModifier::with_fit(rmod, fit_uid);
                let key = (fit_uid, loc_kind, cmod.rmod.affectee_attr_rid);
                process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.loc, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::LocGrp(loc, item_grp_id) => {
                let loc_kind = loc.try_into().ok()?;
                let fit_uid = item.get_fit_uid()?;
                let cmod = CtxModifier::with_fit(rmod, fit_uid);
                let key = (fit_uid, loc_kind, item_grp_id, cmod.rmod.affectee_attr_rid);
                process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.loc_grp, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::LocSrq(loc, srq_type_aid) => {
                let loc_kind = loc.try_into().ok()?;
                let fit_uid = item.get_fit_uid()?;
                let cmod = CtxModifier::with_fit(rmod, fit_uid);
                let key = (fit_uid, loc_kind, srq_type_aid, cmod.rmod.affectee_attr_rid);
                process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.loc_srq, &mut self.cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::OwnSrq(srq_type_aid) => {
                let fit_uid = item.get_fit_uid()?;
                let cmod = CtxModifier::with_fit(rmod, fit_uid);
                let key = (fit_uid, srq_type_aid, cmod.rmod.affectee_attr_rid);
                process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.own_srq, &mut self.cmods.by_aspec);
                Some(cmod)
            }
        }
    }
    // Context modifier the same registration produces, without registering it; has to stay in sync
    // with process_local_mod (same context modifiers, same skipped raw modifiers)
    pub(in crate::svc::calc) fn query_local_mod(item: &UItem, rmod: RawModifier) -> Option<CtxModifier> {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(Location::Item | Location::Other) => Some(CtxModifier::new(rmod)),
            AffecteeFilter::Direct(loc)
            | AffecteeFilter::Loc(loc)
            | AffecteeFilter::LocGrp(loc, _)
            | AffecteeFilter::LocSrq(loc, _) => {
                let _: LocationKind = loc.try_into().ok()?;
                Some(CtxModifier::with_fit(rmod, item.get_fit_uid()?))
            }
            AffecteeFilter::OwnSrq(_) => Some(CtxModifier::with_fit(rmod, item.get_fit_uid()?)),
        }
    }
}
