use crate::{
    svc::{
        SvcCtx,
        calc::{
            AffecteeFilter, Context, CtxModifier, Location, LocationKind, ModifierKind, RawModifier,
            registers::standard::data::StandardRegister,
        },
    },
    ud::{UData, UFitId, UItem, UItemId, UShipKind},
    util::extend_vec_from_map_set_l1,
};

impl StandardRegister {
    // Query methods
    pub(in crate::svc::calc) fn fill_affectees(
        &self,
        reuse_affectees: &mut Vec<UItemId>,
        ctx: SvcCtx,
        cmod: &CtxModifier,
    ) {
        // All the context modifiers passed to this method have to come from the standard register.
        // This way we can ensure context modifiers are valid, and make processing cheaper
        reuse_affectees.clear();
        match cmod.ctx {
            Context::None => self.fill_no_context(reuse_affectees, ctx, &cmod.raw),
            Context::Item(item_uid) => match cmod.raw.kind {
                ModifierKind::Buff | ModifierKind::FleetBuff | ModifierKind::Targeted => {
                    self.fill_direct_only(reuse_affectees, &cmod.raw, item_uid)
                }
                _ => (),
            },
            Context::Fit(fit_uid) => self.fill_for_fit(reuse_affectees, ctx, &cmod.raw, fit_uid),
            Context::FitItem(fit_uid, item_uid) => match cmod.raw.kind {
                ModifierKind::Targeted => {
                    self.fill_for_fit_item_target(reuse_affectees, ctx, &cmod.raw, fit_uid, item_uid)
                }
                ModifierKind::Buff | ModifierKind::FleetBuff => {
                    self.fill_for_fit_item_buff(reuse_affectees, ctx, &cmod.raw, fit_uid)
                }
                _ => (),
            },
        }
    }
    // Private methods
    fn fill_no_context(&self, affectees: &mut Vec<UItemId>, ctx: SvcCtx, rmod: &RawModifier) {
        // No-context modifiers are used only for self/other modifications
        if let AffecteeFilter::Direct(loc) = rmod.affectee_filter {
            match loc {
                Location::Item => {
                    affectees.push(rmod.affector_espec.item_uid);
                }
                Location::Other => {
                    let item = ctx.u_data.items.get(rmod.affector_espec.item_uid);
                    if let Some(other_item_uid) = item.get_other_uid() {
                        affectees.push(other_item_uid);
                    }
                }
                _ => (),
            }
        }
    }
    fn fill_for_fit(&self, affectees: &mut Vec<UItemId>, ctx: SvcCtx, rmod: &RawModifier, fit_uid: UFitId) {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(loc)
                if let Ok(loc_kind) = loc.try_into()
                    && check_location_root(ctx.u_data, loc_kind, fit_uid) =>
            {
                let key = (fit_uid, loc_kind);
                extend_vec_from_map_set_l1(affectees, &self.affectee_root, &key);
            }
            AffecteeFilter::Loc(loc)
                if let Ok(loc_kind) = loc.try_into()
                    && check_location_root(ctx.u_data, loc_kind, fit_uid) =>
            {
                let key = (fit_uid, loc_kind);
                extend_vec_from_map_set_l1(affectees, &self.affectee_loc, &key);
            }
            AffecteeFilter::LocGrp(loc, item_grp_id)
                if let Ok(loc_kind) = loc.try_into()
                    && check_location_root(ctx.u_data, loc_kind, fit_uid) =>
            {
                let key = (fit_uid, loc_kind, item_grp_id);
                extend_vec_from_map_set_l1(affectees, &self.affectee_loc_grp, &key);
            }
            AffecteeFilter::LocSrq(loc, srq_type_id)
                if let Ok(loc_kind) = loc.try_into()
                    && check_location_root(ctx.u_data, loc_kind, fit_uid) =>
            {
                let key = (fit_uid, loc_kind, srq_type_id);
                extend_vec_from_map_set_l1(affectees, &self.affectee_loc_srq, &key);
            }
            AffecteeFilter::OwnSrq(srq_type_id) => {
                let key = (fit_uid, srq_type_id);
                extend_vec_from_map_set_l1(affectees, &self.affectee_own_srq, &key);
            }
            _ => (),
        }
    }
    fn fill_for_fit_item_target(
        &self,
        affectees: &mut Vec<UItemId>,
        ctx: SvcCtx,
        rmod: &RawModifier,
        fit_uid: UFitId,
        projectee_uid: UItemId,
    ) {
        match rmod.affectee_filter {
            AffecteeFilter::Loc(_) => {
                let projectee_item = ctx.u_data.items.get(projectee_uid);
                if let UItem::Ship(projectee_ship) = projectee_item
                    && let Ok(loc_kind) = projectee_ship.get_kind().try_into()
                {
                    let key = (projectee_ship.get_fit_uid(), loc_kind);
                    extend_vec_from_map_set_l1(affectees, &self.affectee_loc, &key);
                }
            }
            AffecteeFilter::LocGrp(_, item_grp_id) => {
                let projectee_item = ctx.u_data.items.get(projectee_uid);
                if let UItem::Ship(projectee_ship) = projectee_item
                    && let Ok(loc_kind) = projectee_ship.get_kind().try_into()
                {
                    let key = (projectee_ship.get_fit_uid(), loc_kind, item_grp_id);
                    extend_vec_from_map_set_l1(affectees, &self.affectee_loc_grp, &key);
                }
            }
            AffecteeFilter::LocSrq(_, srq_type_id) => {
                let projectee_item = ctx.u_data.items.get(projectee_uid);
                if let UItem::Ship(projectee_ship) = projectee_item
                    && let Ok(loc_kind) = projectee_ship.get_kind().try_into()
                {
                    let key = (projectee_ship.get_fit_uid(), loc_kind, srq_type_id);
                    extend_vec_from_map_set_l1(affectees, &self.affectee_loc_srq, &key);
                }
            }
            AffecteeFilter::OwnSrq(srq_type_id) => {
                let key = (fit_uid, srq_type_id);
                extend_vec_from_map_set_l1(affectees, &self.affectee_own_srq, &key);
            }
            _ => (),
        }
    }
    fn fill_for_fit_item_buff(&self, affectees: &mut Vec<UItemId>, ctx: SvcCtx, rmod: &RawModifier, fit_uid: UFitId) {
        match rmod.affectee_filter {
            AffecteeFilter::Loc(_) => {
                let fit = ctx.u_data.fits.get(fit_uid);
                if let Ok(loc_kind) = fit.ship_kind.try_into() {
                    let key = (fit_uid, loc_kind);
                    extend_vec_from_map_set_l1(affectees, &self.affectee_loc, &key);
                }
            }
            AffecteeFilter::LocGrp(_, item_grp_id) => {
                let fit = ctx.u_data.fits.get(fit_uid);
                if let Ok(loc_kind) = fit.ship_kind.try_into() {
                    let key = (fit_uid, loc_kind, item_grp_id);
                    extend_vec_from_map_set_l1(affectees, &self.affectee_loc_grp, &key);
                }
            }
            AffecteeFilter::LocSrq(_, srq_type_id) => {
                let fit = ctx.u_data.fits.get(fit_uid);
                if let Ok(loc_kind) = fit.ship_kind.try_into() {
                    let key = (fit_uid, loc_kind, srq_type_id);
                    extend_vec_from_map_set_l1(affectees, &self.affectee_loc_srq, &key);
                }
            }
            _ => (),
        }
    }
    fn fill_direct_only(&self, affectees: &mut Vec<UItemId>, rmod: &RawModifier, projectee_uid: UItemId) {
        if let AffecteeFilter::Direct(_) = rmod.affectee_filter {
            affectees.push(projectee_uid);
        }
    }
}

fn check_location_root(u_data: &UData, loc: LocationKind, fit_uid: UFitId) -> bool {
    match loc {
        LocationKind::Character => true,
        LocationKind::Ship => {
            let fit = u_data.fits.get(fit_uid);
            matches!(fit.ship_kind, UShipKind::Ship)
        }
        LocationKind::Structure => {
            let fit = u_data.fits.get(fit_uid);
            matches!(fit.ship_kind, UShipKind::Structure)
        }
    }
}
