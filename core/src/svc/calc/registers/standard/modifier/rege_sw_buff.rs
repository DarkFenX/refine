use crate::{
    rd::RItemListId,
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, CtxModifier, Location, RawModifier,
            registers::standard::{
                StandardRegister,
                modifier::func::{add_cmod, remove_cmod},
            },
        },
    },
    ud::{UItemId, UShip},
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
            AffecteeFilter::Direct(Location::ItemList(item_list_rid)) => {
                for fit_uid in ctx.u_data.fits.keys() {
                    let affectee_uids = self.affectee_buffable.get(&(fit_uid, item_list_rid));
                    reuse_cmods.reserve(affectee_uids.len());
                    for &affectee_uid in affectee_uids {
                        let cmod = CtxModifier::new_with_item(rmod, affectee_uid);
                        add_cmod(&mut self.cmods.direct, affectee_uid, cmod, &mut self.cmods.by_aspec);
                        reuse_cmods.push(cmod);
                    }
                }
                true
            }
            AffecteeFilter::Loc(Location::ItemList(item_list_rid)) => {
                let affectee_uids = self.affectee_buffable_ships.get(&item_list_rid);
                reuse_cmods.reserve(affectee_uids.len());
                for &(fit_uid, item_uid, loc_kind) in affectee_uids {
                    let cmod = CtxModifier::new_with_fit_item(rmod, fit_uid, item_uid);
                    let key = (fit_uid, loc_kind);
                    add_cmod(&mut self.cmods.loc, key, cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            AffecteeFilter::LocGrp(Location::ItemList(item_list_rid), item_grp_id) => {
                let affectee_uids = self.affectee_buffable_ships.get(&item_list_rid);
                reuse_cmods.reserve(affectee_uids.len());
                for &(fit_uid, item_uid, loc_kind) in affectee_uids {
                    let cmod = CtxModifier::new_with_fit_item(rmod, fit_uid, item_uid);
                    let key = (fit_uid, loc_kind, item_grp_id);
                    add_cmod(&mut self.cmods.loc_grp, key, cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            AffecteeFilter::LocSrq(Location::ItemList(item_list_rid), srq_type_id) => {
                let affectee_uids = self.affectee_buffable_ships.get(&item_list_rid);
                reuse_cmods.reserve(affectee_uids.len());
                for &(fit_uid, item_uid, loc_kind) in affectee_uids {
                    let cmod = CtxModifier::new_with_fit_item(rmod, fit_uid, item_uid);
                    let key = (fit_uid, loc_kind, srq_type_id);
                    add_cmod(&mut self.cmods.loc_srq, key, cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            _ => false,
        };
        if valid {
            self.rmods_sw_buff.insert(rmod);
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
            AffecteeFilter::Direct(Location::ItemList(item_list_rid)) => {
                for fit_uid in ctx.u_data.fits.keys() {
                    let affectee_uids = self.affectee_buffable.get(&(fit_uid, item_list_rid));
                    reuse_cmods.reserve(affectee_uids.len());
                    for &affectee_uid in affectee_uids {
                        let cmod = CtxModifier::new_with_item(*rmod, affectee_uid);
                        remove_cmod(&mut self.cmods.direct, affectee_uid, &cmod, &mut self.cmods.by_aspec);
                        reuse_cmods.push(cmod);
                    }
                }
            }
            AffecteeFilter::Loc(Location::ItemList(item_list_rid)) => {
                let affectee_uids = self.affectee_buffable_ships.get(&item_list_rid);
                reuse_cmods.reserve(affectee_uids.len());
                for &(fit_uid, item_uid, loc_kind) in affectee_uids {
                    let cmod = CtxModifier::new_with_fit_item(*rmod, fit_uid, item_uid);
                    let key = (fit_uid, loc_kind);
                    remove_cmod(&mut self.cmods.loc, key, &cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
            }
            AffecteeFilter::LocGrp(Location::ItemList(item_list_rid), item_grp_id) => {
                let affectee_uids = self.affectee_buffable_ships.get(&item_list_rid);
                reuse_cmods.reserve(affectee_uids.len());
                for &(fit_uid, item_uid, loc_kind) in affectee_uids {
                    let cmod = CtxModifier::new_with_fit_item(*rmod, fit_uid, item_uid);
                    let key = (fit_uid, loc_kind, item_grp_id);
                    remove_cmod(&mut self.cmods.loc_grp, key, &cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
            }
            AffecteeFilter::LocSrq(Location::ItemList(item_list_rid), srq_type_id) => {
                let affectee_uids = self.affectee_buffable_ships.get(&item_list_rid);
                reuse_cmods.reserve(affectee_uids.len());
                for &(fit_uid, item_uid, loc_kind) in affectee_uids {
                    let cmod = CtxModifier::new_with_fit_item(*rmod, fit_uid, item_uid);
                    let key = (fit_uid, loc_kind, srq_type_id);
                    remove_cmod(&mut self.cmods.loc_srq, key, &cmod, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
            }
            _ => (),
        }
        // Only modifiers which passed validation during registration should reach this function, so
        // we don't do extra validation and just remove them
        self.rmods_sw_buff.remove(rmod);
    }
    pub(in crate::svc::calc::registers::standard) fn reg_affectee_for_sw_buff(
        &mut self,
        item_uid: UItemId,
        ship: Option<&UShip>,
        proj_buff_item_lists: &[RItemListId],
    ) {
        // Direct changes can affect all buffable items
        for rmod in self.rmods_sw_buff.iter() {
            if let AffecteeFilter::Direct(Location::ItemList(item_list_rid)) = rmod.affectee_filter
                && proj_buff_item_lists.contains(&item_list_rid)
            {
                let cmod = CtxModifier::new_with_item(*rmod, item_uid);
                add_cmod(&mut self.cmods.direct, item_uid, cmod, &mut self.cmods.by_aspec);
            }
        }
        // Indirect changes can be applied only via ships
        let (fit_uid, loc_kind) = match ship {
            Some(ship) => match ship.get_kind().try_into() {
                Ok(loc_kind) => (ship.get_fit_uid(), loc_kind),
                Err(_) => return,
            },
            None => return,
        };
        for rmod in self.rmods_sw_buff.iter() {
            match rmod.affectee_filter {
                AffecteeFilter::Loc(Location::ItemList(item_list_rid))
                    if proj_buff_item_lists.contains(&item_list_rid) =>
                {
                    let cmod = CtxModifier::new_with_fit_item(*rmod, fit_uid, item_uid);
                    let key = (fit_uid, loc_kind);
                    add_cmod(&mut self.cmods.loc, key, cmod, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::LocGrp(Location::ItemList(item_list_rid), item_grp_id)
                    if proj_buff_item_lists.contains(&item_list_rid) =>
                {
                    let cmod = CtxModifier::new_with_fit_item(*rmod, fit_uid, item_uid);
                    let key = (fit_uid, loc_kind, item_grp_id);
                    add_cmod(&mut self.cmods.loc_grp, key, cmod, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::LocSrq(Location::ItemList(item_list_rid), srq_type_id)
                    if proj_buff_item_lists.contains(&item_list_rid) =>
                {
                    let cmod = CtxModifier::new_with_fit_item(*rmod, fit_uid, item_uid);
                    let key = (fit_uid, loc_kind, srq_type_id);
                    add_cmod(&mut self.cmods.loc_srq, key, cmod, &mut self.cmods.by_aspec);
                }
                _ => (),
            };
        }
    }
    pub(in crate::svc::calc::registers::standard) fn unreg_affectee_for_sw_buff(
        &mut self,
        item_uid: UItemId,
        ship: Option<&UShip>,
        proj_buff_item_lists: &[RItemListId],
    ) {
        // Direct changes can affect all buffable items
        for rmod in self.rmods_sw_buff.iter() {
            if let AffecteeFilter::Direct(Location::ItemList(item_list_rid)) = rmod.affectee_filter
                && proj_buff_item_lists.contains(&item_list_rid)
            {
                let cmod = CtxModifier::new_with_item(*rmod, item_uid);
                remove_cmod(&mut self.cmods.direct, item_uid, &cmod, &mut self.cmods.by_aspec);
            }
        }
        // Indirect changes can be applied only via ships
        let (fit_uid, loc_kind) = match ship {
            Some(ship) => match ship.get_kind().try_into() {
                Ok(loc_kind) => (ship.get_fit_uid(), loc_kind),
                Err(_) => return,
            },
            None => return,
        };
        for rmod in self.rmods_sw_buff.iter() {
            match rmod.affectee_filter {
                AffecteeFilter::Loc(Location::ItemList(item_list_rid))
                    if proj_buff_item_lists.contains(&item_list_rid) =>
                {
                    let cmod = CtxModifier::new_with_fit_item(*rmod, fit_uid, item_uid);
                    let key = (fit_uid, loc_kind);
                    remove_cmod(&mut self.cmods.loc, key, &cmod, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::LocGrp(Location::ItemList(item_list_rid), item_grp_id)
                    if proj_buff_item_lists.contains(&item_list_rid) =>
                {
                    let cmod = CtxModifier::new_with_fit_item(*rmod, fit_uid, item_uid);
                    let key = (fit_uid, loc_kind, item_grp_id);
                    remove_cmod(&mut self.cmods.loc_grp, key, &cmod, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::LocSrq(Location::ItemList(item_list_rid), srq_type_id)
                    if proj_buff_item_lists.contains(&item_list_rid) =>
                {
                    let cmod = CtxModifier::new_with_fit_item(*rmod, fit_uid, item_uid);
                    let key = (fit_uid, loc_kind, srq_type_id);
                    remove_cmod(&mut self.cmods.loc_srq, key, &cmod, &mut self.cmods.by_aspec);
                }
                _ => (),
            };
        }
    }
}
