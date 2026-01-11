use crate::{
    svc::{
        SvcCtx,
        calc::{
            CtxModifier,
            registers::standard::{data::StandardRegister, iter_locs_pot::PotentialLocations},
        },
    },
    ud::{UItem, UItemId},
};

impl StandardRegister {
    // Modification methods
    pub(in crate::svc::calc) fn reg_affectee(
        &mut self,
        ctx: SvcCtx,
        item_uid: UItemId,
        item: &UItem,
    ) -> Vec<CtxModifier> {
        // Let existing projections know their projectee got updated
        self.load_affectee_for_proj(item_uid, item);
        let mut cmods = Vec::new();
        // Past this point we process data only for fit-related items
        let fit_uid = match item.get_fit_uid() {
            Some(fit_uid) => fit_uid,
            None => return cmods,
        };
        let root_loc_kind = item.get_root_loc_kind();
        let item_grp_id = item.get_group_id().unwrap();
        let srqs = item.get_skill_reqs().unwrap();
        if let Some(root_loc_kind) = root_loc_kind {
            self.affectee_root.add_entry((fit_uid, root_loc_kind), item_uid);
        }
        for loc_kind in PotentialLocations::new(item) {
            self.affectee_loc.add_entry((fit_uid, loc_kind), item_uid);
            self.affectee_loc_grp
                .add_entry((fit_uid, loc_kind, item_grp_id), item_uid);
            for &srq_type_aid in srqs.keys() {
                self.affectee_loc_srq
                    .add_entry((fit_uid, loc_kind, srq_type_aid), item_uid);
            }
        }
        if item.is_owner_modifiable() {
            for &srq_type_aid in srqs.keys() {
                self.affectee_own_srq.add_entry((fit_uid, srq_type_aid), item_uid);
            }
        }
        // Buff-related processing
        if let Some(item_list_rids) = item.get_proj_buff_item_lists()
            && !item_list_rids.is_empty()
        {
            for &item_list_rid in item_list_rids {
                self.affectee_buffable.add_entry((fit_uid, item_list_rid), item_uid);
            }
            let ship = match item {
                UItem::Ship(ship) if let Ok(loc_kind) = ship.get_kind().try_into() => {
                    for &item_list_rid in item_list_rids {
                        self.affectee_buffable_ships
                            .add_entry(item_list_rid, (ship.get_fit_uid(), item_uid, loc_kind));
                    }
                    Some(ship)
                }
                _ => None,
            };
            self.reg_affectee_for_sw_buff(item_uid, ship, item_list_rids);
            self.reg_affectee_for_fw_buff(item_uid, ship, fit_uid, item_list_rids);
        }
        if let UItem::Ship(ship) = item {
            self.load_affectee_for_fleet(ctx, item_uid, ship);
            // If it's ship being unregistered, adding it might trigger attribute changes on various
            // items like modules. Valid list of modifiers can be fetched only with ship in place,
            // so do it after everything is processed
            self.get_mods_for_changed_ship(item, &mut cmods);
        }
        cmods
    }
    pub(in crate::svc::calc) fn unreg_affectee(
        &mut self,
        ctx: SvcCtx,
        item_uid: UItemId,
        item: &UItem,
    ) -> Vec<CtxModifier> {
        let mut cmods = Vec::new();
        if let UItem::Ship(ship) = item {
            // If it's ship being unregistered, removing it might trigger attribute changes on
            // various items like modules. Valid list of modifiers can be fetched only with ship in
            // place, so do it before anything is processed
            self.get_mods_for_changed_ship(item, &mut cmods);
            self.unload_affectee_for_fleet(ctx, item_uid, ship);
        }
        // Let existing projections know their projectee got updated
        self.unload_affectee_for_proj(item_uid, item);
        // Past this point we process data only for fit-related items
        let fit_uid = match item.get_fit_uid() {
            Some(fit_uid) => fit_uid,
            None => return cmods,
        };
        let root_loc_kind = item.get_root_loc_kind();
        let item_grp_id = item.get_group_id().unwrap();
        let srqs = item.get_skill_reqs().unwrap();
        if let Some(root_loc_kind) = root_loc_kind {
            self.affectee_root.remove_entry((fit_uid, root_loc_kind), &item_uid);
        }
        for loc_kind in PotentialLocations::new(item) {
            self.affectee_loc.remove_entry((fit_uid, loc_kind), &item_uid);
            self.affectee_loc_grp
                .remove_entry((fit_uid, loc_kind, item_grp_id), &item_uid);
            for &srq_type_aid in srqs.keys() {
                self.affectee_loc_srq
                    .remove_entry((fit_uid, loc_kind, srq_type_aid), &item_uid);
            }
        }
        if item.is_owner_modifiable() {
            for &srq_type_aid in srqs.keys() {
                self.affectee_own_srq.remove_entry((fit_uid, srq_type_aid), &item_uid);
            }
        }
        // Buff-related processing
        if let Some(item_list_rids) = item.get_proj_buff_item_lists()
            && !item_list_rids.is_empty()
        {
            for &item_list_rid in item_list_rids {
                self.affectee_buffable.remove_entry((fit_uid, item_list_rid), &item_uid);
            }
            let ship = match item {
                UItem::Ship(ship) if let Ok(loc_kind) = ship.get_kind().try_into() => {
                    for &item_list_rid in item_list_rids {
                        self.affectee_buffable_ships
                            .remove_entry(item_list_rid, &(ship.get_fit_uid(), item_uid, loc_kind));
                    }
                    Some(ship)
                }
                _ => None,
            };
            self.unreg_affectee_for_sw_buff(item_uid, ship, item_list_rids);
            self.unreg_affectee_for_fw_buff(item_uid, ship, fit_uid, item_list_rids);
        }
        cmods
    }
    fn get_mods_for_changed_ship(&self, item: &UItem, cmods: &mut Vec<CtxModifier>) {
        if let (Some(item_fit_uid), Some(loc_kind)) = (item.get_fit_uid(), item.get_ship_loc_kind()) {
            cmods.extend(self.cmods.loc.get(&(item_fit_uid, loc_kind)));
            for ((stored_fit_uid, stored_loc_kind, _), stored_cmods) in self.cmods.loc_grp.iter() {
                if item_fit_uid == *stored_fit_uid && loc_kind == *stored_loc_kind {
                    cmods.extend(stored_cmods);
                }
            }
            for ((stored_fit_uid, stored_loc_kind, _), stored_cmods) in self.cmods.loc_srq.iter() {
                if item_fit_uid == *stored_fit_uid && loc_kind == *stored_loc_kind {
                    cmods.extend(stored_cmods);
                }
            }
        }
    }
}
