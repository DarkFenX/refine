use super::{add_ctx_modifier, remove_ctx_modifier};
use crate::sol::{
    ItemKey,
    svc::calc::{AffecteeFilter, CtxModifier, Location, LocationKind, RawModifier, registers::StandardRegister},
    uad::{
        Uad,
        item::{ShipKind, UadItem},
    },
};

impl StandardRegister {
    pub(in crate::sol::svc::calc) fn reg_sw_buff_mod(
        &mut self,
        ctx_modifiers: &mut Vec<CtxModifier>,
        uad: &Uad,
        raw_modifier: RawModifier,
    ) -> bool {
        ctx_modifiers.clear();
        let valid = match raw_modifier.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Everything => {
                    for affectee_item_keys in self.affectee_buffable.values() {
                        ctx_modifiers.reserve(affectee_item_keys.len());
                        for &affectee_item_key in affectee_item_keys {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, affectee_item_key);
                            add_ctx_modifier(
                                &mut self.cmods_direct,
                                affectee_item_key,
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            ctx_modifiers.push(ctx_modifier);
                        }
                    }
                    self.rmods_sw_buff_direct.insert(raw_modifier);
                    true
                }
                Location::Ship => {
                    // Assume all fits are of ship type
                    ctx_modifiers.reserve(uad.fits.len());
                    for (fit_key, fit) in uad.fits.iter() {
                        if matches!(fit.kind, ShipKind::Ship)
                            && let Some(ship_key) = fit.ship
                        {
                            let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, ship_key);
                            add_ctx_modifier(
                                &mut self.cmods_root,
                                (fit_key, LocationKind::Ship),
                                ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            ctx_modifiers.push(ctx_modifier);
                        }
                    }
                    self.rmods_sw_buff_indirect.insert(raw_modifier);
                    true
                }
                _ => false,
            },
            AffecteeFilter::Loc(Location::Everything | Location::Ship) => {
                // Assume all fits are of ship type
                ctx_modifiers.reserve(uad.fits.len());
                for (fit_key, fit) in uad.fits.iter() {
                    if matches!(fit.kind, ShipKind::Ship)
                        && let Some(ship_key) = fit.ship
                    {
                        let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, ship_key);
                        add_ctx_modifier(
                            &mut self.cmods_loc,
                            (fit_key, LocationKind::Ship),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                }
                self.rmods_sw_buff_indirect.insert(raw_modifier);
                true
            }
            AffecteeFilter::LocGrp(Location::Everything | Location::Ship, a_item_grp_id) => {
                // Assume all fits are of ship type
                ctx_modifiers.reserve(uad.fits.len());
                for (fit_key, fit) in uad.fits.iter() {
                    if matches!(fit.kind, ShipKind::Ship)
                        && let Some(ship_key) = fit.ship
                    {
                        let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, ship_key);
                        add_ctx_modifier(
                            &mut self.cmods_loc_grp,
                            (fit_key, LocationKind::Ship, a_item_grp_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                }
                self.rmods_sw_buff_indirect.insert(raw_modifier);
                true
            }
            AffecteeFilter::LocSrq(Location::Everything | Location::Ship, srq_a_item_id) => {
                // Assume all fits are of ship type
                ctx_modifiers.reserve(uad.fits.len());
                for (fit_key, fit) in uad.fits.iter() {
                    if matches!(fit.kind, ShipKind::Ship)
                        && let Some(ship_key) = fit.ship
                    {
                        let ctx_modifier = CtxModifier::from_raw_with_item(raw_modifier, ship_key);
                        add_ctx_modifier(
                            &mut self.cmods_loc_srq,
                            (fit_key, LocationKind::Ship, srq_a_item_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                }
                self.rmods_sw_buff_indirect.insert(raw_modifier);
                true
            }
            _ => false,
        };
        if valid {
            self.rmods_all.add_entry(raw_modifier.affector_espec, raw_modifier);
        }
        valid
    }
    pub(in crate::sol::svc::calc) fn unreg_sw_buff_mod(
        &mut self,
        ctx_modifiers: &mut Vec<CtxModifier>,
        uad: &Uad,
        raw_modifier: &RawModifier,
    ) {
        ctx_modifiers.clear();
        match raw_modifier.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Everything => {
                    for affectee_item_keys in self.affectee_buffable.values() {
                        ctx_modifiers.reserve(affectee_item_keys.len());
                        for affectee_item_key in affectee_item_keys {
                            let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, *affectee_item_key);
                            remove_ctx_modifier(
                                &mut self.cmods_direct,
                                affectee_item_key,
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            ctx_modifiers.push(ctx_modifier);
                        }
                    }
                    self.rmods_sw_buff_direct.remove(raw_modifier);
                }
                Location::Ship => {
                    // Assume all fits are of ship type
                    ctx_modifiers.reserve(uad.fits.len());
                    for (fit_key, fit) in uad.fits.iter() {
                        if matches!(fit.kind, ShipKind::Ship)
                            && let Some(ship_key) = fit.ship
                        {
                            let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, ship_key);
                            remove_ctx_modifier(
                                &mut self.cmods_root,
                                &(fit_key, LocationKind::Ship),
                                &ctx_modifier,
                                &mut self.cmods_by_attr_spec,
                            );
                            ctx_modifiers.push(ctx_modifier);
                        }
                    }
                    self.rmods_sw_buff_indirect.remove(raw_modifier);
                }
                _ => (),
            },
            AffecteeFilter::Loc(Location::Everything | Location::Ship) => {
                // Assume all fits are of ship type
                ctx_modifiers.reserve(uad.fits.len());
                for (fit_key, fit) in uad.fits.iter() {
                    if matches!(fit.kind, ShipKind::Ship)
                        && let Some(ship_key) = fit.ship
                    {
                        let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, ship_key);
                        remove_ctx_modifier(
                            &mut self.cmods_loc,
                            &(fit_key, LocationKind::Ship),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                }
                self.rmods_sw_buff_indirect.remove(raw_modifier);
            }
            AffecteeFilter::LocGrp(Location::Everything | Location::Ship, a_item_grp_id) => {
                // Assume all fits are of ship type
                ctx_modifiers.reserve(uad.fits.len());
                for (fit_key, fit) in uad.fits.iter() {
                    if matches!(fit.kind, ShipKind::Ship)
                        && let Some(ship_key) = fit.ship
                    {
                        let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, ship_key);
                        remove_ctx_modifier(
                            &mut self.cmods_loc_grp,
                            &(fit_key, LocationKind::Ship, a_item_grp_id),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                }
                self.rmods_sw_buff_indirect.remove(raw_modifier);
            }
            AffecteeFilter::LocSrq(Location::Everything | Location::Ship, srq_a_item_id) => {
                // Assume all fits are of ship type
                ctx_modifiers.reserve(uad.fits.len());
                for (fit_key, fit) in uad.fits.iter() {
                    if matches!(fit.kind, ShipKind::Ship)
                        && let Some(ship_key) = fit.ship
                    {
                        let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, ship_key);
                        remove_ctx_modifier(
                            &mut self.cmods_loc_srq,
                            &(fit_key, LocationKind::Ship, srq_a_item_id),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                        ctx_modifiers.push(ctx_modifier);
                    }
                }
                self.rmods_sw_buff_indirect.remove(raw_modifier);
            }
            _ => (),
        }
    }
    // Is supposed to be called only for buffable items
    pub(in crate::sol::svc::calc::registers::standard) fn reg_buffable_for_sw(&mut self, item_key: ItemKey) {
        for raw_modifier in self.rmods_sw_buff_direct.iter() {
            if let AffecteeFilter::Direct(Location::Everything) = raw_modifier.affectee_filter {
                let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, item_key);
                add_ctx_modifier(
                    &mut self.cmods_direct,
                    item_key,
                    ctx_modifier,
                    &mut self.cmods_by_attr_spec,
                );
            }
        }
    }
    // Is supposed to be called only for buffable items
    pub(in crate::sol::svc::calc::registers::standard) fn unreg_buffable_for_sw(&mut self, item_key: ItemKey) {
        for raw_modifier in self.rmods_sw_buff_direct.iter() {
            if let AffecteeFilter::Direct(Location::Everything) = raw_modifier.affectee_filter {
                let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, item_key);
                remove_ctx_modifier(
                    &mut self.cmods_direct,
                    &item_key,
                    &ctx_modifier,
                    &mut self.cmods_by_attr_spec,
                );
            }
        }
    }
    // Is supposed to be called only for buffable location roots (ships)
    pub(super) fn reg_loc_root_for_sw_buff(&mut self, item_key: ItemKey, item: &UadItem) {
        for raw_modifier in self.rmods_sw_buff_indirect.iter() {
            match raw_modifier.affectee_filter {
                AffecteeFilter::Direct(Location::Ship) => {
                    if let UadItem::Ship(ship) = item
                        && matches!(ship.get_kind(), ShipKind::Ship)
                    {
                        let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, item_key);
                        add_ctx_modifier(
                            &mut self.cmods_root,
                            (ship.get_fit_key(), LocationKind::Ship),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                AffecteeFilter::Loc(Location::Everything | Location::Ship) => {
                    if let UadItem::Ship(ship) = item
                        && matches!(ship.get_kind(), ShipKind::Ship)
                    {
                        let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, item_key);
                        add_ctx_modifier(
                            &mut self.cmods_loc,
                            (ship.get_fit_key(), LocationKind::Ship),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                AffecteeFilter::LocGrp(Location::Everything | Location::Ship, a_item_grp_id) => {
                    if let UadItem::Ship(ship) = item
                        && matches!(ship.get_kind(), ShipKind::Ship)
                    {
                        let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, item_key);
                        add_ctx_modifier(
                            &mut self.cmods_loc_grp,
                            (ship.get_fit_key(), LocationKind::Ship, a_item_grp_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                AffecteeFilter::LocSrq(Location::Everything | Location::Ship, srq_a_item_id) => {
                    if let UadItem::Ship(ship) = item
                        && matches!(ship.get_kind(), ShipKind::Ship)
                    {
                        let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, item_key);
                        add_ctx_modifier(
                            &mut self.cmods_loc_srq,
                            (ship.get_fit_key(), LocationKind::Ship, srq_a_item_id),
                            ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                _ => (),
            };
        }
    }
    // Is supposed to be called only for buffable location roots (ships)
    pub(super) fn unreg_loc_root_for_sw_buff(&mut self, item_key: ItemKey, item: &UadItem) {
        for raw_modifier in self.rmods_sw_buff_indirect.iter() {
            match raw_modifier.affectee_filter {
                AffecteeFilter::Direct(Location::Ship) => {
                    if let UadItem::Ship(ship) = item
                        && matches!(ship.get_kind(), ShipKind::Ship)
                    {
                        let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, item_key);
                        remove_ctx_modifier(
                            &mut self.cmods_root,
                            &(ship.get_fit_key(), LocationKind::Ship),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                AffecteeFilter::Loc(Location::Everything | Location::Ship) => {
                    if let UadItem::Ship(ship) = item
                        && matches!(ship.get_kind(), ShipKind::Ship)
                    {
                        let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, item_key);
                        remove_ctx_modifier(
                            &mut self.cmods_loc,
                            &(ship.get_fit_key(), LocationKind::Ship),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                AffecteeFilter::LocGrp(Location::Everything | Location::Ship, a_item_grp_id) => {
                    if let UadItem::Ship(ship) = item
                        && matches!(ship.get_kind(), ShipKind::Ship)
                    {
                        let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, item_key);
                        remove_ctx_modifier(
                            &mut self.cmods_loc_grp,
                            &(ship.get_fit_key(), LocationKind::Ship, a_item_grp_id),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                AffecteeFilter::LocSrq(Location::Everything | Location::Ship, srq_a_item_id) => {
                    if let UadItem::Ship(ship) = item
                        && matches!(ship.get_kind(), ShipKind::Ship)
                    {
                        let ctx_modifier = CtxModifier::from_raw_with_item(*raw_modifier, item_key);
                        remove_ctx_modifier(
                            &mut self.cmods_loc_srq,
                            &(ship.get_fit_key(), LocationKind::Ship, srq_a_item_id),
                            &ctx_modifier,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                _ => (),
            };
        }
    }
}
