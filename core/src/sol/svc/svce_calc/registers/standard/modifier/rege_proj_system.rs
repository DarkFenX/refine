use crate::sol::{
    item::{SolItem, SolShipKind},
    svc::svce_calc::{
        registers::SolStandardRegister, SolAffecteeFilter, SolCtxModifier, SolDomain, SolLocationKind, SolRawModifier,
    },
};

use super::{add_ctx_modifier, remove_ctx_modifier};

impl SolStandardRegister {
    pub(super) fn proj_system_mod(
        &mut self,
        raw_modifier: SolRawModifier,
        projectee_item: &SolItem,
    ) -> Option<SolCtxModifier> {
        match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                        add_ctx_modifier(
                            &mut self.cmods_root,
                            (projectee_ship.get_fit_id(), SolLocationKind::Ship),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Structure => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Structure) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                        add_ctx_modifier(
                            &mut self.cmods_root,
                            (projectee_ship.get_fit_id(), SolLocationKind::Structure),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Char => match projectee_item {
                    SolItem::Ship(projectee_ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                        add_ctx_modifier(
                            &mut self.cmods_root,
                            (projectee_ship.get_fit_id(), SolLocationKind::Character),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                _ => None,
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                        add_ctx_modifier(
                            &mut self.cmods_loc,
                            (projectee_ship.get_fit_id(), SolLocationKind::Ship),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Structure => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Structure) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                        add_ctx_modifier(
                            &mut self.cmods_loc,
                            (projectee_ship.get_fit_id(), SolLocationKind::Structure),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Char => match projectee_item {
                    SolItem::Ship(projectee_ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                        add_ctx_modifier(
                            &mut self.cmods_loc,
                            (projectee_ship.get_fit_id(), SolLocationKind::Character),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                _ => None,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                        add_ctx_modifier(
                            &mut self.cmods_loc_grp,
                            (projectee_ship.get_fit_id(), SolLocationKind::Ship, grp_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Structure => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Structure) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                        add_ctx_modifier(
                            &mut self.cmods_loc_grp,
                            (projectee_ship.get_fit_id(), SolLocationKind::Structure, grp_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Char => match projectee_item {
                    SolItem::Ship(projectee_ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                        add_ctx_modifier(
                            &mut self.cmods_loc_grp,
                            (projectee_ship.get_fit_id(), SolLocationKind::Character, grp_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                _ => None,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                        add_ctx_modifier(
                            &mut self.cmods_loc_srq,
                            (projectee_ship.get_fit_id(), SolLocationKind::Ship, srq_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Structure => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Structure) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                        add_ctx_modifier(
                            &mut self.cmods_loc_srq,
                            (projectee_ship.get_fit_id(), SolLocationKind::Structure, srq_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Char => match projectee_item {
                    SolItem::Ship(projectee_ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                        add_ctx_modifier(
                            &mut self.cmods_loc_srq,
                            (projectee_ship.get_fit_id(), SolLocationKind::Character, srq_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                _ => None,
            },
            SolAffecteeFilter::OwnSrq(srq_id) => match projectee_item {
                SolItem::Ship(projectee_ship) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                    add_ctx_modifier(
                        &mut self.cmods_own_srq,
                        (projectee_ship.get_fit_id(), srq_id),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
        }
    }
    pub(super) fn unproj_system_mod(
        &mut self,
        raw_modifier: SolRawModifier,
        projectee_item: &SolItem,
    ) -> Option<SolCtxModifier> {
        match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                        remove_ctx_modifier(
                            &mut self.cmods_root,
                            &(projectee_ship.get_fit_id(), SolLocationKind::Ship),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Structure => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Structure) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                        remove_ctx_modifier(
                            &mut self.cmods_root,
                            &(projectee_ship.get_fit_id(), SolLocationKind::Structure),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Char => match projectee_item {
                    SolItem::Ship(projectee_ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                        remove_ctx_modifier(
                            &mut self.cmods_root,
                            &(projectee_ship.get_fit_id(), SolLocationKind::Character),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                _ => None,
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                        remove_ctx_modifier(
                            &mut self.cmods_loc,
                            &(projectee_ship.get_fit_id(), SolLocationKind::Ship),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Structure => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Structure) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                        remove_ctx_modifier(
                            &mut self.cmods_loc,
                            &(projectee_ship.get_fit_id(), SolLocationKind::Structure),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Char => match projectee_item {
                    SolItem::Ship(projectee_ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                        remove_ctx_modifier(
                            &mut self.cmods_loc,
                            &(projectee_ship.get_fit_id(), SolLocationKind::Character),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                _ => None,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                        remove_ctx_modifier(
                            &mut self.cmods_loc_grp,
                            &(projectee_ship.get_fit_id(), SolLocationKind::Ship, grp_id),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Structure => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Structure) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                        remove_ctx_modifier(
                            &mut self.cmods_loc_grp,
                            &(projectee_ship.get_fit_id(), SolLocationKind::Structure, grp_id),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Char => match projectee_item {
                    SolItem::Ship(projectee_ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                        remove_ctx_modifier(
                            &mut self.cmods_loc_grp,
                            &(projectee_ship.get_fit_id(), SolLocationKind::Character, grp_id),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                _ => None,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                        remove_ctx_modifier(
                            &mut self.cmods_loc_srq,
                            &(projectee_ship.get_fit_id(), SolLocationKind::Ship, srq_id),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Structure => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Structure) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                        remove_ctx_modifier(
                            &mut self.cmods_loc_srq,
                            &(projectee_ship.get_fit_id(), SolLocationKind::Structure, srq_id),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Char => match projectee_item {
                    SolItem::Ship(projectee_ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                        remove_ctx_modifier(
                            &mut self.cmods_loc_srq,
                            &(projectee_ship.get_fit_id(), SolLocationKind::Character, srq_id),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                _ => None,
            },
            SolAffecteeFilter::OwnSrq(srq_id) => match projectee_item {
                SolItem::Ship(projectee_ship) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.get_id());
                    remove_ctx_modifier(
                        &mut self.cmods_own_srq,
                        &(projectee_ship.get_fit_id(), srq_id),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
        }
    }
}
