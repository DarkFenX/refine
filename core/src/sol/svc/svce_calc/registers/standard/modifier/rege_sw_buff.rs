use crate::sol::{
    item::{SolItem, SolShipKind},
    svc::svce_calc::{
        registers::SolStandardRegister, SolAffecteeFilter, SolCtxModifier, SolDomain, SolLocationKind, SolRawModifier,
    },
    SolView,
};

use super::{add_ctx_modifier, remove_ctx_modifier};

impl SolStandardRegister {
    pub(in crate::sol::svc::svce_calc) fn reg_sw_buff_mod(
        &mut self,
        ctx_modifiers: &mut Vec<SolCtxModifier>,
        sol_view: &SolView,
        raw_modifier: SolRawModifier,
    ) -> bool {
        ctx_modifiers.clear();
        let valid = match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolDomain::Everything => {
                    for projectee_item_ids in self.affectee_buffable.values() {
                        ctx_modifiers.reserve(projectee_item_ids.len());
                        for projectee_item_id in projectee_item_ids {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, *projectee_item_id);
                            add_ctx_modifier(
                                &mut self.cmods_direct,
                                *projectee_item_id,
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            ctx_modifiers.push(ctx_modifier);
                        }
                    }
                    true
                }
                SolDomain::Ship => {
                    // Assume all fits are of ship type
                    ctx_modifiers.reserve_exact(sol_view.fits.len());
                    for fit in sol_view.fits.iter_fits() {
                        if matches!(fit.kind, SolShipKind::Ship) {
                            if let Some(ship_id) = fit.ship {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, ship_id);
                                add_ctx_modifier(
                                    &mut self.cmods_root,
                                    (fit.id, SolLocationKind::Ship),
                                    ctx_modifier,
                                    &mut self.cmods_by_attr_spec,
                                );
                                ctx_modifiers.push(ctx_modifier);
                            }
                        }
                    }
                    true
                }
                _ => false,
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolDomain::Everything | SolDomain::Ship => {
                    // Assume all fits are of ship type
                    ctx_modifiers.reserve_exact(sol_view.fits.len());
                    for fit in sol_view.fits.iter_fits() {
                        if matches!(fit.kind, SolShipKind::Ship) {
                            if let Some(ship_id) = fit.ship {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, ship_id);
                                add_ctx_modifier(
                                    &mut self.cmods_loc,
                                    (fit.id, SolLocationKind::Ship),
                                    ctx_modifier,
                                    &mut self.cmods_by_attr_spec,
                                );
                                ctx_modifiers.push(ctx_modifier);
                            }
                        }
                    }
                    true
                }
                _ => false,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolDomain::Everything | SolDomain::Ship => {
                    // Assume all fits are of ship type
                    ctx_modifiers.reserve_exact(sol_view.fits.len());
                    for fit in sol_view.fits.iter_fits() {
                        if matches!(fit.kind, SolShipKind::Ship) {
                            if let Some(ship_id) = fit.ship {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, ship_id);
                                add_ctx_modifier(
                                    &mut self.cmods_loc_grp,
                                    (fit.id, SolLocationKind::Ship, grp_id),
                                    ctx_modifier,
                                    &mut self.cmods_by_attr_spec,
                                );
                                ctx_modifiers.push(ctx_modifier);
                            }
                        }
                    }
                    true
                }
                _ => false,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolDomain::Everything | SolDomain::Ship => {
                    // Assume all fits are of ship type
                    ctx_modifiers.reserve_exact(sol_view.fits.len());
                    for fit in sol_view.fits.iter_fits() {
                        if matches!(fit.kind, SolShipKind::Ship) {
                            if let Some(ship_id) = fit.ship {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, ship_id);
                                add_ctx_modifier(
                                    &mut self.cmods_loc_srq,
                                    (fit.id, SolLocationKind::Ship, srq_id),
                                    ctx_modifier,
                                    &mut self.cmods_by_attr_spec,
                                );
                                ctx_modifiers.push(ctx_modifier);
                            }
                        }
                    }
                    true
                }
                _ => false,
            },
            _ => false,
        };
        if valid {
            self.rmods_nonproj
                .add_entry((raw_modifier.affector_item_id, raw_modifier.effect_id), raw_modifier);
            self.rmods_sw_buff.insert(raw_modifier);
        }
        valid
    }
    pub(in crate::sol::svc::svce_calc) fn unreg_sw_buff_mod(
        &mut self,
        ctx_modifiers: &mut Vec<SolCtxModifier>,
        sol_view: &SolView,
        raw_modifier: &SolRawModifier,
    ) {
        ctx_modifiers.clear();
        match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolDomain::Everything => {
                    for projectee_item_ids in self.affectee_buffable.values() {
                        ctx_modifiers.reserve(projectee_item_ids.len());
                        for projectee_item_id in projectee_item_ids {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, *projectee_item_id);
                            remove_ctx_modifier(
                                &mut self.cmods_direct,
                                projectee_item_id,
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            ctx_modifiers.push(ctx_modifier);
                        }
                    }
                }
                SolDomain::Ship => {
                    // Assume all fits are of ship type
                    ctx_modifiers.reserve_exact(sol_view.fits.len());
                    for fit in sol_view.fits.iter_fits() {
                        if matches!(fit.kind, SolShipKind::Ship) {
                            if let Some(ship_id) = fit.ship {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, ship_id);
                                remove_ctx_modifier(
                                    &mut self.cmods_root,
                                    &(fit.id, SolLocationKind::Ship),
                                    &ctx_modifier,
                                    &mut self.cmods_by_attr_spec,
                                );
                                ctx_modifiers.push(ctx_modifier);
                            }
                        }
                    }
                }
                _ => (),
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolDomain::Everything | SolDomain::Ship => {
                    // Assume all fits are of ship type
                    ctx_modifiers.reserve_exact(sol_view.fits.len());
                    for fit in sol_view.fits.iter_fits() {
                        if matches!(fit.kind, SolShipKind::Ship) {
                            if let Some(ship_id) = fit.ship {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, ship_id);
                                remove_ctx_modifier(
                                    &mut self.cmods_loc,
                                    &(fit.id, SolLocationKind::Ship),
                                    &ctx_modifier,
                                    &mut self.cmods_by_attr_spec,
                                );
                                ctx_modifiers.push(ctx_modifier);
                            }
                        }
                    }
                }
                _ => (),
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolDomain::Everything | SolDomain::Ship => {
                    // Assume all fits are of ship type
                    ctx_modifiers.reserve_exact(sol_view.fits.len());
                    for fit in sol_view.fits.iter_fits() {
                        if matches!(fit.kind, SolShipKind::Ship) {
                            if let Some(ship_id) = fit.ship {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, ship_id);
                                remove_ctx_modifier(
                                    &mut self.cmods_loc_grp,
                                    &(fit.id, SolLocationKind::Ship, grp_id),
                                    &ctx_modifier,
                                    &mut self.cmods_by_attr_spec,
                                );
                                ctx_modifiers.push(ctx_modifier);
                            }
                        }
                    }
                }
                _ => (),
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolDomain::Everything | SolDomain::Ship => {
                    // Assume all fits are of ship type
                    ctx_modifiers.reserve_exact(sol_view.fits.len());
                    for fit in sol_view.fits.iter_fits() {
                        if matches!(fit.kind, SolShipKind::Ship) {
                            if let Some(ship_id) = fit.ship {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, ship_id);
                                remove_ctx_modifier(
                                    &mut self.cmods_loc_srq,
                                    &(fit.id, SolLocationKind::Ship, srq_id),
                                    &ctx_modifier,
                                    &mut self.cmods_by_attr_spec,
                                );
                                ctx_modifiers.push(ctx_modifier);
                            }
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        }
        self.rmods_sw_buff.remove(raw_modifier);
    }
    // Is supposed to be called only for buffable items
    pub(in crate::sol::svc::svce_calc::registers::standard) fn reg_buffable_for_sw(&mut self, item: &SolItem) {
        for raw_modifier in self.rmods_sw_buff.iter() {
            match raw_modifier.affectee_filter {
                SolAffecteeFilter::Direct(dom) => match dom {
                    SolDomain::Everything => {
                        let item_id = item.get_id();
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, item_id);
                        add_ctx_modifier(
                            &mut self.cmods_direct,
                            item_id,
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                    SolDomain::Ship => {
                        if let SolItem::Ship(ship) = item {
                            if matches!(ship.get_kind(), SolShipKind::Ship) {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, ship.get_id());
                                add_ctx_modifier(
                                    &mut self.cmods_root,
                                    (ship.get_fit_id(), SolLocationKind::Ship),
                                    ctx_modifier,
                                    &mut self.cmods_by_attr_spec,
                                );
                            }
                        }
                    }
                    _ => (),
                },
                SolAffecteeFilter::Loc(dom) => match dom {
                    SolDomain::Everything | SolDomain::Ship => {
                        if let SolItem::Ship(ship) = item {
                            if matches!(ship.get_kind(), SolShipKind::Ship) {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, ship.get_id());
                                add_ctx_modifier(
                                    &mut self.cmods_loc,
                                    (ship.get_fit_id(), SolLocationKind::Ship),
                                    ctx_modifier,
                                    &mut self.cmods_by_attr_spec,
                                );
                            }
                        }
                    }
                    _ => (),
                },
                SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                    SolDomain::Everything | SolDomain::Ship => {
                        if let SolItem::Ship(ship) = item {
                            if matches!(ship.get_kind(), SolShipKind::Ship) {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, ship.get_id());
                                add_ctx_modifier(
                                    &mut self.cmods_loc_grp,
                                    (ship.get_fit_id(), SolLocationKind::Ship, grp_id),
                                    ctx_modifier,
                                    &mut self.cmods_by_attr_spec,
                                );
                            }
                        }
                    }
                    _ => (),
                },
                SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                    SolDomain::Everything | SolDomain::Ship => {
                        if let SolItem::Ship(ship) = item {
                            if matches!(ship.get_kind(), SolShipKind::Ship) {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, ship.get_id());
                                add_ctx_modifier(
                                    &mut self.cmods_loc_srq,
                                    (ship.get_fit_id(), SolLocationKind::Ship, srq_id),
                                    ctx_modifier,
                                    &mut self.cmods_by_attr_spec,
                                );
                            }
                        }
                    }
                    _ => (),
                },
                _ => (),
            };
        }
    }
    // Is supposed to be called only for buffable items
    pub(in crate::sol::svc::svce_calc::registers::standard) fn unreg_buffable_for_sw(&mut self, item: &SolItem) {
        for raw_modifier in self.rmods_sw_buff.iter() {
            match raw_modifier.affectee_filter {
                SolAffecteeFilter::Direct(dom) => match dom {
                    SolDomain::Everything => {
                        let item_id = item.get_id();
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, item_id);
                        remove_ctx_modifier(
                            &mut self.cmods_direct,
                            &item_id,
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                    SolDomain::Ship => {
                        if let SolItem::Ship(ship) = item {
                            if matches!(ship.get_kind(), SolShipKind::Ship) {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, ship.get_id());
                                remove_ctx_modifier(
                                    &mut self.cmods_root,
                                    &(ship.get_fit_id(), SolLocationKind::Ship),
                                    &ctx_modifier,
                                    &mut self.cmods_by_attr_spec,
                                );
                            }
                        }
                    }
                    _ => (),
                },
                SolAffecteeFilter::Loc(dom) => match dom {
                    SolDomain::Everything | SolDomain::Ship => {
                        if let SolItem::Ship(ship) = item {
                            if matches!(ship.get_kind(), SolShipKind::Ship) {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, ship.get_id());
                                remove_ctx_modifier(
                                    &mut self.cmods_loc,
                                    &(ship.get_fit_id(), SolLocationKind::Ship),
                                    &ctx_modifier,
                                    &mut self.cmods_by_attr_spec,
                                );
                            }
                        }
                    }
                    _ => (),
                },
                SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                    SolDomain::Everything | SolDomain::Ship => {
                        if let SolItem::Ship(ship) = item {
                            if matches!(ship.get_kind(), SolShipKind::Ship) {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, ship.get_id());
                                remove_ctx_modifier(
                                    &mut self.cmods_loc_grp,
                                    &(ship.get_fit_id(), SolLocationKind::Ship, grp_id),
                                    &ctx_modifier,
                                    &mut self.cmods_by_attr_spec,
                                );
                            }
                        }
                    }
                    _ => (),
                },
                SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                    SolDomain::Everything | SolDomain::Ship => {
                        if let SolItem::Ship(ship) = item {
                            if matches!(ship.get_kind(), SolShipKind::Ship) {
                                let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, ship.get_id());
                                remove_ctx_modifier(
                                    &mut self.cmods_loc_srq,
                                    &(ship.get_fit_id(), SolLocationKind::Ship, srq_id),
                                    &ctx_modifier,
                                    &mut self.cmods_by_attr_spec,
                                );
                            }
                        }
                    }
                    _ => (),
                },
                _ => (),
            };
        }
    }
}
