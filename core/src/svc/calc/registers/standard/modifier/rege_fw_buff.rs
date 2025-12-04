use crate::{
    ad::AItemListId,
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, Location, RawModifier,
            modifier::CtxModifier,
            registers::standard::{
                StandardRegister,
                modifier::func::{add_cmod, remove_cmod},
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
                    let cmod = CtxModifier::new_with_item(rmod, affectee_key);
                    add_cmod(&mut self.cmods.direct, affectee_key, cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            AffecteeFilter::Loc(Location::ItemList(item_list_id)) => {
                let fit_key = fw_effect.get_fit_key();
                if let Some((ship_key, ship)) = is_fit_ship_on_proj_item_list(ctx, fit_key, &item_list_id)
                    && let Ok(loc_kind) = ship.get_kind().try_into()
                {
                    let cmod = CtxModifier::new_with_fit_item(rmod, fit_key, ship_key);
                    let key = (fit_key, loc_kind);
                    add_cmod(&mut self.cmods.loc, key, cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            AffecteeFilter::LocGrp(Location::ItemList(item_list_id), item_grp_id) => {
                let fit_key = fw_effect.get_fit_key();
                if let Some((ship_key, ship)) = is_fit_ship_on_proj_item_list(ctx, fit_key, &item_list_id)
                    && let Ok(loc_kind) = ship.get_kind().try_into()
                {
                    let cmod = CtxModifier::new_with_fit_item(rmod, fit_key, ship_key);
                    let key = (fit_key, loc_kind, item_grp_id);
                    add_cmod(&mut self.cmods.loc_grp, key, cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_type_id) => {
                let fit_key = fw_effect.get_fit_key();
                if let Some((ship_key, ship)) = is_fit_ship_on_proj_item_list(ctx, fit_key, &item_list_id)
                    && let Ok(loc_kind) = ship.get_kind().try_into()
                {
                    let cmod = CtxModifier::new_with_fit_item(rmod, fit_key, ship_key);
                    let key = (fit_key, loc_kind, srq_type_id);
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
                    let cmod = CtxModifier::new_with_item(rmod, affectee_key);
                    remove_cmod(&mut self.cmods.direct, affectee_key, &cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
            }
            AffecteeFilter::Loc(Location::ItemList(item_list_id)) => {
                let fit_key = fw_effect.get_fit_key();
                if let Some((ship_key, ship)) = is_fit_ship_on_proj_item_list(ctx, fit_key, &item_list_id)
                    && let Ok(loc_kind) = ship.get_kind().try_into()
                {
                    let cmod = CtxModifier::new_with_fit_item(rmod, fit_key, ship_key);
                    let key = (fit_key, loc_kind);
                    remove_cmod(&mut self.cmods.loc, key, &cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
            }
            AffecteeFilter::LocGrp(Location::ItemList(item_list_id), item_grp_id) => {
                let fit_key = fw_effect.get_fit_key();
                if let Some((ship_key, ship)) = is_fit_ship_on_proj_item_list(ctx, fit_key, &item_list_id)
                    && let Ok(loc_kind) = ship.get_kind().try_into()
                {
                    let cmod = CtxModifier::new_with_fit_item(rmod, fit_key, ship_key);
                    let key = (fit_key, loc_kind, item_grp_id);
                    remove_cmod(&mut self.cmods.loc_grp, key, &cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
            }
            AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_type_id) => {
                let fit_key = fw_effect.get_fit_key();
                if let Some((ship_key, ship)) = is_fit_ship_on_proj_item_list(ctx, fit_key, &item_list_id)
                    && let Ok(loc_kind) = ship.get_kind().try_into()
                {
                    let cmod = CtxModifier::new_with_fit_item(rmod, fit_key, ship_key);
                    let key = (fit_key, loc_kind, srq_type_id);
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
        ship: Option<&UShip>,
        fit_key: UFitKey,
        proj_buff_item_lists: &[AItemListId],
    ) {
        // Direct changes can affect all buffable items
        for rmod in self.rmods_fw_buff.get(&fit_key) {
            if let AffecteeFilter::Direct(Location::ItemList(item_list_id)) = rmod.affectee_filter
                && proj_buff_item_lists.contains(&item_list_id)
            {
                let cmod = CtxModifier::new_with_item(*rmod, item_key);
                add_cmod(&mut self.cmods.direct, item_key, cmod, &mut self.cmods.by_aspec);
            }
        }
        // Indirect changes can be applied only via ships
        let loc_kind = match ship {
            Some(ship) => match ship.get_kind().try_into() {
                Ok(loc_kind) => loc_kind,
                Err(_) => return,
            },
            None => return,
        };
        for rmod in self.rmods_fw_buff.get(&fit_key) {
            match rmod.affectee_filter {
                AffecteeFilter::Loc(Location::ItemList(item_list_id))
                    if proj_buff_item_lists.contains(&item_list_id) =>
                {
                    let cmod = CtxModifier::new_with_fit_item(*rmod, fit_key, item_key);
                    let key = (fit_key, loc_kind);
                    add_cmod(&mut self.cmods.loc, key, cmod, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::LocGrp(Location::ItemList(item_list_id), item_grp_id)
                    if proj_buff_item_lists.contains(&item_list_id) =>
                {
                    let cmod = CtxModifier::new_with_fit_item(*rmod, fit_key, item_key);
                    let key = (fit_key, loc_kind, item_grp_id);
                    add_cmod(&mut self.cmods.loc_grp, key, cmod, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_type_id)
                    if proj_buff_item_lists.contains(&item_list_id) =>
                {
                    let cmod = CtxModifier::new_with_fit_item(*rmod, fit_key, item_key);
                    let key = (fit_key, loc_kind, srq_type_id);
                    add_cmod(&mut self.cmods.loc_srq, key, cmod, &mut self.cmods.by_aspec);
                }
                _ => (),
            };
        }
    }
    pub(in crate::svc::calc::registers::standard) fn unreg_affectee_for_fw_buff(
        &mut self,
        item_key: UItemKey,
        ship: Option<&UShip>,
        fit_key: UFitKey,
        proj_buff_item_lists: &[AItemListId],
    ) {
        // Direct changes can affect all buffable items
        for rmod in self.rmods_fw_buff.get(&fit_key) {
            if let AffecteeFilter::Direct(Location::ItemList(item_list_id)) = rmod.affectee_filter
                && proj_buff_item_lists.contains(&item_list_id)
            {
                let cmod = CtxModifier::new_with_item(*rmod, item_key);
                remove_cmod(&mut self.cmods.direct, item_key, &cmod, &mut self.cmods.by_aspec);
            }
        }
        // Indirect changes can be applied only via ships
        let loc_kind = match ship {
            Some(ship) => match ship.get_kind().try_into() {
                Ok(loc_kind) => loc_kind,
                Err(_) => return,
            },
            None => return,
        };
        for rmod in self.rmods_fw_buff.get(&fit_key) {
            match rmod.affectee_filter {
                AffecteeFilter::Loc(Location::ItemList(item_list_id))
                    if proj_buff_item_lists.contains(&item_list_id) =>
                {
                    let cmod = CtxModifier::new_with_fit_item(*rmod, fit_key, item_key);
                    let key = (fit_key, loc_kind);
                    remove_cmod(&mut self.cmods.loc, key, &cmod, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::LocGrp(Location::ItemList(item_list_id), item_grp_id)
                    if proj_buff_item_lists.contains(&item_list_id) =>
                {
                    let cmod = CtxModifier::new_with_fit_item(*rmod, fit_key, item_key);
                    let key = (fit_key, loc_kind, item_grp_id);
                    remove_cmod(&mut self.cmods.loc_grp, key, &cmod, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_type_id)
                    if proj_buff_item_lists.contains(&item_list_id) =>
                {
                    let cmod = CtxModifier::new_with_fit_item(*rmod, fit_key, item_key);
                    let key = (fit_key, loc_kind, srq_type_id);
                    remove_cmod(&mut self.cmods.loc_srq, key, &cmod, &mut self.cmods.by_aspec);
                }
                _ => (),
            };
        }
    }
}

fn is_fit_ship_on_proj_item_list<'u>(
    ctx: SvcCtx<'u, '_>,
    fit_key: UFitKey,
    item_list_id: &AItemListId,
) -> Option<(UItemKey, &'u UShip)> {
    let fit = ctx.u_data.fits.get(fit_key);
    let ship_key = fit.ship?;
    let ship = ctx.u_data.items.get(ship_key).dc_ship().unwrap();
    match ship.get_proj_buff_item_lists()?.contains(item_list_id) {
        true => Some((ship_key, ship)),
        false => None,
    }
}
