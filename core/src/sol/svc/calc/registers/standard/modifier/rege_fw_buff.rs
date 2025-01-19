use crate::sol::{
    svc::calc::{
        modifier::SolCtxModifier, registers::SolStandardRegister, SolAffecteeFilter, SolLocation, SolLocationKind,
        SolRawModifier,
    },
    uad::{
        item::{SolFwEffect, SolItem, SolShipKind},
        SolUad,
    },
};

use super::{add_ctx_modifier, remove_ctx_modifier};

impl SolStandardRegister {
    pub(in crate::sol::svc::calc) fn reg_fw_buff_mod(
        &mut self,
        ctx_modifiers: &mut Vec<SolCtxModifier>,
        uad: &SolUad,
        fw_effect: &SolFwEffect,
        raw_modifier: SolRawModifier,
    ) -> bool {
        ctx_modifiers.clear();
        let valid = match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(loc) => match loc {
                SolLocation::Everything => {
                    let projectee_item_ids = self.affectee_buffable.get(&fw_effect.get_fit_id());
                    ctx_modifiers.reserve(projectee_item_ids.len());
                    for projectee_item_id in projectee_item_ids {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, *projectee_item_id);
                        add_ctx_modifier(
                            &mut self.cmods_direct,
                            *projectee_item_id,
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier)
                    }
                    true
                }
                SolLocation::Ship => {
                    let fit = uad.fits.get_fit(&fw_effect.get_fit_id()).unwrap();
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
                    true
                }
                _ => false,
            },
            SolAffecteeFilter::Loc(loc) => match loc {
                SolLocation::Everything | SolLocation::Ship => {
                    let fit = uad.fits.get_fit(&fw_effect.get_fit_id()).unwrap();
                    if let Some(ship_id) = fit.ship {
                        if matches!(fit.kind, SolShipKind::Ship) {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, ship_id);
                            add_ctx_modifier(
                                &mut self.cmods_loc,
                                (fw_effect.get_fit_id(), SolLocationKind::Ship),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            ctx_modifiers.push(ctx_modifier);
                        }
                    }
                    true
                }
                _ => false,
            },
            SolAffecteeFilter::LocGrp(loc, grp_id) => match loc {
                SolLocation::Everything | SolLocation::Ship => {
                    let fit = uad.fits.get_fit(&fw_effect.get_fit_id()).unwrap();
                    if let Some(ship_id) = fit.ship {
                        if matches!(fit.kind, SolShipKind::Ship) {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, ship_id);
                            add_ctx_modifier(
                                &mut self.cmods_loc_grp,
                                (fw_effect.get_fit_id(), SolLocationKind::Ship, grp_id),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            ctx_modifiers.push(ctx_modifier);
                        }
                    }
                    true
                }
                _ => false,
            },
            SolAffecteeFilter::LocSrq(loc, srq_id) => match loc {
                SolLocation::Everything | SolLocation::Ship => {
                    let fit = uad.fits.get_fit(&fw_effect.get_fit_id()).unwrap();
                    if let Some(ship_id) = fit.ship {
                        if matches!(fit.kind, SolShipKind::Ship) {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, ship_id);
                            add_ctx_modifier(
                                &mut self.cmods_loc_srq,
                                (fw_effect.get_fit_id(), SolLocationKind::Ship, srq_id),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            ctx_modifiers.push(ctx_modifier);
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
            self.rmods_fw_buff.add_entry(fw_effect.get_fit_id(), raw_modifier);
        }
        valid
    }
    pub(in crate::sol::svc::calc) fn unreg_fw_buff_mod(
        &mut self,
        ctx_modifiers: &mut Vec<SolCtxModifier>,
        uad: &SolUad,
        fw_effect: &SolFwEffect,
        raw_modifier: SolRawModifier,
    ) {
        ctx_modifiers.clear();
        match raw_modifier.affectee_filter {
            SolAffecteeFilter::Direct(loc) => match loc {
                SolLocation::Everything => {
                    let projectee_item_ids = self.affectee_buffable.get(&fw_effect.get_fit_id());
                    ctx_modifiers.reserve(projectee_item_ids.len());
                    for projectee_item_id in projectee_item_ids {
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, *projectee_item_id);
                        remove_ctx_modifier(
                            &mut self.cmods_direct,
                            projectee_item_id,
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                }
                SolLocation::Ship => {
                    let fit = uad.fits.get_fit(&fw_effect.get_fit_id()).unwrap();
                    if matches!(fit.kind, SolShipKind::Ship) {
                        if let Some(ship_id) = fit.ship {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, ship_id);
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
                _ => (),
            },
            SolAffecteeFilter::Loc(loc) => match loc {
                SolLocation::Everything | SolLocation::Ship => {
                    let fit = uad.fits.get_fit(&fw_effect.get_fit_id()).unwrap();
                    if let Some(ship_id) = fit.ship {
                        if matches!(fit.kind, SolShipKind::Ship) {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, ship_id);
                            remove_ctx_modifier(
                                &mut self.cmods_loc,
                                &(fw_effect.get_fit_id(), SolLocationKind::Ship),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            ctx_modifiers.push(ctx_modifier);
                        }
                    }
                }
                _ => (),
            },
            SolAffecteeFilter::LocGrp(loc, grp_id) => match loc {
                SolLocation::Everything | SolLocation::Ship => {
                    let fit = uad.fits.get_fit(&fw_effect.get_fit_id()).unwrap();
                    if let Some(ship_id) = fit.ship {
                        if matches!(fit.kind, SolShipKind::Ship) {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, ship_id);
                            remove_ctx_modifier(
                                &mut self.cmods_loc_grp,
                                &(fw_effect.get_fit_id(), SolLocationKind::Ship, grp_id),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            ctx_modifiers.push(ctx_modifier);
                        }
                    }
                }
                _ => (),
            },
            SolAffecteeFilter::LocSrq(loc, srq_id) => match loc {
                SolLocation::Everything | SolLocation::Ship => {
                    let fit = uad.fits.get_fit(&fw_effect.get_fit_id()).unwrap();
                    if let Some(ship_id) = fit.ship {
                        if matches!(fit.kind, SolShipKind::Ship) {
                            let ctx_modifier = SolCtxModifier::from_raw_with_item(raw_modifier, ship_id);
                            remove_ctx_modifier(
                                &mut self.cmods_loc_srq,
                                &(fw_effect.get_fit_id(), SolLocationKind::Ship, srq_id),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            ctx_modifiers.push(ctx_modifier);
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        }
        self.rmods_fw_buff.remove_entry(&fw_effect.get_fit_id(), &raw_modifier);
    }
    // Is supposed to be called only for buffable items
    pub(in crate::sol::svc::calc::registers::standard) fn reg_buffable_for_fw(&mut self, item: &SolItem) {
        let fit_id = match item.get_fit_id() {
            Some(fit_id) => fit_id,
            None => return,
        };
        for raw_modifier in self.rmods_fw_buff.get(&fit_id) {
            match raw_modifier.affectee_filter {
                SolAffecteeFilter::Direct(loc) => match loc {
                    SolLocation::Everything => {
                        let item_id = item.get_id();
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, item_id);
                        add_ctx_modifier(
                            &mut self.cmods_direct,
                            item_id,
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                    SolLocation::Ship => {
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
                SolAffecteeFilter::Loc(loc) => match loc {
                    SolLocation::Everything | SolLocation::Ship => {
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
                SolAffecteeFilter::LocGrp(loc, grp_id) => match loc {
                    SolLocation::Everything | SolLocation::Ship => {
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
                SolAffecteeFilter::LocSrq(loc, srq_id) => match loc {
                    SolLocation::Everything | SolLocation::Ship => {
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
    pub(in crate::sol::svc::calc::registers::standard) fn unreg_buffable_for_fw(&mut self, item: &SolItem) {
        let fit_id = match item.get_fit_id() {
            Some(fit_id) => fit_id,
            None => return,
        };
        for raw_modifier in self.rmods_fw_buff.get(&fit_id) {
            match raw_modifier.affectee_filter {
                SolAffecteeFilter::Direct(loc) => match loc {
                    SolLocation::Everything => {
                        let item_id = item.get_id();
                        let ctx_modifier = SolCtxModifier::from_raw_with_item(*raw_modifier, item_id);
                        remove_ctx_modifier(
                            &mut self.cmods_direct,
                            &item_id,
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                    SolLocation::Ship => {
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
                SolAffecteeFilter::Loc(loc) => match loc {
                    SolLocation::Everything | SolLocation::Ship => {
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
                SolAffecteeFilter::LocGrp(loc, grp_id) => match loc {
                    SolLocation::Everything | SolLocation::Ship => {
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
                SolAffecteeFilter::LocSrq(loc, srq_id) => match loc {
                    SolLocation::Everything | SolLocation::Ship => {
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
