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
    ud::{UFitKey, UFwEffect, UItemKey},
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
                    let cmod = CtxModifier::new_with_projectee_item(rmod, affectee_key);
                    add_cmod(&mut self.cmods.direct, affectee_key, cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            AffecteeFilter::Loc(Location::ItemList(item_list_id)) => {
                let fit_key = fw_effect.get_fit_key();
                if let Some(ship_key) = is_fit_ship_on_item_list(ctx, fit_key, &item_list_id) {
                    let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, ship_key);
                    let key = (fit_key, LocationKind::Ship);
                    add_cmod(&mut self.cmods.loc, key, cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            AffecteeFilter::LocGrp(Location::ItemList(item_list_id), item_grp_id) => {
                let fit_key = fw_effect.get_fit_key();
                if let Some(ship_key) = is_fit_ship_on_item_list(ctx, fit_key, &item_list_id) {
                    let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, ship_key);
                    let key = (fit_key, LocationKind::Ship, item_grp_id);
                    add_cmod(&mut self.cmods.loc_grp, key, cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_type_id) => {
                let fit_key = fw_effect.get_fit_key();
                if let Some(ship_key) = is_fit_ship_on_item_list(ctx, fit_key, &item_list_id) {
                    let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, ship_key);
                    let key = (fit_key, LocationKind::Ship, srq_type_id);
                    add_cmod(&mut self.cmods.loc_srq, key, cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            _ => false,
        };
        if valid {
            self.rmods_fw_buff.add_entry(fw_effect.get_fit_key(), rmod);
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
                for &affectee_key in affectee_keys {
                    let cmod = CtxModifier::new_with_projectee_item(rmod, affectee_key);
                    remove_cmod(&mut self.cmods.direct, affectee_key, &cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
            }
            AffecteeFilter::Loc(Location::ItemList(item_list_id)) => {
                let fit_key = fw_effect.get_fit_key();
                if let Some(ship_key) = is_fit_ship_on_item_list(ctx, fit_key, &item_list_id) {
                    let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, ship_key);
                    let key = (fit_key, LocationKind::Ship);
                    remove_cmod(&mut self.cmods.loc, key, &cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
            }
            AffecteeFilter::LocGrp(Location::ItemList(item_list_id), item_grp_id) => {
                let fit_key = fw_effect.get_fit_key();
                if let Some(ship_key) = is_fit_ship_on_item_list(ctx, fit_key, &item_list_id) {
                    let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, ship_key);
                    let key = (fit_key, LocationKind::Ship, item_grp_id);
                    remove_cmod(&mut self.cmods.loc_grp, key, &cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
            }
            AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_type_id) => {
                let fit_key = fw_effect.get_fit_key();
                if let Some(ship_key) = is_fit_ship_on_item_list(ctx, fit_key, &item_list_id) {
                    let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, ship_key);
                    let key = (fit_key, LocationKind::Ship, srq_type_id);
                    remove_cmod(&mut self.cmods.loc_srq, key, &cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
            }
            _ => (),
        };
        // Only modifiers which passed validation during registration should reach this function, so
        // we don't do extra validation and just remove them
        self.rmods_fw_buff.remove_entry(fw_effect.get_fit_key(), &rmod);
    }
    pub(in crate::svc::calc::registers::standard) fn reg_affectee_for_fw_buff(
        &mut self,
        item_key: UItemKey,
        is_ship: bool,
        fit_key: UFitKey,
        buffable_item_lists: &[AItemListId],
    ) {
        // Direct changes can affect all buffable items
        for rmod in self.rmods_fw_buff.get(&fit_key) {
            if let AffecteeFilter::Direct(Location::ItemList(item_list_id)) = rmod.affectee_filter
                && buffable_item_lists.contains(&item_list_id)
            {
                let cmod = CtxModifier::new_with_projectee_item(*rmod, item_key);
                add_cmod(&mut self.cmods.direct, item_key, cmod, &mut self.cmods.by_aspec);
            }
        }
        // Indirect changes can be applied only via ships
        if !is_ship {
            return;
        }
        for rmod in self.rmods_fw_buff.get(&fit_key) {
            match rmod.affectee_filter {
                AffecteeFilter::Loc(Location::ItemList(item_list_id))
                    if buffable_item_lists.contains(&item_list_id) =>
                {
                    let cmod = CtxModifier::new_with_projectee_fit_item(*rmod, fit_key, item_key);
                    let key = (fit_key, LocationKind::Ship);
                    add_cmod(&mut self.cmods.loc, key, cmod, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::LocGrp(Location::ItemList(item_list_id), item_grp_id)
                    if buffable_item_lists.contains(&item_list_id) =>
                {
                    let cmod = CtxModifier::new_with_projectee_fit_item(*rmod, fit_key, item_key);
                    let key = (fit_key, LocationKind::Ship, item_grp_id);
                    add_cmod(&mut self.cmods.loc_grp, key, cmod, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_type_id)
                    if buffable_item_lists.contains(&item_list_id) =>
                {
                    let cmod = CtxModifier::new_with_projectee_fit_item(*rmod, fit_key, item_key);
                    let key = (fit_key, LocationKind::Ship, srq_type_id);
                    add_cmod(&mut self.cmods.loc_srq, key, cmod, &mut self.cmods.by_aspec);
                }
                _ => (),
            };
        }
    }
    pub(in crate::svc::calc::registers::standard) fn unreg_affectee_for_fw_buff(
        &mut self,
        item_key: UItemKey,
        is_ship: bool,
        fit_key: UFitKey,
        buffable_item_lists: &[AItemListId],
    ) {
        // Direct changes can affect all buffable items
        for rmod in self.rmods_fw_buff.get(&fit_key) {
            if let AffecteeFilter::Direct(Location::ItemList(item_list_id)) = rmod.affectee_filter
                && buffable_item_lists.contains(&item_list_id)
            {
                let cmod = CtxModifier::new_with_projectee_item(*rmod, item_key);
                remove_cmod(&mut self.cmods.direct, item_key, &cmod, &mut self.cmods.by_aspec);
            }
        }
        // Indirect changes can be applied only via ships
        if !is_ship {
            return;
        }
        for rmod in self.rmods_fw_buff.get(&fit_key) {
            match rmod.affectee_filter {
                AffecteeFilter::Loc(Location::ItemList(item_list_id))
                    if buffable_item_lists.contains(&item_list_id) =>
                {
                    let cmod = CtxModifier::new_with_projectee_fit_item(*rmod, fit_key, item_key);
                    let key = (fit_key, LocationKind::Ship);
                    remove_cmod(&mut self.cmods.loc, key, &cmod, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::LocGrp(Location::ItemList(item_list_id), item_grp_id)
                    if buffable_item_lists.contains(&item_list_id) =>
                {
                    let cmod = CtxModifier::new_with_projectee_fit_item(*rmod, fit_key, item_key);
                    let key = (fit_key, LocationKind::Ship, item_grp_id);
                    remove_cmod(&mut self.cmods.loc_grp, key, &cmod, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_type_id)
                    if buffable_item_lists.contains(&item_list_id) =>
                {
                    let cmod = CtxModifier::new_with_projectee_fit_item(*rmod, fit_key, item_key);
                    let key = (fit_key, LocationKind::Ship, srq_type_id);
                    remove_cmod(&mut self.cmods.loc_srq, key, &cmod, &mut self.cmods.by_aspec);
                }
                _ => (),
            };
        }
    }
}
