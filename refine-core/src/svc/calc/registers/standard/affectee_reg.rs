use crate::{
    rd::RItemBase,
    svc::{
        SvcCtx,
        calc::{
            CtxModifier, LocationKind, RawModTemplates,
            registers::{RawModStorage, RegOp, process_entry, standard::data::StandardRegister},
        },
    },
    ud::{UData, UFitId, UItem, UItemId},
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
        let srqs = &item_rib.srqs;
        let root_loc_kind = item.get_root_loc_kind();
        if let Some(root_loc_kind) = root_loc_kind {
            process_entry::<OP, _, _>(&mut self.affectee_root, (fit_uid, root_loc_kind), item_uid);
        }
        if item.is_on_char_root() {
            self.process_affectee_loc::<OP>(fit_uid, LocationKind::Character, item_uid, item_rib);
        }
        // Ship and structure roots hold the same item kinds
        if item.is_on_ship_root() {
            match self.fit_ships_loc_kind.get(&fit_uid) {
                Some(&loc_kind) => self.process_affectee_loc::<OP>(fit_uid, loc_kind, item_uid, item_rib),
                None => process_entry::<OP, _, _>(&mut self.affectee_loc_parked, fit_uid, item_uid),
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
    // Called before a ship's registration, which collects modifiers to invalidate whose affectees
    // include the items registered here
    pub(in crate::svc::calc) fn activate_fit_loc(&mut self, u_data: &UData, fit_uid: UFitId, loc_kind: LocationKind) {
        self.fit_ships_loc_kind.insert(fit_uid, loc_kind);
        for item_uid in self.affectee_loc_parked.remove_key(&fit_uid) {
            let item_rib = u_data.items.get(item_uid).get_r_item_base().unwrap();
            self.process_affectee_loc::<{ RegOp::Reg }>(fit_uid, loc_kind, item_uid, item_rib);
        }
    }
    // Called after a ship's unregistration, once the affectees of the modifiers it collected are
    // invalidated
    pub(in crate::svc::calc) fn deactivate_fit_loc(&mut self, u_data: &UData, fit_uid: UFitId) {
        let Some(loc_kind) = self.fit_ships_loc_kind.remove(&fit_uid) else {
            return;
        };
        for item_uid in self.affectee_loc.remove_key(&(fit_uid, loc_kind)) {
            let item_rib = u_data.items.get(item_uid).get_r_item_base().unwrap();
            process_entry::<{ RegOp::Unreg }, _, _>(
                &mut self.affectee_loc_grp,
                (fit_uid, loc_kind, item_rib.grp_id),
                item_uid,
            );
            for &srq_type_aid in item_rib.srqs.keys() {
                process_entry::<{ RegOp::Unreg }, _, _>(
                    &mut self.affectee_loc_srq,
                    (fit_uid, loc_kind, srq_type_aid),
                    item_uid,
                );
            }
            self.affectee_loc_parked.add_entry(fit_uid, item_uid);
        }
    }
    fn process_affectee_loc<const OP: RegOp>(
        &mut self,
        fit_uid: UFitId,
        loc_kind: LocationKind,
        item_uid: UItemId,
        item_rib: &RItemBase,
    ) {
        process_entry::<OP, _, _>(&mut self.affectee_loc, (fit_uid, loc_kind), item_uid);
        process_entry::<OP, _, _>(
            &mut self.affectee_loc_grp,
            (fit_uid, loc_kind, item_rib.grp_id),
            item_uid,
        );
        for &srq_type_aid in item_rib.srqs.keys() {
            process_entry::<OP, _, _>(&mut self.affectee_loc_srq, (fit_uid, loc_kind, srq_type_aid), item_uid);
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
