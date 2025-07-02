use super::{PotentialLocations, StandardRegister};
use crate::{
    def::{FitKey, ItemKey},
    svc::{
        SvcCtx,
        calc::{AffecteeFilter, Context, CtxModifier, Location, LocationKind, ModifierKind},
    },
    uad::{ShipKind, UadFit, UadItem},
    util::extend_vec_from_map_set_l1,
};

impl StandardRegister {
    // Query methods
    pub(in crate::svc::calc) fn fill_affectees(
        &self,
        reuse_affectees: &mut Vec<ItemKey>,
        ctx: SvcCtx,
        cmod: &CtxModifier,
    ) {
        reuse_affectees.clear();
        match cmod.ctx {
            Context::None => self.fill_affectees_no_context(reuse_affectees, ctx, cmod),
            Context::Fit(fit_key) => self.fill_affectees_for_fit(reuse_affectees, ctx, cmod, fit_key),
            Context::Item(item_key) => match cmod.raw.kind {
                ModifierKind::System => self.fill_affectees_for_item_system(reuse_affectees, ctx, cmod, item_key),
                ModifierKind::Targeted => self.fill_affectees_for_item_target(reuse_affectees, ctx, cmod, item_key),
                ModifierKind::Buff => self.fill_affectees_for_item_buff(reuse_affectees, ctx, cmod, item_key),
                _ => (),
            },
        }
    }
    // Modification methods
    pub(in crate::svc::calc) fn reg_affectee(&mut self, item_key: ItemKey, item: &UadItem) {
        let is_buffable = item.is_buffable();
        if is_buffable {
            self.reg_buffable_for_sw(item_key);
        }
        let fit_key = match item.get_fit_key() {
            Some(fit_key) => fit_key,
            None => return,
        };
        let root_loc = item.get_root_loc_kind();
        let a_item_grp_id = item.get_a_group_id().unwrap();
        let a_srqs = item.get_a_skill_reqs().unwrap();
        if let Some(root_loc) = root_loc {
            self.affectee_root.add_entry((fit_key, root_loc), item_key);
        }
        for loc in PotentialLocations::new(item) {
            self.affectee_loc.add_entry((fit_key, loc), item_key);
            self.affectee_loc_grp.add_entry((fit_key, loc, a_item_grp_id), item_key);
            for srq_a_item_id in a_srqs.keys() {
                self.affectee_loc_srq
                    .add_entry((fit_key, loc, *srq_a_item_id), item_key);
            }
        }
        if item.is_owner_modifiable() {
            for srq_a_item_id in a_srqs.keys() {
                self.affectee_own_srq.add_entry((fit_key, *srq_a_item_id), item_key);
            }
        }
        if is_buffable {
            self.affectee_buffable.add_entry(fit_key, item_key);
            self.reg_buffable_for_fw(item_key, fit_key);
        }
    }
    pub(in crate::svc::calc) fn unreg_affectee(&mut self, item_key: ItemKey, item: &UadItem) {
        let is_buffable = item.is_buffable();
        if is_buffable {
            self.unreg_buffable_for_sw(item_key);
        }
        let fit_key = match item.get_fit_key() {
            Some(fit_key) => fit_key,
            None => return,
        };
        let root_loc = item.get_root_loc_kind();
        let a_item_grp_id = item.get_a_group_id().unwrap();
        let a_srqs = item.get_a_skill_reqs().unwrap();

        if let Some(root_loc) = root_loc {
            self.affectee_root.remove_entry(&(fit_key, root_loc), &item_key);
        }
        for loc in PotentialLocations::new(item) {
            self.affectee_loc.remove_entry(&(fit_key, loc), &item_key);
            self.affectee_loc_grp
                .remove_entry(&(fit_key, loc, a_item_grp_id), &item_key);
            for srq_a_item_id in a_srqs.keys() {
                self.affectee_loc_srq
                    .remove_entry(&(fit_key, loc, *srq_a_item_id), &item_key);
            }
        }
        if item.is_owner_modifiable() {
            for srq_a_item_id in a_srqs.keys() {
                self.affectee_own_srq
                    .remove_entry(&(fit_key, *srq_a_item_id), &item_key);
            }
        }
        if is_buffable {
            self.affectee_buffable.remove_entry(&fit_key, &item_key);
            self.unreg_buffable_for_fw(item_key, fit_key);
        }
    }
    // Private methods
    fn fill_affectees_no_context(&self, affectees: &mut Vec<ItemKey>, ctx: SvcCtx, cmod: &CtxModifier) {
        if let AffecteeFilter::Direct(loc) = cmod.raw.affectee_filter {
            match loc {
                Location::Item => {
                    affectees.push(cmod.raw.affector_espec.item_key);
                }
                Location::Other => {
                    let item = ctx.uad.items.get(cmod.raw.affector_espec.item_key);
                    if let Some(other_item_key) = item.get_other_key() {
                        affectees.push(other_item_key);
                    }
                }
                _ => (),
            }
        }
    }
    fn fill_affectees_for_fit(&self, affectees: &mut Vec<ItemKey>, ctx: SvcCtx, cmod: &CtxModifier, fit_key: FitKey) {
        match cmod.raw.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Everything => extend_vec_from_map_set_l1(affectees, &self.affectee_buffable, &fit_key),
                _ => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let fit = ctx.uad.fits.get(fit_key);
                        if check_loc_owner(loc, fit) {
                            extend_vec_from_map_set_l1(affectees, &self.affectee_root, &(fit_key, loc_kind));
                        }
                    }
                }
            },
            AffecteeFilter::Loc(loc) => match loc {
                Location::Everything => {
                    if is_fit_of_ship_kind(ctx, fit_key) {
                        extend_vec_from_map_set_l1(affectees, &self.affectee_loc, &(fit_key, LocationKind::Ship))
                    }
                }
                _ => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let fit = ctx.uad.fits.get(fit_key);
                        if check_loc_owner(loc, fit) {
                            extend_vec_from_map_set_l1(affectees, &self.affectee_loc, &(fit_key, loc_kind));
                        }
                    }
                }
            },
            AffecteeFilter::LocGrp(loc, a_item_grp_id) => match loc {
                Location::Everything => {
                    if is_fit_of_ship_kind(ctx, fit_key) {
                        extend_vec_from_map_set_l1(
                            affectees,
                            &self.affectee_loc_grp,
                            &(fit_key, LocationKind::Ship, a_item_grp_id),
                        );
                    }
                }
                _ => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let fit = ctx.uad.fits.get(fit_key);
                        if check_loc_owner(loc, fit) {
                            extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_grp,
                                &(fit_key, loc_kind, a_item_grp_id),
                            );
                        }
                    }
                }
            },
            AffecteeFilter::LocSrq(loc, srq_a_item_id) => match loc {
                Location::Everything => {
                    if is_fit_of_ship_kind(ctx, fit_key) {
                        extend_vec_from_map_set_l1(
                            affectees,
                            &self.affectee_loc_srq,
                            &(fit_key, LocationKind::Ship, srq_a_item_id),
                        );
                    }
                }
                _ => {
                    if let Ok(loc_kind) = loc.try_into() {
                        let fit = ctx.uad.fits.get(fit_key);
                        if check_loc_owner(loc, fit) {
                            extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_srq,
                                &(fit_key, loc_kind, srq_a_item_id),
                            );
                        }
                    }
                }
            },
            AffecteeFilter::OwnSrq(srq_a_item_id) => {
                extend_vec_from_map_set_l1(affectees, &self.affectee_own_srq, &(fit_key, srq_a_item_id));
            }
        }
    }
    fn fill_affectees_for_item_system(
        &self,
        affectees: &mut Vec<ItemKey>,
        ctx: SvcCtx,
        cmod: &CtxModifier,
        projectee_item_key: ItemKey,
    ) {
        match cmod.raw.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Ship => {
                    let projectee_item = ctx.uad.items.get(projectee_item_key);
                    if let UadItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), ShipKind::Ship)
                    {
                        affectees.push(projectee_item_key)
                    }
                }
                Location::Structure => {
                    let projectee_item = ctx.uad.items.get(projectee_item_key);
                    if let UadItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), ShipKind::Structure)
                    {
                        affectees.push(projectee_item_key)
                    }
                }
                Location::Char => {
                    let projectee_item = ctx.uad.items.get(projectee_item_key);
                    if let UadItem::Ship(projectee_ship) = projectee_item
                        && let Some(char_key) = ctx.uad.fits.get(projectee_ship.get_fit_key()).character
                    {
                        affectees.push(char_key);
                    }
                }
                _ => (),
            },
            AffecteeFilter::Loc(loc) => match loc {
                Location::Ship => {
                    let projectee_item = ctx.uad.items.get(projectee_item_key);
                    if let UadItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), ShipKind::Ship)
                    {
                        extend_vec_from_map_set_l1(
                            affectees,
                            &self.affectee_loc,
                            &(projectee_ship.get_fit_key(), LocationKind::Ship),
                        )
                    }
                }
                Location::Structure => {
                    let projectee_item = ctx.uad.items.get(projectee_item_key);
                    if let UadItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), ShipKind::Structure)
                    {
                        extend_vec_from_map_set_l1(
                            affectees,
                            &self.affectee_loc,
                            &(projectee_ship.get_fit_key(), LocationKind::Structure),
                        )
                    }
                }
                Location::Char => {
                    let projectee_item = ctx.uad.items.get(projectee_item_key);
                    if let UadItem::Ship(projectee_ship) = projectee_item {
                        extend_vec_from_map_set_l1(
                            affectees,
                            &self.affectee_loc,
                            &(projectee_ship.get_fit_key(), LocationKind::Character),
                        )
                    }
                }
                _ => (),
            },
            AffecteeFilter::LocGrp(loc, a_item_grp_id) => match loc {
                Location::Ship => {
                    let projectee_item = ctx.uad.items.get(projectee_item_key);
                    if let UadItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), ShipKind::Ship)
                    {
                        extend_vec_from_map_set_l1(
                            affectees,
                            &self.affectee_loc_grp,
                            &(projectee_ship.get_fit_key(), LocationKind::Ship, a_item_grp_id),
                        );
                    }
                }
                Location::Structure => {
                    let projectee_item = ctx.uad.items.get(projectee_item_key);
                    if let UadItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), ShipKind::Structure)
                    {
                        extend_vec_from_map_set_l1(
                            affectees,
                            &self.affectee_loc_grp,
                            &(projectee_ship.get_fit_key(), LocationKind::Structure, a_item_grp_id),
                        );
                    }
                }
                Location::Char => {
                    let projectee_item = ctx.uad.items.get(projectee_item_key);
                    if let UadItem::Ship(projectee_ship) = projectee_item {
                        extend_vec_from_map_set_l1(
                            affectees,
                            &self.affectee_loc_grp,
                            &(projectee_ship.get_fit_key(), LocationKind::Character, a_item_grp_id),
                        );
                    }
                }
                _ => (),
            },
            AffecteeFilter::LocSrq(loc, srq_a_item_id) => match loc {
                Location::Ship => {
                    let projectee_item = ctx.uad.items.get(projectee_item_key);
                    if let UadItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), ShipKind::Ship)
                    {
                        extend_vec_from_map_set_l1(
                            affectees,
                            &self.affectee_loc_srq,
                            &(projectee_ship.get_fit_key(), LocationKind::Ship, srq_a_item_id),
                        )
                    }
                }
                Location::Structure => {
                    let projectee_item = ctx.uad.items.get(projectee_item_key);
                    if let UadItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), ShipKind::Structure)
                    {
                        extend_vec_from_map_set_l1(
                            affectees,
                            &self.affectee_loc_srq,
                            &(projectee_ship.get_fit_key(), LocationKind::Structure, srq_a_item_id),
                        )
                    }
                }
                Location::Char => {
                    let projectee_item = ctx.uad.items.get(projectee_item_key);
                    if let UadItem::Ship(projectee_ship) = projectee_item {
                        extend_vec_from_map_set_l1(
                            affectees,
                            &self.affectee_loc_srq,
                            &(projectee_ship.get_fit_key(), LocationKind::Character, srq_a_item_id),
                        )
                    }
                }
                _ => (),
            },
            AffecteeFilter::OwnSrq(srq_a_item_id) => {
                let projectee_item = ctx.uad.items.get(projectee_item_key);
                if let UadItem::Ship(projectee_ship) = projectee_item {
                    extend_vec_from_map_set_l1(
                        affectees,
                        &self.affectee_own_srq,
                        &(projectee_ship.get_fit_key(), srq_a_item_id),
                    )
                }
            }
        }
    }
    fn fill_affectees_for_item_target(
        &self,
        affectees: &mut Vec<ItemKey>,
        ctx: SvcCtx,
        cmod: &CtxModifier,
        projectee_item_key: ItemKey,
    ) {
        match cmod.raw.affectee_filter {
            AffecteeFilter::Direct(loc) => {
                if matches!(loc, Location::Target) {
                    affectees.push(projectee_item_key)
                }
            }
            AffecteeFilter::Loc(loc) => {
                if matches!(loc, Location::Target) {
                    let projectee_item = ctx.uad.items.get(projectee_item_key);
                    if let UadItem::Ship(projectee_ship) = projectee_item {
                        match projectee_ship.get_kind() {
                            ShipKind::Ship => extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc,
                                &(projectee_ship.get_fit_key(), LocationKind::Ship),
                            ),
                            ShipKind::Structure => extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc,
                                &(projectee_ship.get_fit_key(), LocationKind::Structure),
                            ),
                            _ => (),
                        }
                    }
                }
            }
            AffecteeFilter::LocGrp(loc, a_item_grp_id) => {
                if matches!(loc, Location::Target) {
                    let projectee_item = ctx.uad.items.get(projectee_item_key);
                    if let UadItem::Ship(projectee_ship) = projectee_item {
                        match projectee_ship.get_kind() {
                            ShipKind::Ship => extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_grp,
                                &(projectee_ship.get_fit_key(), LocationKind::Ship, a_item_grp_id),
                            ),
                            ShipKind::Structure => extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_grp,
                                &(projectee_ship.get_fit_key(), LocationKind::Structure, a_item_grp_id),
                            ),
                            _ => (),
                        }
                    }
                }
            }
            AffecteeFilter::LocSrq(loc, srq_a_item_id) => {
                if matches!(loc, Location::Target) {
                    let projectee_item = ctx.uad.items.get(projectee_item_key);
                    if let UadItem::Ship(projectee_ship) = projectee_item {
                        match projectee_ship.get_kind() {
                            ShipKind::Ship => extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_srq,
                                &(projectee_ship.get_fit_key(), LocationKind::Ship, srq_a_item_id),
                            ),
                            ShipKind::Structure => extend_vec_from_map_set_l1(
                                affectees,
                                &self.affectee_loc_srq,
                                &(projectee_ship.get_fit_key(), LocationKind::Structure, srq_a_item_id),
                            ),
                            _ => (),
                        }
                    }
                }
            }
            AffecteeFilter::OwnSrq(srq_a_item_id) => {
                let projectee_item = ctx.uad.items.get(projectee_item_key);
                if let UadItem::Ship(projectee_ship) = projectee_item {
                    extend_vec_from_map_set_l1(
                        affectees,
                        &self.affectee_own_srq,
                        &(projectee_ship.get_fit_key(), srq_a_item_id),
                    );
                }
            }
        }
    }
    fn fill_affectees_for_item_buff(
        &self,
        affectees: &mut Vec<ItemKey>,
        ctx: SvcCtx,
        cmod: &CtxModifier,
        projectee_item_key: ItemKey,
    ) {
        match cmod.raw.affectee_filter {
            AffecteeFilter::Direct(loc) => match loc {
                Location::Everything => {
                    let projectee_item = ctx.uad.items.get(projectee_item_key);
                    if projectee_item.is_buffable() {
                        affectees.push(projectee_item_key)
                    }
                }
                Location::Ship => {
                    let projectee_item = ctx.uad.items.get(projectee_item_key);
                    if let UadItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), ShipKind::Ship)
                    {
                        affectees.push(projectee_item_key)
                    }
                }
                _ => (),
            },
            AffecteeFilter::Loc(loc) => match loc {
                Location::Everything => {
                    let projectee_item = ctx.uad.items.get(projectee_item_key);
                    if let UadItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), ShipKind::Ship)
                    {
                        extend_vec_from_map_set_l1(
                            affectees,
                            &self.affectee_loc,
                            &(projectee_ship.get_fit_key(), LocationKind::Ship),
                        );
                    }
                }
                Location::Ship => {
                    let projectee_item = ctx.uad.items.get(projectee_item_key);
                    if let UadItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), ShipKind::Ship)
                    {
                        extend_vec_from_map_set_l1(
                            affectees,
                            &self.affectee_loc,
                            &(projectee_ship.get_fit_key(), LocationKind::Ship),
                        );
                    }
                }
                _ => (),
            },
            AffecteeFilter::LocGrp(loc, a_item_grp_id) => match loc {
                Location::Everything => {
                    let projectee_item = ctx.uad.items.get(projectee_item_key);
                    if let UadItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), ShipKind::Ship)
                    {
                        extend_vec_from_map_set_l1(
                            affectees,
                            &self.affectee_loc_grp,
                            &(projectee_ship.get_fit_key(), LocationKind::Ship, a_item_grp_id),
                        );
                    }
                }
                Location::Ship => {
                    let projectee_item = ctx.uad.items.get(projectee_item_key);
                    if let UadItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), ShipKind::Ship)
                    {
                        extend_vec_from_map_set_l1(
                            affectees,
                            &self.affectee_loc_grp,
                            &(projectee_ship.get_fit_key(), LocationKind::Ship, a_item_grp_id),
                        );
                    }
                }
                _ => (),
            },
            AffecteeFilter::LocSrq(loc, srq_a_item_id) => match loc {
                Location::Everything => {
                    let projectee_item = ctx.uad.items.get(projectee_item_key);
                    if let UadItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), ShipKind::Ship)
                    {
                        extend_vec_from_map_set_l1(
                            affectees,
                            &self.affectee_loc_srq,
                            &(projectee_ship.get_fit_key(), LocationKind::Ship, srq_a_item_id),
                        );
                    }
                }
                Location::Ship => {
                    let projectee_item = ctx.uad.items.get(projectee_item_key);
                    if let UadItem::Ship(projectee_ship) = projectee_item
                        && matches!(projectee_ship.get_kind(), ShipKind::Ship)
                    {
                        extend_vec_from_map_set_l1(
                            affectees,
                            &self.affectee_loc_srq,
                            &(projectee_ship.get_fit_key(), LocationKind::Ship, srq_a_item_id),
                        );
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
}

fn check_loc_owner(loc: Location, fit: &UadFit) -> bool {
    match loc {
        Location::Char => fit.character.is_some(),
        Location::Ship => matches!(fit.kind, ShipKind::Ship),
        Location::Structure => matches!(fit.kind, ShipKind::Structure),
        _ => false,
    }
}

fn is_fit_of_ship_kind(ctx: SvcCtx, fit_key: FitKey) -> bool {
    let fit = ctx.uad.fits.get(fit_key);
    matches!(fit.kind, ShipKind::Ship)
}
