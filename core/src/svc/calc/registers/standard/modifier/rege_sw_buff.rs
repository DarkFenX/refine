use super::{add_cmod, remove_cmod};
use crate::{
    def::ItemKey,
    svc::{
        SvcCtx,
        calc::{AffecteeFilter, CtxModifier, Location, LocationKind, RawModifier, registers::StandardRegister},
    },
    uad::{ShipKind, UadShip},
};

impl StandardRegister {
    pub(in crate::svc::calc) fn reg_sw_buff_mod(
        &mut self,
        reuse_cmods: &mut Vec<CtxModifier>,
        ctx: SvcCtx,
        rmod: RawModifier,
    ) -> bool {
        reuse_cmods.clear();
        let valid = match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Everything => {
                    for affectee_keys in self.affectee_buffable.values() {
                        reuse_cmods.reserve(affectee_keys.len());
                        for &affectee_key in affectee_keys {
                            let cmod = CtxModifier::from_raw_with_item(rmod, affectee_key);
                            add_cmod(&mut self.cmods_direct, affectee_key, cmod, &mut self.cmods_by_attr_spec);
                            reuse_cmods.push(cmod);
                        }
                    }
                    self.rmods_sw_buff_direct.insert(rmod);
                    true
                }
                Location::Ship => {
                    // Assume all fits are of ship type
                    reuse_cmods.reserve(ctx.uad.fits.len());
                    for (fit_key, fit) in ctx.uad.fits.iter() {
                        if matches!(fit.kind, ShipKind::Ship)
                            && let Some(ship_key) = fit.ship
                        {
                            let cmod = CtxModifier::from_raw_with_item(rmod, ship_key);
                            add_cmod(
                                &mut self.cmods_root,
                                (fit_key, LocationKind::Ship),
                                cmod,
                                &mut self.cmods_by_attr_spec,
                            );
                            reuse_cmods.push(cmod);
                        }
                    }
                    self.rmods_sw_buff_indirect.insert(rmod);
                    true
                }
                _ => false,
            },
            AffecteeFilter::Loc(Location::Everything | Location::Ship) => {
                // Assume all fits are of ship type
                reuse_cmods.reserve(ctx.uad.fits.len());
                for (fit_key, fit) in ctx.uad.fits.iter() {
                    if matches!(fit.kind, ShipKind::Ship)
                        && let Some(ship_key) = fit.ship
                    {
                        let cmod = CtxModifier::from_raw_with_item(rmod, ship_key);
                        add_cmod(
                            &mut self.cmods_loc,
                            (fit_key, LocationKind::Ship),
                            cmod,
                            &mut self.cmods_by_attr_spec,
                        );
                        reuse_cmods.push(cmod);
                    }
                }
                self.rmods_sw_buff_indirect.insert(rmod);
                true
            }
            AffecteeFilter::LocGrp(Location::Everything | Location::Ship, a_item_grp_id) => {
                // Assume all fits are of ship type
                reuse_cmods.reserve(ctx.uad.fits.len());
                for (fit_key, fit) in ctx.uad.fits.iter() {
                    if matches!(fit.kind, ShipKind::Ship)
                        && let Some(ship_key) = fit.ship
                    {
                        let cmod = CtxModifier::from_raw_with_item(rmod, ship_key);
                        add_cmod(
                            &mut self.cmods_loc_grp,
                            (fit_key, LocationKind::Ship, a_item_grp_id),
                            cmod,
                            &mut self.cmods_by_attr_spec,
                        );
                        reuse_cmods.push(cmod);
                    }
                }
                self.rmods_sw_buff_indirect.insert(rmod);
                true
            }
            AffecteeFilter::LocSrq(Location::Everything | Location::Ship, srq_a_item_id) => {
                // Assume all fits are of ship type
                reuse_cmods.reserve(ctx.uad.fits.len());
                for (fit_key, fit) in ctx.uad.fits.iter() {
                    if matches!(fit.kind, ShipKind::Ship)
                        && let Some(ship_key) = fit.ship
                    {
                        let cmod = CtxModifier::from_raw_with_item(rmod, ship_key);
                        add_cmod(
                            &mut self.cmods_loc_srq,
                            (fit_key, LocationKind::Ship, srq_a_item_id),
                            cmod,
                            &mut self.cmods_by_attr_spec,
                        );
                        reuse_cmods.push(cmod);
                    }
                }
                self.rmods_sw_buff_indirect.insert(rmod);
                true
            }
            _ => false,
        };
        if valid {
            self.rmods_all.add_entry(rmod.affector_espec, rmod);
        }
        valid
    }
    pub(in crate::svc::calc) fn unreg_sw_buff_mod(
        &mut self,
        reuse_cmods: &mut Vec<CtxModifier>,
        ctx: SvcCtx,
        rmod: &RawModifier,
    ) {
        reuse_cmods.clear();
        match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Everything => {
                    for affectee_keys in self.affectee_buffable.values() {
                        reuse_cmods.reserve(affectee_keys.len());
                        for affectee_key in affectee_keys {
                            let cmod = CtxModifier::from_raw_with_item(*rmod, *affectee_key);
                            remove_cmod(
                                &mut self.cmods_direct,
                                affectee_key,
                                &cmod,
                                &mut self.cmods_by_attr_spec,
                            );
                            reuse_cmods.push(cmod);
                        }
                    }
                    self.rmods_sw_buff_direct.remove(rmod);
                }
                Location::Ship => {
                    // Assume all fits are of ship type
                    reuse_cmods.reserve(ctx.uad.fits.len());
                    for (fit_key, fit) in ctx.uad.fits.iter() {
                        if matches!(fit.kind, ShipKind::Ship)
                            && let Some(ship_key) = fit.ship
                        {
                            let cmod = CtxModifier::from_raw_with_item(*rmod, ship_key);
                            remove_cmod(
                                &mut self.cmods_root,
                                &(fit_key, LocationKind::Ship),
                                &cmod,
                                &mut self.cmods_by_attr_spec,
                            );
                            reuse_cmods.push(cmod);
                        }
                    }
                    self.rmods_sw_buff_indirect.remove(rmod);
                }
                _ => (),
            },
            AffecteeFilter::Loc(Location::Everything | Location::Ship) => {
                // Assume all fits are of ship type
                reuse_cmods.reserve(ctx.uad.fits.len());
                for (fit_key, fit) in ctx.uad.fits.iter() {
                    if matches!(fit.kind, ShipKind::Ship)
                        && let Some(ship_key) = fit.ship
                    {
                        let cmod = CtxModifier::from_raw_with_item(*rmod, ship_key);
                        remove_cmod(
                            &mut self.cmods_loc,
                            &(fit_key, LocationKind::Ship),
                            &cmod,
                            &mut self.cmods_by_attr_spec,
                        );
                        reuse_cmods.push(cmod);
                    }
                }
                self.rmods_sw_buff_indirect.remove(rmod);
            }
            AffecteeFilter::LocGrp(Location::Everything | Location::Ship, a_item_grp_id) => {
                // Assume all fits are of ship type
                reuse_cmods.reserve(ctx.uad.fits.len());
                for (fit_key, fit) in ctx.uad.fits.iter() {
                    if matches!(fit.kind, ShipKind::Ship)
                        && let Some(ship_key) = fit.ship
                    {
                        let cmod = CtxModifier::from_raw_with_item(*rmod, ship_key);
                        remove_cmod(
                            &mut self.cmods_loc_grp,
                            &(fit_key, LocationKind::Ship, a_item_grp_id),
                            &cmod,
                            &mut self.cmods_by_attr_spec,
                        );
                        reuse_cmods.push(cmod);
                    }
                }
                self.rmods_sw_buff_indirect.remove(rmod);
            }
            AffecteeFilter::LocSrq(Location::Everything | Location::Ship, srq_a_item_id) => {
                // Assume all fits are of ship type
                reuse_cmods.reserve(ctx.uad.fits.len());
                for (fit_key, fit) in ctx.uad.fits.iter() {
                    if matches!(fit.kind, ShipKind::Ship)
                        && let Some(ship_key) = fit.ship
                    {
                        let cmod = CtxModifier::from_raw_with_item(*rmod, ship_key);
                        remove_cmod(
                            &mut self.cmods_loc_srq,
                            &(fit_key, LocationKind::Ship, srq_a_item_id),
                            &cmod,
                            &mut self.cmods_by_attr_spec,
                        );
                        reuse_cmods.push(cmod);
                    }
                }
                self.rmods_sw_buff_indirect.remove(rmod);
            }
            _ => (),
        }
    }
    // Is supposed to be called only for buffable items
    pub(in crate::svc::calc::registers::standard) fn reg_buffable_for_sw(&mut self, item_key: ItemKey) {
        for rmod in self.rmods_sw_buff_direct.iter() {
            if let AffecteeFilter::Direct(Location::Everything) = rmod.affectee_filter {
                let cmod = CtxModifier::from_raw_with_item(*rmod, item_key);
                add_cmod(&mut self.cmods_direct, item_key, cmod, &mut self.cmods_by_attr_spec);
            }
        }
    }
    // Is supposed to be called only for buffable items
    pub(in crate::svc::calc::registers::standard) fn unreg_buffable_for_sw(&mut self, item_key: ItemKey) {
        for rmod in self.rmods_sw_buff_direct.iter() {
            if let AffecteeFilter::Direct(Location::Everything) = rmod.affectee_filter {
                let cmod = CtxModifier::from_raw_with_item(*rmod, item_key);
                remove_cmod(&mut self.cmods_direct, &item_key, &cmod, &mut self.cmods_by_attr_spec);
            }
        }
    }
    // Is supposed to be called only for buffable location roots (ships)
    pub(super) fn reg_loc_root_for_sw_buff(&mut self, ship_key: ItemKey, ship: &UadShip) {
        for rmod in self.rmods_sw_buff_indirect.iter() {
            match rmod.affectee_filter {
                AffecteeFilter::Direct(Location::Ship) => {
                    if matches!(ship.get_kind(), ShipKind::Ship) {
                        let cmod = CtxModifier::from_raw_with_item(*rmod, ship_key);
                        add_cmod(
                            &mut self.cmods_root,
                            (ship.get_fit_key(), LocationKind::Ship),
                            cmod,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                AffecteeFilter::Loc(Location::Everything | Location::Ship) => {
                    if matches!(ship.get_kind(), ShipKind::Ship) {
                        let cmod = CtxModifier::from_raw_with_item(*rmod, ship_key);
                        add_cmod(
                            &mut self.cmods_loc,
                            (ship.get_fit_key(), LocationKind::Ship),
                            cmod,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                AffecteeFilter::LocGrp(Location::Everything | Location::Ship, a_item_grp_id) => {
                    if matches!(ship.get_kind(), ShipKind::Ship) {
                        let cmod = CtxModifier::from_raw_with_item(*rmod, ship_key);
                        add_cmod(
                            &mut self.cmods_loc_grp,
                            (ship.get_fit_key(), LocationKind::Ship, a_item_grp_id),
                            cmod,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                AffecteeFilter::LocSrq(Location::Everything | Location::Ship, srq_a_item_id) => {
                    if matches!(ship.get_kind(), ShipKind::Ship) {
                        let cmod = CtxModifier::from_raw_with_item(*rmod, ship_key);
                        add_cmod(
                            &mut self.cmods_loc_srq,
                            (ship.get_fit_key(), LocationKind::Ship, srq_a_item_id),
                            cmod,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                _ => (),
            };
        }
    }
    // Is supposed to be called only for buffable location roots (ships)
    pub(super) fn unreg_loc_root_for_sw_buff(&mut self, ship_key: ItemKey, ship: &UadShip) {
        for rmod in self.rmods_sw_buff_indirect.iter() {
            match rmod.affectee_filter {
                AffecteeFilter::Direct(Location::Ship) => {
                    if matches!(ship.get_kind(), ShipKind::Ship) {
                        let cmod = CtxModifier::from_raw_with_item(*rmod, ship_key);
                        remove_cmod(
                            &mut self.cmods_root,
                            &(ship.get_fit_key(), LocationKind::Ship),
                            &cmod,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                AffecteeFilter::Loc(Location::Everything | Location::Ship) => {
                    if matches!(ship.get_kind(), ShipKind::Ship) {
                        let cmod = CtxModifier::from_raw_with_item(*rmod, ship_key);
                        remove_cmod(
                            &mut self.cmods_loc,
                            &(ship.get_fit_key(), LocationKind::Ship),
                            &cmod,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                AffecteeFilter::LocGrp(Location::Everything | Location::Ship, a_item_grp_id) => {
                    if matches!(ship.get_kind(), ShipKind::Ship) {
                        let cmod = CtxModifier::from_raw_with_item(*rmod, ship_key);
                        remove_cmod(
                            &mut self.cmods_loc_grp,
                            &(ship.get_fit_key(), LocationKind::Ship, a_item_grp_id),
                            &cmod,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                AffecteeFilter::LocSrq(Location::Everything | Location::Ship, srq_a_item_id) => {
                    if matches!(ship.get_kind(), ShipKind::Ship) {
                        let cmod = CtxModifier::from_raw_with_item(*rmod, ship_key);
                        remove_cmod(
                            &mut self.cmods_loc_srq,
                            &(ship.get_fit_key(), LocationKind::Ship, srq_a_item_id),
                            &cmod,
                            &mut self.cmods_by_attr_spec,
                        );
                    }
                }
                _ => (),
            };
        }
    }
}
