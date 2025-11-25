use crate::{
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, Context, CtxModifier, Location, LocationKind, ModifierKind, RawModifier,
            registers::standard::data::StandardRegister,
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
        // This way we can ensure context modifiers are valid, and make processing cheaper
        reuse_affectees.clear();
        match cmod.ctx {
            Context::None => self.fill_no_context(reuse_affectees, ctx, &cmod.raw),
            Context::Fit(fit_key) => self.fill_for_fit(reuse_affectees, ctx, &cmod.raw, fit_key),
            Context::ProjItem(item_key) => match cmod.raw.kind {
                ModifierKind::Targeted | ModifierKind::Buff => {
                    self.fill_direct_only(reuse_affectees, &cmod.raw, item_key)
                }
                _ => (),
            },
            Context::ProjFitItem(fit_key, item_key) => match cmod.raw.kind {
                ModifierKind::Targeted => {
                    self.fill_for_fit_item_target(reuse_affectees, ctx, &cmod.raw, fit_key, item_key)
                }
                ModifierKind::Buff => self.fill_for_fit_item_buff(reuse_affectees, ctx, &cmod.raw, fit_key),
                _ => (),
            },
        }
    }
    // Private methods
    fn fill_no_context(&self, affectees: &mut Vec<UItemKey>, ctx: SvcCtx, rmod: &RawModifier) {
        // No-context modifiers are used only for self/other modifications
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
    fn fill_for_fit(&self, affectees: &mut Vec<UItemKey>, ctx: SvcCtx, rmod: &RawModifier, fit_key: UFitKey) {
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
    fn fill_for_fit_item_target(
        &self,
        affectees: &mut Vec<UItemKey>,
        ctx: SvcCtx,
        rmod: &RawModifier,
        fit_key: UFitKey,
        projectee_key: UItemKey,
    ) {
        match rmod.affectee_filter {
            AffecteeFilter::Loc(_) => {
                let projectee_item = ctx.u_data.items.get(projectee_key);
                if let UItem::Ship(projectee_ship) = projectee_item
                    && let Ok(loc_kind) = projectee_ship.get_kind().try_into()
                {
                    let key = (projectee_ship.get_fit_key(), loc_kind);
                    extend_vec_from_map_set_l1(affectees, &self.affectee_loc, &key);
                }
            }
            AffecteeFilter::LocGrp(_, item_grp_id) => {
                let projectee_item = ctx.u_data.items.get(projectee_key);
                if let UItem::Ship(projectee_ship) = projectee_item
                    && let Ok(loc_kind) = projectee_ship.get_kind().try_into()
                {
                    let key = (projectee_ship.get_fit_key(), loc_kind, item_grp_id);
                    extend_vec_from_map_set_l1(affectees, &self.affectee_loc_grp, &key);
                }
            }
            AffecteeFilter::LocSrq(_, srq_type_id) => {
                let projectee_item = ctx.u_data.items.get(projectee_key);
                if let UItem::Ship(projectee_ship) = projectee_item
                    && let Ok(loc_kind) = projectee_ship.get_kind().try_into()
                {
                    let key = (projectee_ship.get_fit_key(), loc_kind, srq_type_id);
                    extend_vec_from_map_set_l1(affectees, &self.affectee_loc_srq, &key);
                }
            }
            AffecteeFilter::OwnSrq(srq_type_id) => {
                let key = (fit_key, srq_type_id);
                extend_vec_from_map_set_l1(affectees, &self.affectee_own_srq, &key);
            }
            _ => (),
        }
    }
    fn fill_for_fit_item_buff(&self, affectees: &mut Vec<UItemKey>, ctx: SvcCtx, rmod: &RawModifier, fit_key: UFitKey) {
        match rmod.affectee_filter {
            AffecteeFilter::Loc(_) => {
                let fit = ctx.u_data.fits.get(fit_key);
                if let Ok(loc_kind) = fit.ship_kind.try_into() {
                    let key = (fit_key, loc_kind);
                    extend_vec_from_map_set_l1(affectees, &self.affectee_loc, &key);
                }
            }
            AffecteeFilter::LocGrp(_, item_grp_id) => {
                let fit = ctx.u_data.fits.get(fit_key);
                if let Ok(loc_kind) = fit.ship_kind.try_into() {
                    let key = (fit_key, loc_kind, item_grp_id);
                    extend_vec_from_map_set_l1(affectees, &self.affectee_loc_grp, &key);
                }
            }
            AffecteeFilter::LocSrq(_, srq_type_id) => {
                let fit = ctx.u_data.fits.get(fit_key);
                if let Ok(loc_kind) = fit.ship_kind.try_into() {
                    let key = (fit_key, loc_kind, srq_type_id);
                    extend_vec_from_map_set_l1(affectees, &self.affectee_loc_srq, &key);
                }
            }
            _ => (),
        }
    }
    fn fill_direct_only(&self, affectees: &mut Vec<UItemKey>, rmod: &RawModifier, projectee_key: UItemKey) {
        if let AffecteeFilter::Direct(_) = rmod.affectee_filter {
            affectees.push(projectee_key);
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
