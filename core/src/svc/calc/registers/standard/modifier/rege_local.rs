use super::{add_ctx_modifier, remove_ctx_modifier};
use crate::{
    svc::calc::{AffecteeFilter, CtxModifier, Location, RawModifier, registers::StandardRegister},
    uad::UadItem,
};

impl StandardRegister {
    pub(in crate::svc::calc) fn reg_local_mod(
        &mut self,
        item: &UadItem,
        raw_modifier: RawModifier,
    ) -> Option<CtxModifier> {
        let ctx_modifier = match raw_modifier.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Item => {
                    let ctx_modifier = CtxModifier::from_raw(raw_modifier);
                    add_ctx_modifier(
                        &mut self.cmods_direct,
                        ctx_modifier.raw.affector_espec.item_key,
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                Location::Other => {
                    let ctx_modifier = CtxModifier::from_raw(raw_modifier);
                    add_ctx_modifier(
                        &mut self.cmods_other,
                        ctx_modifier.raw.affector_espec.item_key,
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => match loc.try_into() {
                    Ok(loc_kind) => match item.get_fit_key() {
                        Some(fit_key) => {
                            let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fit_key);
                            add_ctx_modifier(
                                &mut self.cmods_root,
                                (fit_key, loc_kind),
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
            AffecteeFilter::Loc(loc) => match loc.try_into() {
                Ok(loc_kind) => match item.get_fit_key() {
                    Some(fit_key) => {
                        let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fit_key);
                        add_ctx_modifier(
                            &mut self.cmods_loc,
                            (fit_key, loc_kind),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    None => None,
                },
                _ => None,
            },
            AffecteeFilter::LocGrp(loc, a_item_grp_id) => match loc.try_into() {
                Ok(loc_kind) => match item.get_fit_key() {
                    Some(fit_key) => {
                        let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fit_key);
                        add_ctx_modifier(
                            &mut self.cmods_loc_grp,
                            (fit_key, loc_kind, a_item_grp_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    None => None,
                },
                _ => None,
            },
            AffecteeFilter::LocSrq(loc, srq_a_item_id) => match loc.try_into() {
                Ok(loc_kind) => match item.get_fit_key() {
                    Some(fit_key) => {
                        let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fit_key);
                        add_ctx_modifier(
                            &mut self.cmods_loc_srq,
                            (fit_key, loc_kind, srq_a_item_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    None => None,
                },
                _ => None,
            },
            AffecteeFilter::OwnSrq(srq_a_item_id) => match item.get_fit_key() {
                Some(fit_key) => {
                    let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fit_key);
                    add_ctx_modifier(
                        &mut self.cmods_own_srq,
                        (fit_key, srq_a_item_id),
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
            self.rmods_all.add_entry(raw_modifier.affector_espec, raw_modifier);
        }
        ctx_modifier
    }
    pub(in crate::svc::calc) fn unreg_local_mod(
        &mut self,
        item: &UadItem,
        raw_modifier: RawModifier,
    ) -> Option<CtxModifier> {
        match raw_modifier.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Item => {
                    let ctx_modifier = CtxModifier::from_raw(raw_modifier);
                    remove_ctx_modifier(
                        &mut self.cmods_direct,
                        &ctx_modifier.raw.affector_espec.item_key,
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                Location::Other => {
                    let ctx_modifier = CtxModifier::from_raw(raw_modifier);
                    remove_ctx_modifier(
                        &mut self.cmods_other,
                        &ctx_modifier.raw.affector_espec.item_key,
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => match loc.try_into() {
                    Ok(loc_kind) => match item.get_fit_key() {
                        Some(fit_key) => {
                            let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fit_key);
                            remove_ctx_modifier(
                                &mut self.cmods_root,
                                &(fit_key, loc_kind),
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
            AffecteeFilter::Loc(loc) => match loc.try_into() {
                Ok(loc_kind) => match item.get_fit_key() {
                    Some(fit_key) => {
                        let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fit_key);
                        remove_ctx_modifier(
                            &mut self.cmods_loc,
                            &(fit_key, loc_kind),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    None => None,
                },
                _ => None,
            },
            AffecteeFilter::LocGrp(loc, a_item_grp_id) => match loc.try_into() {
                Ok(loc_kind) => match item.get_fit_key() {
                    Some(fit_key) => {
                        let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fit_key);
                        remove_ctx_modifier(
                            &mut self.cmods_loc_grp,
                            &(fit_key, loc_kind, a_item_grp_id),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    None => None,
                },
                _ => None,
            },
            AffecteeFilter::LocSrq(loc, srq_a_item_id) => match loc.try_into() {
                Ok(loc_kind) => match item.get_fit_key() {
                    Some(fit_key) => {
                        let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fit_key);
                        remove_ctx_modifier(
                            &mut self.cmods_loc_srq,
                            &(fit_key, loc_kind, srq_a_item_id),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    None => None,
                },
                _ => None,
            },
            AffecteeFilter::OwnSrq(srq_a_item_id) => match item.get_fit_key() {
                Some(fit_key) => {
                    let ctx_modifier = CtxModifier::from_raw_with_fit(raw_modifier, fit_key);
                    remove_ctx_modifier(
                        &mut self.cmods_own_srq,
                        &(fit_key, srq_a_item_id),
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
