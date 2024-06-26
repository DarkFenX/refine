use crate::sol::{
    item::{SolItem, SolShipKind},
    svc::svce_calc::{
        registers::SolStandardRegister, SolAffecteeFilter, SolCtxModifier, SolDomain, SolLocationKind, SolRawModifier,
    },
};

impl SolStandardRegister {
    pub(super) fn proj_buff_mod(
        &mut self,
        raw_modifier: SolRawModifier,
        projectee_item: &SolItem,
    ) -> Option<SolCtxModifier> {
        match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolDomain::Everything => match projectee_item {
                    SolItem::Ship(projectee_ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        match projectee_ship.kind {
                            SolShipKind::Ship => self
                                .cmods_root
                                .add_entry((projectee_ship.fit_id, SolLocationKind::Ship), ctx_modifier),
                            SolShipKind::Structure => self
                                .cmods_root
                                .add_entry((projectee_ship.fit_id, SolLocationKind::Structure), ctx_modifier),
                            _ => self.cmods_direct.add_entry(projectee_ship.id, ctx_modifier),
                        }
                        Some(ctx_modifier)
                    }
                    _ if projectee_item.is_buff_modifiable() => {
                        let item_id = projectee_item.get_id();
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, item_id);
                        self.cmods_direct.add_entry(item_id, ctx_modifier);
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_root
                            .add_entry((projectee_ship.fit_id, SolLocationKind::Ship), ctx_modifier);
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                _ => None,
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolDomain::Everything => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.kind {
                        SolShipKind::Ship => {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                            self.cmods_loc
                                .add_entry((projectee_ship.fit_id, SolLocationKind::Ship), ctx_modifier);
                            Some(ctx_modifier)
                        }
                        SolShipKind::Structure => {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                            self.cmods_loc
                                .add_entry((projectee_ship.fit_id, SolLocationKind::Structure), ctx_modifier);
                            Some(ctx_modifier)
                        }
                        _ => None,
                    },
                    _ => None,
                },
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_loc
                            .add_entry((projectee_ship.fit_id, SolLocationKind::Ship), ctx_modifier);
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                _ => None,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolDomain::Everything => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.kind {
                        SolShipKind::Ship => {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                            self.cmods_loc_grp
                                .add_entry((projectee_ship.fit_id, SolLocationKind::Ship, grp_id), ctx_modifier);
                            Some(ctx_modifier)
                        }
                        SolShipKind::Structure => {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                            self.cmods_loc_grp.add_entry(
                                (projectee_ship.fit_id, SolLocationKind::Structure, grp_id),
                                ctx_modifier,
                            );
                            Some(ctx_modifier)
                        }
                        _ => None,
                    },
                    _ => None,
                },
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_loc_grp
                            .add_entry((projectee_ship.fit_id, SolLocationKind::Ship, grp_id), ctx_modifier);
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                _ => None,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolDomain::Everything => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.kind {
                        SolShipKind::Ship => {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                            self.cmods_loc_srq
                                .add_entry((projectee_ship.fit_id, SolLocationKind::Ship, srq_id), ctx_modifier);
                            Some(ctx_modifier)
                        }
                        SolShipKind::Structure => {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                            self.cmods_loc_srq.add_entry(
                                (projectee_ship.fit_id, SolLocationKind::Structure, srq_id),
                                ctx_modifier,
                            );
                            Some(ctx_modifier)
                        }
                        _ => None,
                    },
                    _ => None,
                },
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_loc_srq
                            .add_entry((projectee_ship.fit_id, SolLocationKind::Ship, srq_id), ctx_modifier);
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        }
    }
    pub(super) fn unproj_buff_mod(
        &mut self,
        raw_modifier: SolRawModifier,
        projectee_item: &SolItem,
    ) -> Option<SolCtxModifier> {
        match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolDomain::Everything => match projectee_item {
                    SolItem::Ship(projectee_ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        match projectee_ship.kind {
                            SolShipKind::Ship => self
                                .cmods_root
                                .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Ship), &ctx_modifier),
                            SolShipKind::Structure => self
                                .cmods_root
                                .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Structure), &ctx_modifier),
                            _ => self.cmods_direct.remove_entry(&projectee_ship.id, &ctx_modifier),
                        }
                        Some(ctx_modifier)
                    }
                    _ if projectee_item.is_buff_modifiable() => {
                        let item_id = projectee_item.get_id();
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, item_id);
                        self.cmods_direct.remove_entry(&item_id, &ctx_modifier);
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_root
                            .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Ship), &ctx_modifier);
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                _ => None,
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolDomain::Everything => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.kind {
                        SolShipKind::Ship => {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                            self.cmods_loc
                                .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Ship), &ctx_modifier);
                            Some(ctx_modifier)
                        }
                        SolShipKind::Structure => {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                            self.cmods_loc
                                .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Structure), &ctx_modifier);
                            Some(ctx_modifier)
                        }
                        _ => None,
                    },
                    _ => None,
                },
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_loc
                            .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Ship), &ctx_modifier);
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                _ => None,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolDomain::Everything => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.kind {
                        SolShipKind::Ship => {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                            self.cmods_loc_grp
                                .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Ship, grp_id), &ctx_modifier);
                            Some(ctx_modifier)
                        }
                        SolShipKind::Structure => {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                            self.cmods_loc_grp.remove_entry(
                                &(projectee_ship.fit_id, SolLocationKind::Structure, grp_id),
                                &ctx_modifier,
                            );
                            Some(ctx_modifier)
                        }
                        _ => None,
                    },
                    _ => None,
                },
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_loc_grp
                            .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Ship, grp_id), &ctx_modifier);
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                _ => None,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolDomain::Everything => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.kind {
                        SolShipKind::Ship => {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                            self.cmods_loc_srq
                                .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Ship, srq_id), &ctx_modifier);
                            Some(ctx_modifier)
                        }
                        SolShipKind::Structure => {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                            self.cmods_loc_srq.remove_entry(
                                &(projectee_ship.fit_id, SolLocationKind::Structure, srq_id),
                                &ctx_modifier,
                            );
                            Some(ctx_modifier)
                        }
                        _ => None,
                    },
                    _ => None,
                },
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, projectee_ship.id);
                        self.cmods_loc_srq
                            .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Ship, srq_id), &ctx_modifier);
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        }
    }
}
