use crate::{
    rd::RItemListId,
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, CtxModifier, Location, RawModId, RawModTemplates, RawModifier,
            registers::{
                RawModStorage, RegOp,
                standard::{
                    StandardRegister,
                    modifier::func::{add_cmod, process_cmod, remove_cmod},
                },
            },
        },
    },
    ud::{UItemId, UShip},
};

impl StandardRegister {
    pub(in crate::svc::calc) fn process_sw_buff_mod<const OP: RegOp>(
        &mut self,
        reuse_cmods: &mut Vec<CtxModifier>,
        ctx: &SvcCtx,
        rmod_id: RawModId,
        rmod: RawModifier,
    ) -> bool {
        let valid = match rmod.affectee_filter {
            AffecteeFilter::Direct(Location::ItemList(item_list_rid)) => {
                for fit_uid in ctx.u_data.fits.keys() {
                    let affectee_uids = self.affectee_buffable.get(&(fit_uid, item_list_rid));
                    reuse_cmods.reserve(affectee_uids.len());
                    for &affectee_uid in affectee_uids {
                        let cmod = CtxModifier::with_item(rmod, affectee_uid);
                        let key = (affectee_uid, cmod.rmod.affectee_attr_rid);
                        process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.direct, &mut self.cmods.by_aspec);
                        reuse_cmods.push(cmod);
                    }
                }
                true
            }
            AffecteeFilter::Loc(Location::ItemList(item_list_rid)) => {
                let affectee_uids = self.affectee_buffable_ships.get(&item_list_rid);
                reuse_cmods.reserve(affectee_uids.len());
                for &(fit_uid, item_uid, loc_kind) in affectee_uids {
                    let cmod = CtxModifier::with_fit_item(rmod, fit_uid, item_uid);
                    let key = (fit_uid, loc_kind, cmod.rmod.affectee_attr_rid);
                    process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.loc, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            AffecteeFilter::LocGrp(Location::ItemList(item_list_rid), item_grp_id) => {
                let affectee_uids = self.affectee_buffable_ships.get(&item_list_rid);
                reuse_cmods.reserve(affectee_uids.len());
                for &(fit_uid, item_uid, loc_kind) in affectee_uids {
                    let cmod = CtxModifier::with_fit_item(rmod, fit_uid, item_uid);
                    let key = (fit_uid, loc_kind, item_grp_id, cmod.rmod.affectee_attr_rid);
                    process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.loc_grp, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            AffecteeFilter::LocSrq(Location::ItemList(item_list_rid), srq_type_aid) => {
                let affectee_uids = self.affectee_buffable_ships.get(&item_list_rid);
                reuse_cmods.reserve(affectee_uids.len());
                for &(fit_uid, item_uid, loc_kind) in affectee_uids {
                    let cmod = CtxModifier::with_fit_item(rmod, fit_uid, item_uid);
                    let key = (fit_uid, loc_kind, srq_type_aid, cmod.rmod.affectee_attr_rid);
                    process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.loc_srq, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            _ => false,
        };
        if valid {
            match OP {
                RegOp::Reg => self.rmods_sw_buff.insert(rmod_id),
                RegOp::Unreg => self.rmods_sw_buff.remove(&rmod_id),
            };
        }
        valid
    }
    pub(in crate::svc::calc::registers::standard) fn reg_affectee_for_sw_buff(
        &mut self,
        templates: &RawModTemplates,
        rmods: &RawModStorage,
        item_uid: UItemId,
        ship: Option<&UShip>,
        proj_buff_item_lists: &[RItemListId],
    ) {
        // Direct changes can affect all buffable items
        for &rmod_id in self.rmods_sw_buff.iter() {
            let rmod = rmods.get_rmod(templates, rmod_id);
            if let AffecteeFilter::Direct(Location::ItemList(item_list_rid)) = rmod.affectee_filter
                && proj_buff_item_lists.contains(&item_list_rid)
            {
                let cmod = CtxModifier::with_item(rmod, item_uid);
                let key = (item_uid, cmod.rmod.affectee_attr_rid);
                add_cmod(key, rmod_id, &cmod, &mut self.cmods.direct, &mut self.cmods.by_aspec);
            }
        }
        // Indirect changes can be applied only via ships
        let (fit_uid, loc_kind) = match ship {
            Some(ship) => match ship.get_ship_kind().try_into() {
                Ok(loc_kind) => (ship.get_fit_uid(), loc_kind),
                Err(..) => return,
            },
            None => return,
        };
        for &rmod_id in self.rmods_sw_buff.iter() {
            let rmod = rmods.get_rmod(templates, rmod_id);
            match rmod.affectee_filter {
                AffecteeFilter::Loc(Location::ItemList(item_list_rid))
                    if proj_buff_item_lists.contains(&item_list_rid) =>
                {
                    let cmod = CtxModifier::with_fit_item(rmod, fit_uid, item_uid);
                    let key = (fit_uid, loc_kind, cmod.rmod.affectee_attr_rid);
                    add_cmod(key, rmod_id, &cmod, &mut self.cmods.loc, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::LocGrp(Location::ItemList(item_list_rid), item_grp_id)
                    if proj_buff_item_lists.contains(&item_list_rid) =>
                {
                    let cmod = CtxModifier::with_fit_item(rmod, fit_uid, item_uid);
                    let key = (fit_uid, loc_kind, item_grp_id, cmod.rmod.affectee_attr_rid);
                    add_cmod(key, rmod_id, &cmod, &mut self.cmods.loc_grp, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::LocSrq(Location::ItemList(item_list_rid), srq_type_aid)
                    if proj_buff_item_lists.contains(&item_list_rid) =>
                {
                    let cmod = CtxModifier::with_fit_item(rmod, fit_uid, item_uid);
                    let key = (fit_uid, loc_kind, srq_type_aid, cmod.rmod.affectee_attr_rid);
                    add_cmod(key, rmod_id, &cmod, &mut self.cmods.loc_srq, &mut self.cmods.by_aspec);
                }
                _ => (),
            };
        }
    }
    pub(in crate::svc::calc::registers::standard) fn unreg_affectee_for_sw_buff(
        &mut self,
        templates: &RawModTemplates,
        rmods: &RawModStorage,
        item_uid: UItemId,
        ship: Option<&UShip>,
        proj_buff_item_lists: &[RItemListId],
    ) {
        // Direct changes can affect all buffable items
        for &rmod_id in self.rmods_sw_buff.iter() {
            let rmod = rmods.get_rmod(templates, rmod_id);
            if let AffecteeFilter::Direct(Location::ItemList(item_list_rid)) = rmod.affectee_filter
                && proj_buff_item_lists.contains(&item_list_rid)
            {
                let cmod = CtxModifier::with_item(rmod, item_uid);
                let key = (item_uid, cmod.rmod.affectee_attr_rid);
                remove_cmod(key, rmod_id, &cmod, &mut self.cmods.direct, &mut self.cmods.by_aspec);
            }
        }
        // Indirect changes can be applied only via ships
        let (fit_uid, loc_kind) = match ship {
            Some(ship) => match ship.get_ship_kind().try_into() {
                Ok(loc_kind) => (ship.get_fit_uid(), loc_kind),
                Err(..) => return,
            },
            None => return,
        };
        for &rmod_id in self.rmods_sw_buff.iter() {
            let rmod = rmods.get_rmod(templates, rmod_id);
            match rmod.affectee_filter {
                AffecteeFilter::Loc(Location::ItemList(item_list_rid))
                    if proj_buff_item_lists.contains(&item_list_rid) =>
                {
                    let cmod = CtxModifier::with_fit_item(rmod, fit_uid, item_uid);
                    let key = (fit_uid, loc_kind, cmod.rmod.affectee_attr_rid);
                    remove_cmod(key, rmod_id, &cmod, &mut self.cmods.loc, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::LocGrp(Location::ItemList(item_list_rid), item_grp_id)
                    if proj_buff_item_lists.contains(&item_list_rid) =>
                {
                    let cmod = CtxModifier::with_fit_item(rmod, fit_uid, item_uid);
                    let key = (fit_uid, loc_kind, item_grp_id, cmod.rmod.affectee_attr_rid);
                    remove_cmod(key, rmod_id, &cmod, &mut self.cmods.loc_grp, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::LocSrq(Location::ItemList(item_list_rid), srq_type_aid)
                    if proj_buff_item_lists.contains(&item_list_rid) =>
                {
                    let cmod = CtxModifier::with_fit_item(rmod, fit_uid, item_uid);
                    let key = (fit_uid, loc_kind, srq_type_aid, cmod.rmod.affectee_attr_rid);
                    remove_cmod(key, rmod_id, &cmod, &mut self.cmods.loc_srq, &mut self.cmods.by_aspec);
                }
                _ => (),
            };
        }
    }
}
