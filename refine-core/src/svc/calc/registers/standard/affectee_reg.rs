use crate::{
    svc::{
        SvcCtx,
        calc::{
            CtxModifier, RawModTemplates,
            registers::{
                RawModStorage, RegOp, process_entry,
                standard::{data::StandardRegister, iter_locs_pot::PotentialLocations},
            },
        },
    },
    ud::{UItem, UItemId},
};

impl StandardRegister {
    // Modification methods
    pub(in crate::svc::calc) fn process_affectee<const OP: RegOp>(
        &mut self,
        reuse_cmods: &mut Vec<CtxModifier>,
        ctx: &SvcCtx,
        rmods: &RawModStorage,
        item_uid: UItemId,
        item: &UItem,
    ) {
        if OP == RegOp::Unreg
            && let UItem::Ship(ship) = item
        {
            // If it's ship being unregistered, removing it might trigger attribute changes on
            // various items like modules. Valid list of modifiers can be fetched only with ship in
            // place, so do it before anything is processed
            self.get_mods_for_changed_ship(reuse_cmods, ctx.rmt(), rmods, item);
            self.process_affectee_for_fleet::<OP>(ctx, rmods, item_uid, ship);
        }
        // Let existing projections know their projectee got updated
        self.process_affectee_for_proj::<OP>(ctx.rmt(), rmods, item_uid, item);
        // Past this point we process data only for fit-related items
        let Some(fit_uid) = item.get_fit_uid() else {
            return;
        };
        let item_rib = item.get_r_item_base().unwrap();
        let item_grp_id = item_rib.grp_id;
        let srqs = &item_rib.srqs;
        let root_loc_kind = item.get_root_loc_kind();
        if let Some(root_loc_kind) = root_loc_kind {
            process_entry::<OP, _, _>(&mut self.affectee_root, (fit_uid, root_loc_kind), item_uid);
        }
        for loc_kind in PotentialLocations::new(item) {
            process_entry::<OP, _, _>(&mut self.affectee_loc, (fit_uid, loc_kind), item_uid);
            process_entry::<OP, _, _>(&mut self.affectee_loc_grp, (fit_uid, loc_kind, item_grp_id), item_uid);
            for &srq_type_aid in srqs.keys() {
                process_entry::<OP, _, _>(&mut self.affectee_loc_srq, (fit_uid, loc_kind, srq_type_aid), item_uid);
            }
        }
        if item.is_owner_modifiable() {
            for &srq_type_aid in srqs.keys() {
                process_entry::<OP, _, _>(&mut self.affectee_own_srq, (fit_uid, srq_type_aid), item_uid);
            }
        }
        // Buff-related processing
        if let Some(item_list_rids) = item.get_proj_buff_item_lists()
            && !item_list_rids.is_empty()
        {
            for &item_list_rid in item_list_rids {
                process_entry::<OP, _, _>(&mut self.affectee_buffable, (fit_uid, item_list_rid), item_uid);
            }
            let ship = match item {
                UItem::Ship(ship) if let Ok(loc_kind) = ship.get_ship_kind().try_into() => {
                    for &item_list_rid in item_list_rids {
                        process_entry::<OP, _, _>(
                            &mut self.affectee_buffable_ships,
                            item_list_rid,
                            (ship.get_fit_uid(), item_uid, loc_kind),
                        );
                    }
                    Some(ship)
                }
                _ => None,
            };
            self.process_affectee_for_sw_buff::<OP>(ctx.rmt(), rmods, item_uid, ship, item_list_rids);
            self.process_affectee_for_fw_buff::<OP>(ctx.rmt(), rmods, item_uid, ship, fit_uid, item_list_rids);
        }
        if OP == RegOp::Reg
            && let UItem::Ship(ship) = item
        {
            self.process_affectee_for_fleet::<OP>(ctx, rmods, item_uid, ship);
            // If it's ship being registered, adding it might trigger attribute changes on various
            // items like modules. Valid list of modifiers can be fetched only with ship in place,
            // so do it after everything is processed
            self.get_mods_for_changed_ship(reuse_cmods, ctx.rmt(), rmods, item);
        }
    }
    fn get_mods_for_changed_ship(
        &self,
        reuse_cmods: &mut Vec<CtxModifier>,
        templates: &RawModTemplates,
        rmods: &RawModStorage,
        item: &UItem,
    ) {
        if let (Some(item_fit_uid), Some(loc_kind)) = (item.get_fit_uid(), item.get_ship_loc_kind()) {
            for ((stored_fit_uid, stored_loc_kind, _), stored_cmods) in self.cmods.loc.iter() {
                if item_fit_uid == *stored_fit_uid && loc_kind == *stored_loc_kind {
                    reuse_cmods.extend(stored_cmods.map(|ckey| rmods.get_cmod(templates, ckey)));
                }
            }
            for ((stored_fit_uid, stored_loc_kind, _, _), stored_cmods) in self.cmods.loc_grp.iter() {
                if item_fit_uid == *stored_fit_uid && loc_kind == *stored_loc_kind {
                    reuse_cmods.extend(stored_cmods.map(|ckey| rmods.get_cmod(templates, ckey)));
                }
            }
            for ((stored_fit_uid, stored_loc_kind, _, _), stored_cmods) in self.cmods.loc_srq.iter() {
                if item_fit_uid == *stored_fit_uid && loc_kind == *stored_loc_kind {
                    reuse_cmods.extend(stored_cmods.map(|ckey| rmods.get_cmod(templates, ckey)));
                }
            }
        }
    }
}
