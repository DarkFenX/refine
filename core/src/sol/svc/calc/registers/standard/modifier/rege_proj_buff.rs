use crate::sol::{
    ItemKey,
    svc::calc::{AffecteeFilter, CtxModifier, Location, LocationKind, RawModifier, registers::StandardRegister},
    uad::item::{Item, ShipKind},
};

use super::{add_ctx_modifier, remove_ctx_modifier};

impl StandardRegister {
    pub(super) fn proj_buff_mod(
        &mut self,
        raw_modifier: RawModifier,
        projectee_item_key: ItemKey,
        projectee_item: &Item,
    ) -> Option<CtxModifier> {
        self.process_buff_mod(raw_modifier, projectee_item_key, projectee_item, true)
    }
    pub(super) fn query_buff_mod(
        &mut self,
        raw_modifier: RawModifier,
        projectee_item_key: ItemKey,
        projectee_item: &Item,
    ) -> Option<CtxModifier> {
        self.process_buff_mod(raw_modifier, projectee_item_key, projectee_item, false)
    }
    fn process_buff_mod(
        &mut self,
        raw_modifier: RawModifier,
        projectee_item_key: ItemKey,
        projectee_item: &Item,
        register: bool,
    ) -> Option<CtxModifier> {
        match raw_modifier.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Everything => match projectee_item {
                    Item::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        ShipKind::Ship => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            if register {
                                add_ctx_modifier(
                                    &mut self.cmods_root,
                                    (projectee_ship.get_fit_key(), LocationKind::Ship),
                                    ctx_modifier,
                                    &mut self.cmods_by_attr_spec,
                                );
                            }
                            Some(ctx_modifier)
                        }
                        _ => None,
                    },
                    _ if projectee_item.is_buffable() => {
                        let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                        if register {
                            add_ctx_modifier(
                                &mut self.cmods_direct,
                                projectee_item_key,
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                        }
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                Location::Ship => match projectee_item {
                    Item::Ship(projectee_ship) if matches!(projectee_ship.get_kind(), ShipKind::Ship) => {
                        let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                        if register {
                            add_ctx_modifier(
                                &mut self.cmods_root,
                                (projectee_ship.get_fit_key(), LocationKind::Ship),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                        }
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::Loc(Location::Everything | Location::Ship) => match projectee_item {
                Item::Ship(projectee_ship) if matches!(projectee_ship.get_kind(), ShipKind::Ship) => {
                    let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                    if register {
                        add_ctx_modifier(
                            &mut self.cmods_loc,
                            (projectee_ship.get_fit_key(), LocationKind::Ship),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                    Some(ctx_modifier)
                }
                _ => None,
            },
            AffecteeFilter::LocGrp(Location::Everything | Location::Ship, a_item_grp_id) => match projectee_item {
                Item::Ship(projectee_ship) if matches!(projectee_ship.get_kind(), ShipKind::Ship) => {
                    let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                    if register {
                        add_ctx_modifier(
                            &mut self.cmods_loc_grp,
                            (projectee_ship.get_fit_key(), LocationKind::Ship, a_item_grp_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                    Some(ctx_modifier)
                }
                _ => None,
            },
            AffecteeFilter::LocSrq(Location::Everything | Location::Ship, srq_a_item_id) => match projectee_item {
                Item::Ship(projectee_ship) if matches!(projectee_ship.get_kind(), ShipKind::Ship) => {
                    let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                    if register {
                        add_ctx_modifier(
                            &mut self.cmods_loc_srq,
                            (projectee_ship.get_fit_key(), LocationKind::Ship, srq_a_item_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                    Some(ctx_modifier)
                }
                _ => None,
            },
            _ => None,
        }
    }
    pub(super) fn unproj_buff_mod(
        &mut self,
        raw_modifier: RawModifier,
        projectee_item_key: ItemKey,
        projectee_item: &Item,
    ) -> Option<CtxModifier> {
        match raw_modifier.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Everything => match projectee_item {
                    Item::Ship(projectee_ship) => match projectee_ship.get_kind() {
                        ShipKind::Ship => {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                            remove_ctx_modifier(
                                &mut self.cmods_root,
                                &(projectee_ship.get_fit_key(), LocationKind::Ship),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            Some(ctx_modifier)
                        }
                        _ => None,
                    },
                    _ if projectee_item.is_buffable() => {
                        let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                        remove_ctx_modifier(
                            &mut self.cmods_direct,
                            &projectee_item_key,
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                Location::Ship => match projectee_item {
                    Item::Ship(projectee_ship) if matches!(projectee_ship.get_kind(), ShipKind::Ship) => {
                        let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                        remove_ctx_modifier(
                            &mut self.cmods_root,
                            &(projectee_ship.get_fit_key(), LocationKind::Ship),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        Some(ctx_modifier)
                    }
                    _ => None,
                },
                _ => None,
            },
            AffecteeFilter::Loc(Location::Everything | Location::Ship) => match projectee_item {
                Item::Ship(projectee_ship) if matches!(projectee_ship.get_kind(), ShipKind::Ship) => {
                    let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                    remove_ctx_modifier(
                        &mut self.cmods_loc,
                        &(projectee_ship.get_fit_key(), LocationKind::Ship),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            AffecteeFilter::LocGrp(Location::Everything | Location::Ship, a_item_grp_id) => match projectee_item {
                Item::Ship(projectee_ship) if matches!(projectee_ship.get_kind(), ShipKind::Ship) => {
                    let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                    remove_ctx_modifier(
                        &mut self.cmods_loc_grp,
                        &(projectee_ship.get_fit_key(), LocationKind::Ship, a_item_grp_id),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            AffecteeFilter::LocSrq(Location::Everything | Location::Ship, srq_a_item_id) => match projectee_item {
                Item::Ship(projectee_ship) if matches!(projectee_ship.get_kind(), ShipKind::Ship) => {
                    let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, projectee_item_key);
                    remove_ctx_modifier(
                        &mut self.cmods_loc_srq,
                        &(projectee_ship.get_fit_key(), LocationKind::Ship, srq_a_item_id),
                        &ctx_modifier,
                        &mut self.cmods_by_attr_spec,
                    );
                    Some(ctx_modifier)
                }
                _ => None,
            },
            _ => None,
        }
    }
}
