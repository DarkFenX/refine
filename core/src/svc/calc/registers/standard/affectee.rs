use crate::{
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, Context, CtxModifier, Location, LocationKind, ModifierKind, RawModifier,
            registers::standard::{data::StandardRegister, iter_locs_pot::PotentialLocations},
        },
    },
    ud::{UFit, UFitKey, UItem, UItemKey, UShipKind},
    util::extend_vec_from_map_set_l1,
};

impl StandardRegister {
    // Query methods
    pub(in crate::svc::calc) fn fill_affectees(
        &self,
        reuse_affectees: &mut Vec<UItemKey>,
        ctx: SvcCtx,
        cmod: &CtxModifier,
    ) {
        // All the context modifiers passed to this method have to come from the standard register.
        // This way we can ensure context modifiers are valid, and make some of processing cheaper
        reuse_affectees.clear();
        match cmod.ctx {
            Context::None => self.fill_affectees_no_context(reuse_affectees, ctx, &cmod.raw),
            Context::Fit(fit_key) => self.fill_affectees_for_fit(reuse_affectees, ctx, &cmod.raw, fit_key),
            Context::Item(item_key) => match cmod.raw.kind {
                ModifierKind::System => self.fill_affectees_for_item_system(reuse_affectees, ctx, &cmod.raw, item_key),
                ModifierKind::Targeted => {
                    self.fill_affectees_for_item_target(reuse_affectees, ctx, &cmod.raw, item_key)
                }
                ModifierKind::Buff => self.fill_affectees_for_item_buff(reuse_affectees, ctx, &cmod.raw, item_key),
                _ => (),
            },
        }
    }
    // Modification methods
    pub(in crate::svc::calc) fn reg_affectee(&mut self, item_key: UItemKey, item: &UItem) -> Vec<CtxModifier> {
        // Let existing projections know their projectee got updated
        self.load_affectee_for_proj(item_key, item);
        let mut cmods = Vec::new();
        // Past this point we process data only for fit-related items
        let fit_key = match item.get_fit_key() {
            Some(fit_key) => fit_key,
            None => return cmods,
        };
        let root_loc = item.get_root_loc_kind();
        let item_grp_id = item.get_group_id().unwrap();
        let srqs = item.get_skill_reqs().unwrap();
        if let Some(root_loc) = root_loc {
            self.affectee_root.add_entry((fit_key, root_loc), item_key);
        }
        for loc in PotentialLocations::new(item) {
            self.affectee_loc.add_entry((fit_key, loc), item_key);
            self.affectee_loc_grp.add_entry((fit_key, loc, item_grp_id), item_key);
            for &srq_type_id in srqs.keys() {
                self.affectee_loc_srq.add_entry((fit_key, loc, srq_type_id), item_key);
            }
        }
        if item.is_owner_modifiable() {
            for &srq_type_id in srqs.keys() {
                self.affectee_own_srq.add_entry((fit_key, srq_type_id), item_key);
            }
        }
        // Buff-related processing
        if let Some(item_list_ids) = item.get_item_buff_item_lists()
            && !item_list_ids.is_empty()
        {
            for &item_list_id in item_list_ids {
                self.affectee_buffable.add_entry((fit_key, item_list_id), item_key);
            }
            let ship = match item {
                UItem::Ship(ship) => {
                    for &item_list_id in item_list_ids {
                        self.affectee_buffable_ships
                            .add_entry(item_list_id, (ship.get_fit_key(), item_key));
                    }
                    Some(ship)
                }
                _ => None,
            };
            self.reg_affectee_for_sw_buff(item_key, ship, item_list_ids);
            self.reg_affectee_for_fw_buff(item_key, ship.is_some(), fit_key, item_list_ids);
        }
        // If it's ship being unregistered, adding it might trigger attribute changes on various
        // items like modules. Valid list of modifiers can be fetched only with ship in place, so
        // do it after everything is processed
        if let UItem::Ship(_) = item {
            self.get_mods_for_changed_ship(item, &mut cmods);
        }
        cmods
    }
    pub(in crate::svc::calc) fn unreg_affectee(&mut self, item_key: UItemKey, item: &UItem) -> Vec<CtxModifier> {
        // If it's ship being unregistered, removing it might trigger attribute changes on various
        // items like modules. Valid list of modifiers can be fetched only with ship in place, so
        // do it before anything is processed
        let mut cmods = Vec::new();
        if let UItem::Ship(_) = item {
            self.get_mods_for_changed_ship(item, &mut cmods);
        }
        // Let existing projections know their projectee got updated
        self.unload_affectee_for_proj(item_key, item);
        // Past this point we process data only for fit-related items
        let fit_key = match item.get_fit_key() {
            Some(fit_key) => fit_key,
            None => return cmods,
        };
        let root_loc = item.get_root_loc_kind();
        let item_grp_id = item.get_group_id().unwrap();
        let srqs = item.get_skill_reqs().unwrap();
        if let Some(root_loc) = root_loc {
            self.affectee_root.remove_entry((fit_key, root_loc), &item_key);
        }
        for loc in PotentialLocations::new(item) {
            self.affectee_loc.remove_entry((fit_key, loc), &item_key);
            self.affectee_loc_grp
                .remove_entry((fit_key, loc, item_grp_id), &item_key);
            for &srq_type_id in srqs.keys() {
                self.affectee_loc_srq
                    .remove_entry((fit_key, loc, srq_type_id), &item_key);
            }
        }
        if item.is_owner_modifiable() {
            for &srq_type_id in srqs.keys() {
                self.affectee_own_srq.remove_entry((fit_key, srq_type_id), &item_key);
            }
        }
        // Buff-related processing
        if let Some(item_list_ids) = item.get_item_buff_item_lists()
            && !item_list_ids.is_empty()
        {
            for &item_list_id in item_list_ids {
                self.affectee_buffable.remove_entry((fit_key, item_list_id), &item_key);
            }
            let ship = match item {
                UItem::Ship(ship) => {
                    for &item_list_id in item_list_ids {
                        self.affectee_buffable_ships
                            .remove_entry(item_list_id, &(ship.get_fit_key(), item_key));
                    }
                    Some(ship)
                }
                _ => None,
            };
            self.unreg_affectee_for_sw_buff(item_key, ship, item_list_ids);
            self.unreg_affectee_for_fw_buff(item_key, ship.is_some(), fit_key, item_list_ids);
        }
        cmods
    }
    // Private methods
    fn fill_affectees_no_context(&self, affectees: &mut Vec<UItemKey>, ctx: SvcCtx, rmod: &RawModifier) {
        if let AffecteeFilter::Direct(loc) = rmod.affectee_filter {
            match loc {
                Location::Item => {
                    affectees.push(rmod.affector_espec.item_key);
                }
                Location::Other => {
                    let item = ctx.u_data.items.get(rmod.affector_espec.item_key);
                    if let Some(other_item_key) = item.get_other_key() {
                        affectees.push(other_item_key);
                    }
                }
                _ => (),
            }
        }
    }
    fn fill_affectees_for_fit(&self, affectees: &mut Vec<UItemKey>, ctx: SvcCtx, rmod: &RawModifier, fit_key: UFitKey) {
        // The only fit-context modifiers with item list filter are fleet buffs, and those are
        // hardcoded to use ship location during reg/unreg, follow that here as well
        match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) => {
                let loc_kind = match loc {
                    Location::ItemList(_) => LocationKind::Ship,
                    _ => match loc.try_into() {
                        Ok(loc_kind) => loc_kind,
                        _ => return,
                    },
                };
                let fit = ctx.u_data.fits.get(fit_key);
                if check_loc_owner(loc_kind, fit) {
                    let key = (fit_key, loc_kind);
                    extend_vec_from_map_set_l1(affectees, &self.affectee_root, &key);
                }
            }
            AffecteeFilter::Loc(loc) => {
                let loc_kind = match loc {
                    Location::ItemList(_) => LocationKind::Ship,
                    _ => match loc.try_into() {
                        Ok(loc_kind) => loc_kind,
                        _ => return,
                    },
                };
                let fit = ctx.u_data.fits.get(fit_key);
                if check_loc_owner(loc_kind, fit) {
                    let key = (fit_key, loc_kind);
                    extend_vec_from_map_set_l1(affectees, &self.affectee_loc, &key);
                }
            }
            AffecteeFilter::LocGrp(loc, item_grp_id) => {
                let loc_kind = match loc {
                    Location::ItemList(_) => LocationKind::Ship,
                    _ => match loc.try_into() {
                        Ok(loc_kind) => loc_kind,
                        _ => return,
                    },
                };
                let fit = ctx.u_data.fits.get(fit_key);
                if check_loc_owner(loc_kind, fit) {
                    let key = (fit_key, loc_kind, item_grp_id);
                    extend_vec_from_map_set_l1(affectees, &self.affectee_loc_grp, &key);
                }
            }
            AffecteeFilter::LocSrq(loc, srq_type_id) => {
                let loc_kind = match loc {
                    Location::ItemList(_) => LocationKind::Ship,
                    _ => match loc.try_into() {
                        Ok(loc_kind) => loc_kind,
                        _ => return,
                    },
                };
                let fit = ctx.u_data.fits.get(fit_key);
                if check_loc_owner(loc_kind, fit) {
                    let key = (fit_key, loc_kind, srq_type_id);
                    extend_vec_from_map_set_l1(affectees, &self.affectee_loc_srq, &key);
                }
            }
            AffecteeFilter::OwnSrq(srq_type_id) => {
                let key = (fit_key, srq_type_id);
                extend_vec_from_map_set_l1(affectees, &self.affectee_own_srq, &key);
            }
        }
    }
    fn fill_affectees_for_item_system(
        &self,
        affectees: &mut Vec<UItemKey>,
        ctx: SvcCtx,
        rmod: &RawModifier,
        projectee_key: UItemKey,
    ) {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Ship => {
                    let projectee_item = ctx.u_data.items.get(projectee_key);
                    if let UItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), UShipKind::Ship)
                    {
                        affectees.push(projectee_key);
                    }
                }
                Location::Structure => {
                    let projectee_item = ctx.u_data.items.get(projectee_key);
                    if let UItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), UShipKind::Structure)
                    {
                        affectees.push(projectee_key);
                    }
                }
                Location::Char => {
                    let projectee_item = ctx.u_data.items.get(projectee_key);
                    if let UItem::Ship(projectee_ship) = projectee_item
                        && let Some(char_key) = ctx.u_data.fits.get(projectee_ship.get_fit_key()).character
                    {
                        affectees.push(char_key);
                    }
                }
                _ => (),
            },
            AffecteeFilter::Loc(loc) => match loc {
                Location::Ship => {
                    let projectee_item = ctx.u_data.items.get(projectee_key);
                    if let UItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), UShipKind::Ship)
                    {
                        let key = (projectee_ship.get_fit_key(), LocationKind::Ship);
                        extend_vec_from_map_set_l1(affectees, &self.affectee_loc, &key);
                    }
                }
                Location::Structure => {
                    let projectee_item = ctx.u_data.items.get(projectee_key);
                    if let UItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), UShipKind::Structure)
                    {
                        let key = (projectee_ship.get_fit_key(), LocationKind::Structure);
                        extend_vec_from_map_set_l1(affectees, &self.affectee_loc, &key);
                    }
                }
                Location::Char => {
                    let projectee_item = ctx.u_data.items.get(projectee_key);
                    if let UItem::Ship(projectee_ship) = projectee_item {
                        let key = (projectee_ship.get_fit_key(), LocationKind::Character);
                        extend_vec_from_map_set_l1(affectees, &self.affectee_loc, &key);
                    }
                }
                _ => (),
            },
            AffecteeFilter::LocGrp(loc, item_grp_id) => match loc {
                Location::Ship => {
                    let projectee_item = ctx.u_data.items.get(projectee_key);
                    if let UItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), UShipKind::Ship)
                    {
                        let key = (projectee_ship.get_fit_key(), LocationKind::Ship, item_grp_id);
                        extend_vec_from_map_set_l1(affectees, &self.affectee_loc_grp, &key);
                    }
                }
                Location::Structure => {
                    let projectee_item = ctx.u_data.items.get(projectee_key);
                    if let UItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), UShipKind::Structure)
                    {
                        let key = (projectee_ship.get_fit_key(), LocationKind::Structure, item_grp_id);
                        extend_vec_from_map_set_l1(affectees, &self.affectee_loc_grp, &key);
                    }
                }
                Location::Char => {
                    let projectee_item = ctx.u_data.items.get(projectee_key);
                    if let UItem::Ship(projectee_ship) = projectee_item {
                        let key = (projectee_ship.get_fit_key(), LocationKind::Character, item_grp_id);
                        extend_vec_from_map_set_l1(affectees, &self.affectee_loc_grp, &key);
                    }
                }
                _ => (),
            },
            AffecteeFilter::LocSrq(loc, srq_type_id) => match loc {
                Location::Ship => {
                    let projectee_item = ctx.u_data.items.get(projectee_key);
                    if let UItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), UShipKind::Ship)
                    {
                        let key = (projectee_ship.get_fit_key(), LocationKind::Ship, srq_type_id);
                        extend_vec_from_map_set_l1(affectees, &self.affectee_loc_srq, &key);
                    }
                }
                Location::Structure => {
                    let projectee_item = ctx.u_data.items.get(projectee_key);
                    if let UItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), UShipKind::Structure)
                    {
                        let key = (projectee_ship.get_fit_key(), LocationKind::Structure, srq_type_id);
                        extend_vec_from_map_set_l1(affectees, &self.affectee_loc_srq, &key);
                    }
                }
                Location::Char => {
                    let projectee_item = ctx.u_data.items.get(projectee_key);
                    if let UItem::Ship(projectee_ship) = projectee_item {
                        let key = (projectee_ship.get_fit_key(), LocationKind::Character, srq_type_id);
                        extend_vec_from_map_set_l1(affectees, &self.affectee_loc_srq, &key);
                    }
                }
                _ => (),
            },
            AffecteeFilter::OwnSrq(srq_type_id) => {
                let projectee_item = ctx.u_data.items.get(projectee_key);
                if let UItem::Ship(projectee_ship) = projectee_item {
                    let key = (projectee_ship.get_fit_key(), srq_type_id);
                    extend_vec_from_map_set_l1(affectees, &self.affectee_own_srq, &key);
                }
            }
        }
    }
    fn fill_affectees_for_item_target(
        &self,
        affectees: &mut Vec<UItemKey>,
        ctx: SvcCtx,
        rmod: &RawModifier,
        projectee_key: UItemKey,
    ) {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) => {
                if matches!(loc, Location::Target) {
                    affectees.push(projectee_key);
                }
            }
            AffecteeFilter::Loc(loc) => {
                if matches!(loc, Location::Target) {
                    let projectee_item = ctx.u_data.items.get(projectee_key);
                    if let UItem::Ship(projectee_ship) = projectee_item {
                        match projectee_ship.get_kind() {
                            UShipKind::Ship => {
                                let key = (projectee_ship.get_fit_key(), LocationKind::Ship);
                                extend_vec_from_map_set_l1(affectees, &self.affectee_loc, &key);
                            }
                            UShipKind::Structure => {
                                let key = (projectee_ship.get_fit_key(), LocationKind::Structure);
                                extend_vec_from_map_set_l1(affectees, &self.affectee_loc, &key);
                            }
                            _ => (),
                        }
                    }
                }
            }
            AffecteeFilter::LocGrp(loc, item_grp_id) => {
                if matches!(loc, Location::Target) {
                    let projectee_item = ctx.u_data.items.get(projectee_key);
                    if let UItem::Ship(projectee_ship) = projectee_item {
                        match projectee_ship.get_kind() {
                            UShipKind::Ship => {
                                let key = (projectee_ship.get_fit_key(), LocationKind::Ship, item_grp_id);
                                extend_vec_from_map_set_l1(affectees, &self.affectee_loc_grp, &key);
                            }
                            UShipKind::Structure => {
                                let key = (projectee_ship.get_fit_key(), LocationKind::Structure, item_grp_id);
                                extend_vec_from_map_set_l1(affectees, &self.affectee_loc_grp, &key);
                            }
                            _ => (),
                        }
                    }
                }
            }
            AffecteeFilter::LocSrq(loc, srq_type_id) => {
                if matches!(loc, Location::Target) {
                    let projectee_item = ctx.u_data.items.get(projectee_key);
                    if let UItem::Ship(projectee_ship) = projectee_item {
                        match projectee_ship.get_kind() {
                            UShipKind::Ship => {
                                let key = (projectee_ship.get_fit_key(), LocationKind::Ship, srq_type_id);
                                extend_vec_from_map_set_l1(affectees, &self.affectee_loc_srq, &key);
                            }
                            UShipKind::Structure => {
                                let key = (projectee_ship.get_fit_key(), LocationKind::Structure, srq_type_id);
                                extend_vec_from_map_set_l1(affectees, &self.affectee_loc_srq, &key);
                            }
                            _ => (),
                        }
                    }
                }
            }
            AffecteeFilter::OwnSrq(srq_type_id) => {
                let projectee_item = ctx.u_data.items.get(projectee_key);
                if let UItem::Ship(projectee_ship) = projectee_item {
                    let key = (projectee_ship.get_fit_key(), srq_type_id);
                    extend_vec_from_map_set_l1(affectees, &self.affectee_own_srq, &key);
                }
            }
        }
    }
    fn fill_affectees_for_item_buff(
        &self,
        affectees: &mut Vec<UItemKey>,
        ctx: SvcCtx,
        rmod: &RawModifier,
        projectee_key: UItemKey,
    ) {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(Location::ItemList(item_list_id)) => {
                let projectee_item = ctx.u_data.items.get(projectee_key);
                if projectee_item.is_item_buffable_by_item_list(&item_list_id) {
                    affectees.push(projectee_key);
                }
            }
            AffecteeFilter::Loc(Location::ItemList(item_list_id)) => {
                let projectee_item = ctx.u_data.items.get(projectee_key);
                if let Some(projectee_ship) = projectee_item.is_ship_buffable_by_item_list(&item_list_id) {
                    let key = (projectee_ship.get_fit_key(), LocationKind::Ship);
                    extend_vec_from_map_set_l1(affectees, &self.affectee_loc, &key);
                }
            }
            AffecteeFilter::LocGrp(Location::ItemList(item_list_id), item_grp_id) => {
                let projectee_item = ctx.u_data.items.get(projectee_key);
                if let Some(projectee_ship) = projectee_item.is_ship_buffable_by_item_list(&item_list_id) {
                    let key = (projectee_ship.get_fit_key(), LocationKind::Ship, item_grp_id);
                    extend_vec_from_map_set_l1(affectees, &self.affectee_loc_grp, &key);
                }
            }
            AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_type_id) => {
                let projectee_item = ctx.u_data.items.get(projectee_key);
                if let Some(projectee_ship) = projectee_item.is_ship_buffable_by_item_list(&item_list_id) {
                    let key = (projectee_ship.get_fit_key(), LocationKind::Ship, srq_type_id);
                    extend_vec_from_map_set_l1(affectees, &self.affectee_loc_srq, &key);
                }
            }
            _ => (),
        }
    }
    fn get_mods_for_changed_ship(&self, item: &UItem, cmods: &mut Vec<CtxModifier>) {
        if let (Some(item_fit_key), Some(item_loc)) = (item.get_fit_key(), item.get_ship_loc_kind()) {
            cmods.extend(self.cmods.loc.get(&(item_fit_key, item_loc)));
            for ((stored_fit_key, stored_loc, _), stored_cmods) in self.cmods.loc_grp.iter() {
                if item_fit_key == *stored_fit_key && item_loc == *stored_loc {
                    cmods.extend(stored_cmods);
                }
            }
            for ((stored_fit_key, stored_loc, _), stored_cmods) in self.cmods.loc_srq.iter() {
                if item_fit_key == *stored_fit_key && item_loc == *stored_loc {
                    cmods.extend(stored_cmods);
                }
            }
        }
    }
}

fn check_loc_owner(loc: LocationKind, fit: &UFit) -> bool {
    match loc {
        LocationKind::Character => true,
        LocationKind::Ship => matches!(fit.ship_kind, UShipKind::Ship),
        LocationKind::Structure => matches!(fit.ship_kind, UShipKind::Structure),
    }
}
