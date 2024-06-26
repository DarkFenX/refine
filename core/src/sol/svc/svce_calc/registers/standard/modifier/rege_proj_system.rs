use crate::sol::{
    item::{SolItem, SolShipKind},
    svc::svce_calc::{
        registers::SolStandardRegister, SolAffecteeFilter, SolCtxModifier, SolDomain, SolLocationKind, SolRawModifier,
    },
};

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
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_root
                            .add_entry((projectee_ship.fit_id, SolLocationKind::Ship), ctx_modifier);
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Structure => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Structure) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_root
                            .add_entry((projectee_ship.fit_id, SolLocationKind::Structure), ctx_modifier);
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Char => match projectee_item {
                    SolItem::Ship(projectee_ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_root
                            .add_entry((projectee_ship.fit_id, SolLocationKind::Character), ctx_modifier);
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                _ => None,
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_loc
                            .add_entry((projectee_ship.fit_id, SolLocationKind::Ship), ctx_modifier);
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Structure => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Structure) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_loc
                            .add_entry((projectee_ship.fit_id, SolLocationKind::Structure), ctx_modifier);
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Char => match projectee_item {
                    SolItem::Ship(projectee_ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_loc
                            .add_entry((projectee_ship.fit_id, SolLocationKind::Character), ctx_modifier);
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                _ => None,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_loc_grp
                            .add_entry((projectee_ship.fit_id, SolLocationKind::Ship, grp_id), ctx_modifier);
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Structure => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Structure) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_loc_grp.add_entry(
                            (projectee_ship.fit_id, SolLocationKind::Structure, grp_id),
                            ctx_modifier,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Char => match projectee_item {
                    SolItem::Ship(projectee_ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_loc_grp.add_entry(
                            (projectee_ship.fit_id, SolLocationKind::Character, grp_id),
                            ctx_modifier,
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
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_loc_srq
                            .add_entry((projectee_ship.fit_id, SolLocationKind::Ship, srq_id), ctx_modifier);
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Structure => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Structure) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_loc_srq.add_entry(
                            (projectee_ship.fit_id, SolLocationKind::Structure, srq_id),
                            ctx_modifier,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Char => match projectee_item {
                    SolItem::Ship(projectee_ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_loc_srq.add_entry(
                            (projectee_ship.fit_id, SolLocationKind::Character, srq_id),
                            ctx_modifier,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                _ => None,
            },
            SolAffecteeFilter::OwnSrq(srq_id) => match projectee_item {
                SolItem::Ship(projectee_ship) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                    self.cmods_own_srq
                        .add_entry((projectee_ship.fit_id, srq_id), ctx_modifier);
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
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_root
                            .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Ship), &ctx_modifier);
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Structure => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Structure) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_root
                            .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Structure), &ctx_modifier);
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Char => match projectee_item {
                    SolItem::Ship(projectee_ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_root
                            .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Character), &ctx_modifier);
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                _ => None,
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_loc
                            .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Ship), &ctx_modifier);
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Structure => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Structure) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_loc
                            .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Structure), &ctx_modifier);
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Char => match projectee_item {
                    SolItem::Ship(projectee_ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_loc
                            .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Character), &ctx_modifier);
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                _ => None,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_loc_grp
                            .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Ship, grp_id), &ctx_modifier);
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Structure => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Structure) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_loc_grp.remove_entry(
                            &(projectee_ship.fit_id, SolLocationKind::Structure, grp_id),
                            &ctx_modifier,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Char => match projectee_item {
                    SolItem::Ship(projectee_ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_loc_grp.remove_entry(
                            &(projectee_ship.fit_id, SolLocationKind::Character, grp_id),
                            &ctx_modifier,
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
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_loc_srq
                            .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Ship, srq_id), &ctx_modifier);
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Structure => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Structure) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_loc_srq.remove_entry(
                            &(projectee_ship.fit_id, SolLocationKind::Structure, srq_id),
                            &ctx_modifier,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Char => match projectee_item {
                    SolItem::Ship(projectee_ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_loc_srq.remove_entry(
                            &(projectee_ship.fit_id, SolLocationKind::Character, srq_id),
                            &ctx_modifier,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                _ => None,
            },
            SolAffecteeFilter::OwnSrq(srq_id) => match projectee_item {
                SolItem::Ship(projectee_ship) => {
                    let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                    self.cmods_own_srq
                        .remove_entry(&(projectee_ship.fit_id, srq_id), &ctx_modifier);
                    Some(ctx_modifier)
                }
                _ => None,
            },
        }
    }
}
