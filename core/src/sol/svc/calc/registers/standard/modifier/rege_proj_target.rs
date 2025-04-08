use crate::sol::{
    ItemKey,
    svc::calc::{AffecteeFilter, CtxModifier, Location, LocationKind, RawModifier, registers::StandardRegister},
    uad::item::{Item, ShipKind},
};

use super::{add_ctx_modifier, remove_ctx_modifier};

impl StandardRegister {
    pub(super) fn proj_target_mod(
        &mut self,
        raw_modifier: RawModifier,
        projectee_item_key: ItemKey,
        projectee_item: &Item,
    ) -> Option<CtxModifier> {
        match raw_modifier.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Target => match projectee_item {
                    Item::Ship(projectee_ship) => {
                        let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                        match projectee_ship.get_kind() {
                            ShipKind::Ship => add_ctx_modifier(
                                &mut self.cmods_root,
                                (projectee_ship.get_fit_id(), LocationKind::Ship),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            ),
                            ShipKind::Structure => add_ctx_modifier(
                                &mut self.cmods_root,
                                (projectee_ship.get_fit_id(), LocationKind::Structure),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            ),
                            _ => add_ctx_modifier(
                                &mut self.cmods_direct,
                                projectee_item_key,
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            ),
                        }
                        Some(ctx_modifier)
                    }
                    _ => {
                        let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                        add_ctx_modifier(
                            &mut self.cmods_direct,
                            projectee_item_key,
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                },
                _ => None,
            },
            AffecteeFilter::Loc(loc) => match loc {
                Location::Target => match projectee_item {
                    Item::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        ShipKind::Ship => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            add_ctx_modifier(
                                &mut self.cmods_loc,
                                (projectee_ship.get_fit_id(), LocationKind::Ship),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        ShipKind::Structure => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            add_ctx_modifier(
                                &mut self.cmods_loc,
                                (projectee_ship.get_fit_id(), LocationKind::Structure),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::LocGrp(loc, a_item_grp_id) => match loc {
                Location::Target => match projectee_item {
                    Item::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        ShipKind::Ship => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            add_ctx_modifier(
                                &mut self.cmods_loc_grp,
                                (projectee_ship.get_fit_id(), LocationKind::Ship, a_item_grp_id),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        ShipKind::Structure => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            add_ctx_modifier(
                                &mut self.cmods_loc_grp,
                                (projectee_ship.get_fit_id(), LocationKind::Structure, a_item_grp_id),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::LocSrq(loc, srq_a_item_id) => match loc {
                Location::Target => match projectee_item {
                    Item::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        ShipKind::Ship => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            add_ctx_modifier(
                                &mut self.cmods_loc_srq,
                                (projectee_ship.get_fit_id(), LocationKind::Ship, srq_a_item_id),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        ShipKind::Structure => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            add_ctx_modifier(
                                &mut self.cmods_loc_srq,
                                (projectee_ship.get_fit_id(), LocationKind::Structure, srq_a_item_id),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::OwnSrq(srq_a_item_id) => match projectee_item {
                Item::Ship(projectee_ship) => {
                    let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                    add_ctx_modifier(
                        &mut self.cmods_own_srq,
                        (projectee_ship.get_fit_id(), srq_a_item_id),
                        ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
        }
    }
    pub(super) fn unproj_target_mod(
        &mut self,
        raw_modifier: RawModifier,
        projectee_item_key: ItemKey,
        projectee_item: &Item,
    ) -> Option<CtxModifier> {
        match raw_modifier.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Target => match projectee_item {
                    Item::Ship(projectee_ship) => {
                        let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                        match projectee_ship.get_kind() {
                            ShipKind::Ship => remove_ctx_modifier(
                                &mut self.cmods_root,
                                &(projectee_ship.get_fit_id(), LocationKind::Ship),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            ),
                            ShipKind::Structure => remove_ctx_modifier(
                                &mut self.cmods_root,
                                &(projectee_ship.get_fit_id(), LocationKind::Structure),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            ),
                            _ => remove_ctx_modifier(
                                &mut self.cmods_direct,
                                &projectee_item_key,
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            ),
                        };
                        Some(ctx_modifier)
                    }
                    _ => {
                        let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                        remove_ctx_modifier(
                            &mut self.cmods_direct,
                            &projectee_item_key,
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                },
                _ => None,
            },
            AffecteeFilter::Loc(loc) => match loc {
                Location::Target => match projectee_item {
                    Item::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        ShipKind::Ship => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            remove_ctx_modifier(
                                &mut self.cmods_loc,
                                &(projectee_ship.get_fit_id(), LocationKind::Ship),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        ShipKind::Structure => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            remove_ctx_modifier(
                                &mut self.cmods_loc,
                                &(projectee_ship.get_fit_id(), LocationKind::Structure),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::LocGrp(loc, a_item_grp_id) => match loc {
                Location::Target => match projectee_item {
                    Item::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        ShipKind::Ship => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            remove_ctx_modifier(
                                &mut self.cmods_loc_grp,
                                &(projectee_ship.get_fit_id(), LocationKind::Ship, a_item_grp_id),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        ShipKind::Structure => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            remove_ctx_modifier(
                                &mut self.cmods_loc_grp,
                                &(projectee_ship.get_fit_id(), LocationKind::Structure, a_item_grp_id),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::LocSrq(loc, srq_a_item_id) => match loc {
                Location::Target => match projectee_item {
                    Item::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        ShipKind::Ship => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            remove_ctx_modifier(
                                &mut self.cmods_loc_srq,
                                &(projectee_ship.get_fit_id(), LocationKind::Ship, srq_a_item_id),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        ShipKind::Structure => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            remove_ctx_modifier(
                                &mut self.cmods_loc_srq,
                                &(projectee_ship.get_fit_id(), LocationKind::Structure, srq_a_item_id),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::OwnSrq(srq_a_item_id) => match projectee_item {
                Item::Ship(projectee_ship) => {
                    let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                    remove_ctx_modifier(
                        &mut self.cmods_own_srq,
                        &(projectee_ship.get_fit_id(), srq_a_item_id),
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
