use crate::{
    ad::AItemListId,
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, CtxModifier, Location, LocationKind, RawModifier,
            registers::standard::{
                StandardRegister,
                func::{add_cmod, remove_cmod},
            },
        },
    },
    ud::{UItemKey, UShip},
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
            AffecteeFilter::Direct(Location::ItemList(item_list_id)) => {
                // TODO: consider optimizations (extra map with item list IDs
                // TODO: as keys to know fits instead of iterating over them?)
                for fit_key in ctx.u_data.fits.keys() {
                    let affectee_keys = self.affectee_buffable.get(&(fit_key, item_list_id));
                    reuse_cmods.reserve(affectee_keys.len());
                    for &affectee_key in affectee_keys {
                        let cmod = CtxModifier::from_raw_with_item(rmod, affectee_key);
                        add_cmod(&mut self.cmods_direct, affectee_key, cmod, &mut self.cmods_by_aspec);
                        reuse_cmods.push(cmod);
                    }
                }
                self.rmods_sw_buff_direct.insert(rmod);
                true
            }
            AffecteeFilter::Loc(Location::ItemList(item_list_id)) => {
                let affectee_keys = self.affectee_buffable_ships.get(&item_list_id);
                reuse_cmods.reserve(affectee_keys.len());
                for &(fit_key, item_key) in affectee_keys {
                    let cmod = CtxModifier::from_raw_with_item(rmod, item_key);
                    add_cmod(
                        &mut self.cmods_loc,
                        (fit_key, LocationKind::Ship),
                        cmod,
                        &mut self.cmods_by_aspec,
                    );
                    reuse_cmods.push(cmod);
                }
                self.rmods_sw_buff_indirect.insert(rmod);
                true
            }
            AffecteeFilter::LocGrp(Location::ItemList(item_list_id), a_item_grp_id) => {
                let affectee_keys = self.affectee_buffable_ships.get(&item_list_id);
                reuse_cmods.reserve(affectee_keys.len());
                for &(fit_key, item_key) in affectee_keys {
                    let cmod = CtxModifier::from_raw_with_item(rmod, item_key);
                    add_cmod(
                        &mut self.cmods_loc_grp,
                        (fit_key, LocationKind::Ship, a_item_grp_id),
                        cmod,
                        &mut self.cmods_by_aspec,
                    );
                    reuse_cmods.push(cmod);
                }
                self.rmods_sw_buff_indirect.insert(rmod);
                true
            }
            AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_a_item_id) => {
                let affectee_keys = self.affectee_buffable_ships.get(&item_list_id);
                reuse_cmods.reserve(affectee_keys.len());
                for &(fit_key, item_key) in affectee_keys {
                    let cmod = CtxModifier::from_raw_with_item(rmod, item_key);
                    add_cmod(
                        &mut self.cmods_loc_srq,
                        (fit_key, LocationKind::Ship, srq_a_item_id),
                        cmod,
                        &mut self.cmods_by_aspec,
                    );
                    reuse_cmods.push(cmod);
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
            AffecteeFilter::Direct(Location::ItemList(item_list_id)) => {
                // TODO: consider optimizations (extra map with item list IDs
                // TODO: as keys to know fits instead of iterating over them?)
                for fit_key in ctx.u_data.fits.keys() {
                    let affectee_keys = self.affectee_buffable.get(&(fit_key, item_list_id));
                    reuse_cmods.reserve(affectee_keys.len());
                    for &affectee_key in affectee_keys {
                        let cmod = CtxModifier::from_raw_with_item(*rmod, affectee_key);
                        remove_cmod(&mut self.cmods_direct, affectee_key, &cmod, &mut self.cmods_by_aspec);
                        reuse_cmods.push(cmod);
                    }
                }
                self.rmods_sw_buff_direct.remove(rmod);
            }
            AffecteeFilter::Loc(Location::ItemList(item_list_id)) => {
                let affectee_keys = self.affectee_buffable_ships.get(&item_list_id);
                reuse_cmods.reserve(affectee_keys.len());
                for &(fit_key, item_key) in affectee_keys {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, item_key);
                    remove_cmod(
                        &mut self.cmods_loc,
                        (fit_key, LocationKind::Ship),
                        &cmod,
                        &mut self.cmods_by_aspec,
                    );
                    reuse_cmods.push(cmod);
                }
                self.rmods_sw_buff_indirect.remove(rmod);
            }
            AffecteeFilter::LocGrp(Location::ItemList(item_list_id), a_item_grp_id) => {
                let affectee_keys = self.affectee_buffable_ships.get(&item_list_id);
                reuse_cmods.reserve(affectee_keys.len());
                for &(fit_key, item_key) in affectee_keys {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, item_key);
                    remove_cmod(
                        &mut self.cmods_loc_grp,
                        (fit_key, LocationKind::Ship, a_item_grp_id),
                        &cmod,
                        &mut self.cmods_by_aspec,
                    );
                    reuse_cmods.push(cmod);
                }
                self.rmods_sw_buff_indirect.remove(rmod);
            }
            AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_a_item_id) => {
                let affectee_keys = self.affectee_buffable_ships.get(&item_list_id);
                reuse_cmods.reserve(affectee_keys.len());
                for &(fit_key, item_key) in affectee_keys {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, item_key);
                    remove_cmod(
                        &mut self.cmods_loc_srq,
                        (fit_key, LocationKind::Ship, srq_a_item_id),
                        &cmod,
                        &mut self.cmods_by_aspec,
                    );
                    reuse_cmods.push(cmod);
                }
                self.rmods_sw_buff_indirect.remove(rmod);
            }
            _ => (),
        }
    }
    pub(in crate::svc::calc::registers::standard) fn reg_buffable_for_sw(
        &mut self,
        item_key: UItemKey,
        buffable_item_lists: &[AItemListId],
    ) {
        for rmod in self.rmods_sw_buff_direct.iter() {
            if let AffecteeFilter::Direct(Location::ItemList(item_list_id)) = rmod.affectee_filter
                && buffable_item_lists.contains(&item_list_id)
            {
                let cmod = CtxModifier::from_raw_with_item(*rmod, item_key);
                add_cmod(&mut self.cmods_direct, item_key, cmod, &mut self.cmods_by_aspec);
            }
        }
    }
    pub(in crate::svc::calc::registers::standard) fn unreg_buffable_for_sw(
        &mut self,
        item_key: UItemKey,
        buffable_item_lists: &[AItemListId],
    ) {
        for rmod in self.rmods_sw_buff_direct.iter() {
            if let AffecteeFilter::Direct(Location::ItemList(item_list_id)) = rmod.affectee_filter
                && buffable_item_lists.contains(&item_list_id)
            {
                let cmod = CtxModifier::from_raw_with_item(*rmod, item_key);
                remove_cmod(&mut self.cmods_direct, item_key, &cmod, &mut self.cmods_by_aspec);
            }
        }
    }
    pub(super) fn reg_loc_root_for_sw_buff(
        &mut self,
        ship_key: UItemKey,
        ship: &UShip,
        buffable_item_lists: &[AItemListId],
    ) {
        for rmod in self.rmods_sw_buff_indirect.iter() {
            match rmod.affectee_filter {
                AffecteeFilter::Direct(Location::ItemList(item_list_id))
                    if buffable_item_lists.contains(&item_list_id) =>
                {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, ship_key);
                    add_cmod(
                        &mut self.cmods_root,
                        (ship.get_fit_key(), LocationKind::Ship),
                        cmod,
                        &mut self.cmods_by_aspec,
                    );
                }
                AffecteeFilter::Loc(Location::ItemList(item_list_id))
                    if buffable_item_lists.contains(&item_list_id) =>
                {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, ship_key);
                    add_cmod(
                        &mut self.cmods_loc,
                        (ship.get_fit_key(), LocationKind::Ship),
                        cmod,
                        &mut self.cmods_by_aspec,
                    );
                }
                AffecteeFilter::LocGrp(Location::ItemList(item_list_id), a_item_grp_id)
                    if buffable_item_lists.contains(&item_list_id) =>
                {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, ship_key);
                    add_cmod(
                        &mut self.cmods_loc_grp,
                        (ship.get_fit_key(), LocationKind::Ship, a_item_grp_id),
                        cmod,
                        &mut self.cmods_by_aspec,
                    );
                }
                AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_a_item_id)
                    if buffable_item_lists.contains(&item_list_id) =>
                {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, ship_key);
                    add_cmod(
                        &mut self.cmods_loc_srq,
                        (ship.get_fit_key(), LocationKind::Ship, srq_a_item_id),
                        cmod,
                        &mut self.cmods_by_aspec,
                    );
                }
                _ => (),
            };
        }
    }
    // Is supposed to be called only for buffable location roots (ships)
    pub(super) fn unreg_loc_root_for_sw_buff(
        &mut self,
        ship_key: UItemKey,
        ship: &UShip,
        buffable_item_lists: &[AItemListId],
    ) {
        for rmod in self.rmods_sw_buff_indirect.iter() {
            match rmod.affectee_filter {
                AffecteeFilter::Direct(Location::ItemList(item_list_id))
                    if buffable_item_lists.contains(&item_list_id) =>
                {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, ship_key);
                    remove_cmod(
                        &mut self.cmods_root,
                        (ship.get_fit_key(), LocationKind::Ship),
                        &cmod,
                        &mut self.cmods_by_aspec,
                    );
                }
                AffecteeFilter::Loc(Location::ItemList(item_list_id))
                    if buffable_item_lists.contains(&item_list_id) =>
                {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, ship_key);
                    remove_cmod(
                        &mut self.cmods_loc,
                        (ship.get_fit_key(), LocationKind::Ship),
                        &cmod,
                        &mut self.cmods_by_aspec,
                    );
                }
                AffecteeFilter::LocGrp(Location::ItemList(item_list_id), a_item_grp_id)
                    if buffable_item_lists.contains(&item_list_id) =>
                {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, ship_key);
                    remove_cmod(
                        &mut self.cmods_loc_grp,
                        (ship.get_fit_key(), LocationKind::Ship, a_item_grp_id),
                        &cmod,
                        &mut self.cmods_by_aspec,
                    );
                }
                AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_a_item_id)
                    if buffable_item_lists.contains(&item_list_id) =>
                {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, ship_key);
                    remove_cmod(
                        &mut self.cmods_loc_srq,
                        (ship.get_fit_key(), LocationKind::Ship, srq_a_item_id),
                        &cmod,
                        &mut self.cmods_by_aspec,
                    );
                }
                _ => (),
            };
        }
    }
}
