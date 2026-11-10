use crate::{
    rd::RItemListId,
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, Location, RawModId, RawModTemplates, RawModifier,
            modifier::CtxModifier,
            registers::{
                RawModStorage, RegOp,
                standard::{StandardRegister, modifier::func::process_cmod},
            },
        },
    },
    ud::{UData, UFitId, UFwEffect, UItemId, UShip},
};

impl StandardRegister {
    pub(in crate::svc::calc) fn process_fw_buff_mod<const OP: RegOp>(
        &mut self,
        reuse_cmods: &mut Vec<CtxModifier>,
        ctx: &SvcCtx,
        fw_effect: &UFwEffect,
        rmod_id: RawModId,
        rmod: RawModifier,
    ) -> bool {
        let valid = match rmod.affectee_filter {
            AffecteeFilter::Direct(Location::ItemList(item_list_rid)) => {
                let fit_uid = fw_effect.get_fit_uid();
                let affectee_uids = self.affectee_buffable.get(&(fit_uid, item_list_rid));
                reuse_cmods.reserve(affectee_uids.len());
                for &affectee_uid in affectee_uids {
                    let cmod = CtxModifier::with_item(rmod, affectee_uid);
                    let key = (affectee_uid, cmod.rmod.affectee_attr_rid);
                    process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.direct, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            AffecteeFilter::Loc(Location::ItemList(item_list_rid)) => {
                let fit_uid = fw_effect.get_fit_uid();
                if let Some((ship_uid, ship)) = is_fit_ship_on_proj_item_list(ctx.u_data, fit_uid, item_list_rid)
                    && let Ok(loc_kind) = ship.get_ship_kind().try_into()
                {
                    let cmod = CtxModifier::with_fit_item(rmod, fit_uid, ship_uid);
                    let key = (fit_uid, loc_kind, cmod.rmod.affectee_attr_rid);
                    process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.loc, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            AffecteeFilter::LocGrp(Location::ItemList(item_list_rid), item_grp_id) => {
                let fit_uid = fw_effect.get_fit_uid();
                if let Some((ship_uid, ship)) = is_fit_ship_on_proj_item_list(ctx.u_data, fit_uid, item_list_rid)
                    && let Ok(loc_kind) = ship.get_ship_kind().try_into()
                {
                    let cmod = CtxModifier::with_fit_item(rmod, fit_uid, ship_uid);
                    let key = (fit_uid, loc_kind, item_grp_id, cmod.rmod.affectee_attr_rid);
                    process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.loc_grp, &mut self.cmods.by_aspec);
                    reuse_cmods.push(cmod);
                }
                true
            }
            AffecteeFilter::LocSrq(Location::ItemList(item_list_rid), srq_type_aid) => {
                let fit_uid = fw_effect.get_fit_uid();
                if let Some((ship_uid, ship)) = is_fit_ship_on_proj_item_list(ctx.u_data, fit_uid, item_list_rid)
                    && let Ok(loc_kind) = ship.get_ship_kind().try_into()
                {
                    let cmod = CtxModifier::with_fit_item(rmod, fit_uid, ship_uid);
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
                RegOp::Reg => self.rmods_fw_buff.add_entry(fw_effect.get_fit_uid(), rmod_id),
                RegOp::Unreg => self.rmods_fw_buff.remove_entry(fw_effect.get_fit_uid(), &rmod_id),
            }
        }
        valid
    }
    pub(in crate::svc::calc::registers::standard) fn process_affectee_for_fw_buff<const OP: RegOp>(
        &mut self,
        templates: &RawModTemplates,
        rmods: &RawModStorage,
        item_uid: UItemId,
        ship: Option<&UShip>,
        fit_uid: UFitId,
        proj_buff_item_lists: &[RItemListId],
    ) {
        // Direct changes can affect all buffable items
        for &rmod_id in self.rmods_fw_buff.get(&fit_uid) {
            let rmod = rmods.get_rmod(templates, rmod_id);
            if let AffecteeFilter::Direct(Location::ItemList(item_list_rid)) = rmod.affectee_filter
                && proj_buff_item_lists.contains(&item_list_rid)
            {
                let cmod = CtxModifier::with_item(rmod, item_uid);
                let key = (item_uid, cmod.rmod.affectee_attr_rid);
                process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.direct, &mut self.cmods.by_aspec);
            }
        }
        // Indirect changes can be applied only via ships
        let Some(ship) = ship else {
            return;
        };
        let Ok(loc_kind) = ship.get_ship_kind().try_into() else {
            return;
        };
        for &rmod_id in self.rmods_fw_buff.get(&fit_uid) {
            let rmod = rmods.get_rmod(templates, rmod_id);
            match rmod.affectee_filter {
                AffecteeFilter::Loc(Location::ItemList(item_list_rid))
                    if proj_buff_item_lists.contains(&item_list_rid) =>
                {
                    let cmod = CtxModifier::with_fit_item(rmod, fit_uid, item_uid);
                    let key = (fit_uid, loc_kind, cmod.rmod.affectee_attr_rid);
                    process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.loc, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::LocGrp(Location::ItemList(item_list_rid), item_grp_id)
                    if proj_buff_item_lists.contains(&item_list_rid) =>
                {
                    let cmod = CtxModifier::with_fit_item(rmod, fit_uid, item_uid);
                    let key = (fit_uid, loc_kind, item_grp_id, cmod.rmod.affectee_attr_rid);
                    process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.loc_grp, &mut self.cmods.by_aspec);
                }
                AffecteeFilter::LocSrq(Location::ItemList(item_list_rid), srq_type_aid)
                    if proj_buff_item_lists.contains(&item_list_rid) =>
                {
                    let cmod = CtxModifier::with_fit_item(rmod, fit_uid, item_uid);
                    let key = (fit_uid, loc_kind, srq_type_aid, cmod.rmod.affectee_attr_rid);
                    process_cmod::<OP, _>(key, rmod_id, &cmod, &mut self.cmods.loc_srq, &mut self.cmods.by_aspec);
                }
                _ => (),
            };
        }
    }
}

fn is_fit_ship_on_proj_item_list(
    u_data: &UData,
    fit_uid: UFitId,
    item_list_rid: RItemListId,
) -> Option<(UItemId, &UShip)> {
    let fit = u_data.fits.get(fit_uid);
    let ship_uid = fit.ship?;
    let ship = u_data.items.get(ship_uid).dc_ship().unwrap();
    match ship
        .get_r_item_base()?
        .proj_buff_item_list_rids
        .contains(&item_list_rid)
    {
        true => Some((ship_uid, ship)),
        false => None,
    }
}
