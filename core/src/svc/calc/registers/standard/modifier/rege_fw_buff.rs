use crate::{
    ad::AItemListId,
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, Location, LocationKind, RawModifier,
            modifier::CtxModifier,
            registers::standard::{
                StandardRegister,
                func::{add_cmod, is_fit_ship_on_item_list, remove_cmod},
            },
        },
    },
    ud::{UFitKey, UFwEffect, UItemKey, UShip},
};

impl StandardRegister {
    pub(in crate::svc::calc) fn reg_fw_buff_mod(
        &mut self,
        reuse_cmods: &mut Vec<CtxModifier>,
        ctx: SvcCtx,
        fw_effect: &UFwEffect,
        rmod: RawModifier,
    ) -> bool {
        reuse_cmods.clear();
        let valid = match rmod.affectee_filter {
            AffecteeFilter::Direct(Location::ItemList(item_list_id)) => {
                let fit_key = fw_effect.get_fit_key();
                let affectee_keys = self.affectee_buffable.get(&(fit_key, item_list_id));
                reuse_cmods.reserve(affectee_keys.len());
                for &affectee_key in affectee_keys {
                    let cmod = CtxModifier::from_raw_with_item(rmod, affectee_key);
                    add_cmod(&mut self.cmods_direct, affectee_key, cmod, &mut self.cmods_by_aspec);
                    reuse_cmods.push(cmod);
                }
                self.rmods_fw_buff_direct.add_entry(fit_key, rmod);
                true
            }
            AffecteeFilter::Loc(Location::ItemList(item_list_id)) => {
                let fit_key = fw_effect.get_fit_key();
                if let Some(ship_key) = is_fit_ship_on_item_list(ctx, fit_key, &item_list_id) {
                    let cmod = CtxModifier::from_raw_with_item(rmod, ship_key);
                    add_cmod(
                        &mut self.cmods_loc,
                        (fit_key, LocationKind::Ship),
                        cmod,
                        &mut self.cmods_by_aspec,
                    );
                    reuse_cmods.push(cmod);
                }
                self.rmods_fw_buff_indirect.add_entry(fit_key, rmod);
                true
            }
            AffecteeFilter::LocGrp(Location::ItemList(item_list_id), a_item_grp_id) => {
                let fit_key = fw_effect.get_fit_key();
                if let Some(ship_key) = is_fit_ship_on_item_list(ctx, fit_key, &item_list_id) {
                    let cmod = CtxModifier::from_raw_with_item(rmod, ship_key);
                    add_cmod(
                        &mut self.cmods_loc_grp,
                        (fit_key, LocationKind::Ship, a_item_grp_id),
                        cmod,
                        &mut self.cmods_by_aspec,
                    );
                    reuse_cmods.push(cmod);
                }
                self.rmods_fw_buff_indirect.add_entry(fit_key, rmod);
                true
            }
            AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_a_item_id) => {
                let fit_key = fw_effect.get_fit_key();
                if let Some(ship_key) = is_fit_ship_on_item_list(ctx, fit_key, &item_list_id) {
                    let cmod = CtxModifier::from_raw_with_item(rmod, ship_key);
                    add_cmod(
                        &mut self.cmods_loc_srq,
                        (fit_key, LocationKind::Ship, srq_a_item_id),
                        cmod,
                        &mut self.cmods_by_aspec,
                    );
                    reuse_cmods.push(cmod);
                }
                self.rmods_fw_buff_indirect.add_entry(fit_key, rmod);
                true
            }
            _ => false,
        };
        if valid {
            self.rmods_all.add_entry(rmod.affector_espec, rmod);
        }
        valid
    }
    pub(in crate::svc::calc) fn unreg_fw_buff_mod(
        &mut self,
        reuse_cmods: &mut Vec<CtxModifier>,
        ctx: SvcCtx,
        fw_effect: &UFwEffect,
        rmod: RawModifier,
    ) {
        reuse_cmods.clear();
        match rmod.affectee_filter {
            AffecteeFilter::Direct(Location::ItemList(item_list_id)) => {
                let fit_key = fw_effect.get_fit_key();
                let affectee_keys = self.affectee_buffable.get(&(fit_key, item_list_id));
                reuse_cmods.reserve(affectee_keys.len());
                for affectee_key in affectee_keys {
                    let cmod = CtxModifier::from_raw_with_item(rmod, *affectee_key);
                    remove_cmod(&mut self.cmods_direct, affectee_key, &cmod, &mut self.cmods_by_aspec);
                    reuse_cmods.push(cmod);
                }
                self.rmods_fw_buff_direct.remove_entry(&fit_key, &rmod);
            }
            AffecteeFilter::Loc(Location::ItemList(item_list_id)) => {
                let fit_key = fw_effect.get_fit_key();
                if let Some(ship_key) = is_fit_ship_on_item_list(ctx, fit_key, &item_list_id) {
                    let cmod = CtxModifier::from_raw_with_item(rmod, ship_key);
                    remove_cmod(
                        &mut self.cmods_loc,
                        &(fit_key, LocationKind::Ship),
                        &cmod,
                        &mut self.cmods_by_aspec,
                    );
                    reuse_cmods.push(cmod);
                }
                self.rmods_fw_buff_indirect.remove_entry(&fit_key, &rmod);
            }
            AffecteeFilter::LocGrp(Location::ItemList(item_list_id), a_item_grp_id) => {
                let fit_key = fw_effect.get_fit_key();
                if let Some(ship_key) = is_fit_ship_on_item_list(ctx, fit_key, &item_list_id) {
                    let cmod = CtxModifier::from_raw_with_item(rmod, ship_key);
                    remove_cmod(
                        &mut self.cmods_loc_grp,
                        &(fit_key, LocationKind::Ship, a_item_grp_id),
                        &cmod,
                        &mut self.cmods_by_aspec,
                    );
                    reuse_cmods.push(cmod);
                }
                self.rmods_fw_buff_indirect.remove_entry(&fit_key, &rmod);
            }
            AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_a_item_id) => {
                let fit_key = fw_effect.get_fit_key();
                if let Some(ship_key) = is_fit_ship_on_item_list(ctx, fit_key, &item_list_id) {
                    let cmod = CtxModifier::from_raw_with_item(rmod, ship_key);
                    remove_cmod(
                        &mut self.cmods_loc_srq,
                        &(fit_key, LocationKind::Ship, srq_a_item_id),
                        &cmod,
                        &mut self.cmods_by_aspec,
                    );
                    reuse_cmods.push(cmod);
                }
                self.rmods_fw_buff_indirect.remove_entry(&fit_key, &rmod);
            }
            _ => (),
        }
    }
    pub(in crate::svc::calc::registers::standard) fn reg_buffable_for_fw(
        &mut self,
        item_key: UItemKey,
        fit_key: UFitKey,
        buffable_item_lists: &Vec<AItemListId>,
    ) {
        for rmod in self.rmods_fw_buff_direct.get(&fit_key) {
            if let AffecteeFilter::Direct(Location::ItemList(item_list_id)) = rmod.affectee_filter
                && buffable_item_lists.contains(&item_list_id)
            {
                let cmod = CtxModifier::from_raw_with_item(*rmod, item_key);
                add_cmod(&mut self.cmods_direct, item_key, cmod, &mut self.cmods_by_aspec);
            }
        }
    }
    pub(in crate::svc::calc::registers::standard) fn unreg_buffable_for_fw(
        &mut self,
        item_key: UItemKey,
        fit_key: UFitKey,
        buffable_item_lists: &Vec<AItemListId>,
    ) {
        for rmod in self.rmods_fw_buff_direct.get(&fit_key) {
            if let AffecteeFilter::Direct(Location::ItemList(item_list_id)) = rmod.affectee_filter
                && buffable_item_lists.contains(&item_list_id)
            {
                let cmod = CtxModifier::from_raw_with_item(*rmod, item_key);
                remove_cmod(&mut self.cmods_direct, &item_key, &cmod, &mut self.cmods_by_aspec);
            }
        }
    }
    // Is supposed to be called only for buffable location roots (ships)
    pub(super) fn reg_loc_root_for_fw_buff(
        &mut self,
        ship_key: UItemKey,
        ship: &UShip,
        buffable_item_lists: &Vec<AItemListId>,
    ) {
        let fit_key = ship.get_fit_key();
        for rmod in self.rmods_fw_buff_indirect.get(&fit_key) {
            match rmod.affectee_filter {
                AffecteeFilter::Direct(Location::ItemList(item_list_id))
                    if buffable_item_lists.contains(&item_list_id) =>
                {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, ship_key);
                    add_cmod(
                        &mut self.cmods_root,
                        (fit_key, LocationKind::Ship),
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
                        (fit_key, LocationKind::Ship),
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
                        (fit_key, LocationKind::Ship, a_item_grp_id),
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
                        (fit_key, LocationKind::Ship, srq_a_item_id),
                        cmod,
                        &mut self.cmods_by_aspec,
                    );
                }
                _ => (),
            };
        }
    }
    pub(super) fn unreg_loc_root_for_fw_buff(
        &mut self,
        ship_key: UItemKey,
        ship: &UShip,
        buffable_item_lists: &Vec<AItemListId>,
    ) {
        let fit_key = ship.get_fit_key();
        for rmod in self.rmods_fw_buff_indirect.get(&fit_key) {
            match rmod.affectee_filter {
                AffecteeFilter::Direct(Location::ItemList(item_list_id))
                    if buffable_item_lists.contains(&item_list_id) =>
                {
                    let cmod = CtxModifier::from_raw_with_item(*rmod, ship_key);
                    remove_cmod(
                        &mut self.cmods_root,
                        &(fit_key, LocationKind::Ship),
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
                        &(fit_key, LocationKind::Ship),
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
                        &(fit_key, LocationKind::Ship, a_item_grp_id),
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
                        &(fit_key, LocationKind::Ship, srq_a_item_id),
                        &cmod,
                        &mut self.cmods_by_aspec,
                    );
                }
                _ => (),
            };
        }
    }
}
