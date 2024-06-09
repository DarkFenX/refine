use std::{convert::TryInto, hash::Hash};

use itertools::Itertools;

use crate::{
    defs::{EAttrId, EItemGrpId, EItemId, SolFitId, SolItemId},
    sol::{
        fit::SolFits,
        fleet::SolFleet,
        item::{SolItem, SolShipKind},
        svc::svce_calc::{
            SolAffecteeFilter, SolAttrSpec, SolDomain, SolFleetUpdates, SolLocationKind, SolModifier, SolModifierKind,
        },
        SolView,
    },
    util::{StMapSetL1, StSet},
};

use super::ActiveLocations;

pub(in crate::sol::svc::svce_calc) struct SolModifierRegister {
    // Modifiers registered for an item
    // Map<affector item ID, modifiers>
    pub(super) by_affector: StMapSetL1<SolItemId, SolModifier>,
    // Modifiers which rely on an item-attribute pair value
    // Map<attr spec, modifiers>
    pub(super) by_attr_spec: StMapSetL1<SolAttrSpec, SolModifier>,
    // Modifiers which modify item directly
    // Map<affectee item ID, modifiers>
    pub(super) direct: StMapSetL1<SolItemId, SolModifier>,
    // Modifiers which modify 'other' domain are always stored here, regardless if they actually
    // modify something or not
    // Map<affector item ID, modifiers>
    pub(super) other: StMapSetL1<SolItemId, SolModifier>,
    // All modifiers which modify root entities (via ship or character reference) are kept here
    // Map<(affectee fit ID, affectee location kind), modifiers>
    pub(super) root: StMapSetL1<(SolFitId, SolLocationKind), SolModifier>,
    // Modifiers influencing all items belonging to certain fit and location kind
    // Map<(affectee fit ID, affectee location kind), modifiers>
    pub(super) loc: StMapSetL1<(SolFitId, SolLocationKind), SolModifier>,
    // Modifiers influencing items belonging to certain fit, location and group
    // Map<(affectee fit ID, affectee location, affectee group ID), modifiers>
    pub(super) loc_grp: StMapSetL1<(SolFitId, SolLocationKind, EItemGrpId), SolModifier>,
    // Modifiers influencing items belonging to certain fit and location, and having certain skill
    // requirement
    // Map<(affectee fit ID, affectee location, affectee skillreq type ID), modifiers>
    pub(super) loc_srq: StMapSetL1<(SolFitId, SolLocationKind, EItemId), SolModifier>,
    // Modifiers influencing owner-modifiable items belonging to certain fit and having certain
    // skill requirement
    // Map<(affectee fit ID, affectee skillreq type ID), modifiers>
    pub(super) own_srq: StMapSetL1<(SolFitId, EItemId), SolModifier>,
    // Modifiers influencing all buff-modifiable items
    // Map<affectee fit ID, modifiers>
    pub(super) buff_all: StMapSetL1<SolFitId, SolModifier>,
    // Fleet modifiers on a per-fit basis
    // Map<affector fit ID, modifiers>
    pub(super) fleet_fit: StMapSetL1<SolFitId, SolModifier>,
    // System-wide modifiers
    pub(super) sw: StSet<SolModifier>,
}
impl SolModifierRegister {
    pub(in crate::sol::svc::svce_calc) fn new() -> Self {
        Self {
            by_affector: StMapSetL1::new(),
            by_attr_spec: StMapSetL1::new(),
            direct: StMapSetL1::new(),
            other: StMapSetL1::new(),
            root: StMapSetL1::new(),
            loc: StMapSetL1::new(),
            loc_grp: StMapSetL1::new(),
            loc_srq: StMapSetL1::new(),
            own_srq: StMapSetL1::new(),
            buff_all: StMapSetL1::new(),
            fleet_fit: StMapSetL1::new(),
            sw: StSet::new(),
        }
    }
    // Query methods
    pub(in crate::sol::svc::svce_calc) fn get_mods_for_affectee(
        &self,
        item: &SolItem,
        attr_id: &EAttrId,
        fits: &SolFits,
    ) -> Vec<SolModifier> {
        let item_id = item.get_id();
        let fit_opt = item.get_fit_id().map(|v| fits.get_fit(&v).ok()).flatten();
        let root_loc_opt = item.get_root_loc_kind();
        let grp_id_opt = item.get_group_id().ok();
        let srqs_opt = item.get_skill_reqs().ok();
        let mut mods = Vec::new();
        filter_and_extend(&mut mods, &self.direct, &item_id, attr_id);
        if let Some(other_item_id) = item.get_other() {
            filter_and_extend(&mut mods, &self.other, &other_item_id, attr_id);
        }
        if let (Some(fit), Some(root_loc)) = (fit_opt, root_loc_opt) {
            filter_and_extend(&mut mods, &self.root, &(fit.id, root_loc), attr_id);
        }
        if let Some(fit) = fit_opt {
            for loc_kind in ActiveLocations::new(item, fit) {
                filter_and_extend(&mut mods, &self.loc, &(fit.id, loc_kind), attr_id);
            }
        }
        if let (Some(fit), Some(grp_id)) = (fit_opt, grp_id_opt) {
            for loc_kind in ActiveLocations::new(item, fit) {
                filter_and_extend(&mut mods, &self.loc_grp, &(fit.id, loc_kind, grp_id), attr_id);
            }
        }
        if let (Some(fit), Some(srqs)) = (fit_opt, &srqs_opt) {
            for loc_kind in ActiveLocations::new(item, fit) {
                for srq_id in srqs.keys() {
                    filter_and_extend(&mut mods, &self.loc_srq, &(fit.id, loc_kind, *srq_id), attr_id);
                }
            }
        }
        if item.is_owner_modifiable() {
            if let (Some(fit), Some(srqs)) = (fit_opt, &srqs_opt) {
                for srq_id in srqs.keys() {
                    filter_and_extend(&mut mods, &self.own_srq, &(fit.id, *srq_id), attr_id);
                }
            }
        }
        if item.is_buff_modifiable() {
            if let Some(fit) = fit_opt {
                filter_and_extend(&mut mods, &self.buff_all, &fit.id, attr_id);
            }
        }
        mods
    }
    pub(in crate::sol::svc::svce_calc) fn get_mods_for_changed_root(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
    ) -> Vec<SolModifier> {
        let mut mods = Vec::new();
        if let (Some(_), Some(loc_kind)) = (item.get_fit_id(), item.get_root_loc_kind()) {
            for (sub_item_id, sub_mods) in self.by_affector.iter() {
                if let Ok(_) = sol_view.items.get_item(sub_item_id) {
                    // TODO: This should be refined/optimized. It should pick only modifiers which
                    // TODO: target fit of item being changed.
                    for sub_mod in sub_mods {
                        if match sub_mod.affectee_filter {
                            SolAffecteeFilter::Loc(sub_dom) => compare_loc_dom(loc_kind, sub_dom),
                            SolAffecteeFilter::LocGrp(sub_dom, _) => compare_loc_dom(loc_kind, sub_dom),
                            SolAffecteeFilter::LocSrq(sub_dom, _) => compare_loc_dom(loc_kind, sub_dom),
                            _ => false,
                        } {
                            mods.push(*sub_mod);
                        }
                    }
                }
            }
        }
        mods
    }
    pub(in crate::sol::svc::svce_calc) fn iter_affector_item_mods(
        &self,
        affector_item_id: &SolItemId,
    ) -> impl ExactSizeIterator<Item = &SolModifier> {
        self.by_affector.get(affector_item_id)
    }
    pub(in crate::sol::svc::svce_calc) fn iter_affector_spec_mods(
        &self,
        affector_attr_spec: &SolAttrSpec,
    ) -> impl ExactSizeIterator<Item = &SolModifier> {
        self.by_attr_spec.get(affector_attr_spec)
    }
    // Modification methods
    pub(in crate::sol::svc::svce_calc) fn reg_fit(&mut self, fit_id: &SolFitId) {
        let sw_modifiers = self.sw.iter().map(|v| *v).collect_vec();
        if !sw_modifiers.is_empty() {
            let fit_ids = vec![*fit_id];
            for modifier in sw_modifiers.iter() {
                self.apply_mod_to_fits(*modifier, &fit_ids);
            }
        }
    }
    pub(in crate::sol::svc::svce_calc) fn unreg_fit(&mut self, fit_id: &SolFitId) {
        let sw_modifiers = self.sw.iter().map(|v| *v).collect_vec();
        if !sw_modifiers.is_empty() {
            let fit_ids = vec![*fit_id];
            for modifier in sw_modifiers.iter() {
                self.unapply_mod_from_fits(modifier, &fit_ids);
            }
        }
    }
    pub(in crate::sol::svc::svce_calc) fn reg_fleet_for_fit(
        &mut self,
        fleet: &SolFleet,
        fit_id: &SolFitId,
    ) -> SolFleetUpdates {
        let updates = self.get_fleet_updates(fleet, fit_id);
        if !updates.incoming.is_empty() {
            let fit_ids = vec![*fit_id];
            for modifier in updates.incoming.iter() {
                self.apply_mod_to_fits(*modifier, &fit_ids);
            }
        }
        if !updates.outgoing.is_empty() {
            let fit_ids = fleet.iter_fits().map(|v| *v).filter(|v| v != fit_id).collect();
            for modifier in updates.outgoing.iter() {
                self.apply_mod_to_fits(*modifier, &fit_ids);
            }
        }
        updates
    }
    pub(in crate::sol::svc::svce_calc) fn unreg_fleet_for_fit(
        &mut self,
        fleet: &SolFleet,
        fit_id: &SolFitId,
    ) -> SolFleetUpdates {
        let updates = self.get_fleet_updates(fleet, fit_id);
        if !updates.incoming.is_empty() {
            let fit_ids = vec![*fit_id];
            for modifier in updates.incoming.iter() {
                self.unapply_mod_from_fits(modifier, &fit_ids);
            }
        }
        if !updates.outgoing.is_empty() {
            let fit_ids = fleet.iter_fits().map(|v| *v).filter(|v| v != fit_id).collect();
            for modifier in updates.outgoing.iter() {
                self.unapply_mod_from_fits(modifier, &fit_ids);
            }
        }
        updates
    }
    pub(in crate::sol::svc::svce_calc) fn reg_mod(
        &mut self,
        fit_ids: &mut Vec<SolFitId>,
        sol_view: &SolView,
        item: &SolItem,
        modifier: SolModifier,
    ) -> bool {
        let item_id = item.get_id();
        // Maintain helper data containers
        self.by_affector.add_entry(modifier.affector_item_id, modifier);
        if let Some(affector_attr_id) = modifier.get_affector_attr_id() {
            let affector_spec = SolAttrSpec::new(item_id, affector_attr_id);
            self.by_attr_spec.add_entry(affector_spec, modifier);
        }
        if let Some(proj_info) = modifier.proj_info {
            if let Some(optimal_attr_id) = proj_info.optimal_attr_id {
                let affector_spec = SolAttrSpec::new(item_id, optimal_attr_id);
                self.by_attr_spec.add_entry(affector_spec, modifier);
            }
            if let Some(falloff_attr_id) = proj_info.falloff_attr_id {
                let affector_spec = SolAttrSpec::new(item_id, falloff_attr_id);
                self.by_attr_spec.add_entry(affector_spec, modifier);
            }
        }
        if matches!(item, SolItem::SwEffect(_)) {
            self.sw.insert(modifier);
        }
        if matches!(modifier.kind, SolModifierKind::FleetBuff) {
            if let Some(fit_id) = item.get_fit_id() {
                self.fleet_fit.add_entry(fit_id, modifier);
            }
        }
        // Process simple modifiers which do not rely on any extra context
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolDomain::Item => {
                    self.direct.add_entry(modifier.affector_item_id, modifier);
                    return true;
                }
                SolDomain::Other => {
                    self.other.add_entry(modifier.affector_item_id, modifier);
                    return true;
                }
                _ => (),
            },
            _ => (),
        }
        match (modifier.kind, item) {
            // System-wide modifications affect all fits
            (SolModifierKind::System | SolModifierKind::Buff, SolItem::SwEffect(_)) => {
                fit_ids.clear();
                fit_ids.extend(sol_view.fits.iter_fit_ids());
                self.apply_mod_to_fits(modifier, fit_ids)
            }
            // Fit-wide modifications affect only affector fit itself
            (SolModifierKind::System | SolModifierKind::Buff, SolItem::FwEffect(fw_effect)) => {
                fit_ids.clear();
                fit_ids.push(fw_effect.fit_id);
                self.apply_mod_to_fits(modifier, fit_ids)
            }
            // Local modifications are the same
            (SolModifierKind::Local, _) => match item.get_fit_id() {
                Some(fit_id) => {
                    fit_ids.clear();
                    fit_ids.push(fit_id);
                    self.apply_mod_to_fits(modifier, fit_ids)
                }
                None => false,
            },
            // Fleet modifications affect whole fleet, or just affector fit itself, if fleet isn't
            // set
            (SolModifierKind::FleetBuff, SolItem::Module(module)) => {
                fill_fleet_fits(fit_ids, sol_view, module.fit_id);
                self.apply_mod_to_fits(modifier, fit_ids)
            }
            // Various projectable effects affect only what they are project, depending on modifier
            // kind
            (SolModifierKind::System, SolItem::ProjEffect(proj_effect)) => {
                let mut changed = false;
                for projectee_item_id in proj_effect.projs.iter_items() {
                    let projectee_item = sol_view.items.get_item(projectee_item_id).unwrap();
                    changed = changed | self.apply_system_mod_to_item(modifier, projectee_item);
                }
                changed
            }
            (SolModifierKind::Targeted, _) => {
                let mut changed = false;
                if let Some(projectee_item_ids) = item.iter_projectee_items() {
                    for projectee_item_id in projectee_item_ids {
                        let projectee_item = sol_view.items.get_item(projectee_item_id).unwrap();
                        changed = changed | self.apply_targeted_mod_to_item(modifier, projectee_item);
                    }
                }
                changed
            }
            (SolModifierKind::Buff, _) => {
                let mut changed = false;
                if let Some(projectee_item_ids) = item.iter_projectee_items() {
                    for projectee_item_id in projectee_item_ids {
                        let projectee_item = sol_view.items.get_item(projectee_item_id).unwrap();
                        changed = changed | self.apply_buff_mod_to_item(modifier, projectee_item);
                    }
                }
                changed
            }
            _ => false,
        }
    }
    pub(in crate::sol::svc::svce_calc) fn unreg_mod(
        &mut self,
        fit_ids: &mut Vec<SolFitId>,
        sol_view: &SolView,
        item: &SolItem,
        modifier: &SolModifier,
    ) -> bool {
        let item_id = item.get_id();
        // Maintain helper data containers
        self.by_affector.remove_entry(&modifier.affector_item_id, modifier);
        if let Some(affector_attr_id) = modifier.get_affector_attr_id() {
            let affector_spec = SolAttrSpec::new(item_id, affector_attr_id);
            self.by_attr_spec.remove_entry(&affector_spec, modifier);
        }
        if let Some(proj_info) = modifier.proj_info {
            if let Some(optimal_attr_id) = proj_info.optimal_attr_id {
                let affector_spec = SolAttrSpec::new(item_id, optimal_attr_id);
                self.by_attr_spec.remove_entry(&affector_spec, modifier);
            }
            if let Some(falloff_attr_id) = proj_info.falloff_attr_id {
                let affector_spec = SolAttrSpec::new(item_id, falloff_attr_id);
                self.by_attr_spec.remove_entry(&affector_spec, modifier);
            }
        }
        if matches!(item, SolItem::SwEffect(_)) {
            self.sw.remove(modifier);
        }
        if matches!(modifier.kind, SolModifierKind::FleetBuff) {
            if let Some(fit_id) = item.get_fit_id() {
                self.fleet_fit.remove_entry(&fit_id, modifier);
            }
        }
        // Process simple modifiers which do not rely on any extra context
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolDomain::Item => {
                    self.direct.remove_entry(&modifier.affector_item_id, modifier);
                    return true;
                }
                SolDomain::Other => {
                    self.other.remove_entry(&modifier.affector_item_id, modifier);
                    return true;
                }
                _ => (),
            },
            _ => (),
        }
        match (modifier.kind, item) {
            // System-wide modifications affect all fits
            (SolModifierKind::System | SolModifierKind::Buff, SolItem::SwEffect(_)) => {
                fit_ids.clear();
                fit_ids.extend(sol_view.fits.iter_fit_ids());
                self.unapply_mod_from_fits(modifier, fit_ids)
            }
            // Fit-wide modifications affect only affector fit itself
            (SolModifierKind::System | SolModifierKind::Buff, SolItem::FwEffect(fw_effect)) => {
                fit_ids.clear();
                fit_ids.push(fw_effect.fit_id);
                self.unapply_mod_from_fits(modifier, fit_ids)
            }
            // Local modifications are the same
            (SolModifierKind::Local, _) => match item.get_fit_id() {
                Some(fit_id) => {
                    fit_ids.clear();
                    fit_ids.push(fit_id);
                    self.unapply_mod_from_fits(modifier, fit_ids)
                }
                None => false,
            },
            // Fleet modifications affect whole fleet, or just affector fit itself, if fleet isn't
            // set
            (SolModifierKind::FleetBuff, SolItem::Module(module)) => {
                fill_fleet_fits(fit_ids, sol_view, module.fit_id);
                self.unapply_mod_from_fits(modifier, fit_ids)
            }
            // Various projectable effects affect only what they are project, depending on modifier
            // kind
            (SolModifierKind::System, SolItem::ProjEffect(proj_effect)) => {
                let mut changed = false;
                for projectee_item_id in proj_effect.projs.iter_items() {
                    let projectee_item = sol_view.items.get_item(projectee_item_id).unwrap();
                    changed = changed | self.unapply_system_mod_from_item(modifier, projectee_item);
                }
                changed
            }
            (SolModifierKind::Targeted, _) => {
                let mut changed = false;
                if let Some(projectee_item_ids) = item.iter_projectee_items() {
                    for projectee_item_id in projectee_item_ids {
                        let projectee_item = sol_view.items.get_item(projectee_item_id).unwrap();
                        changed = changed | self.unapply_targeted_mod_from_item(modifier, projectee_item);
                    }
                }
                changed
            }
            (SolModifierKind::Buff, _) => {
                let mut changed = false;
                if let Some(projectee_item_ids) = item.iter_projectee_items() {
                    for projectee_item_id in projectee_item_ids {
                        let projectee_item = sol_view.items.get_item(projectee_item_id).unwrap();
                        changed = changed | self.unapply_buff_mod_from_item(modifier, projectee_item);
                    }
                }
                changed
            }
            _ => false,
        }
    }
    pub(in crate::sol::svc::svce_calc) fn add_mod_projection(
        &mut self,
        modifier: SolModifier,
        projectee_item: &SolItem,
    ) -> bool {
        match modifier.kind {
            SolModifierKind::System => self.apply_system_mod_to_item(modifier, projectee_item),
            SolModifierKind::Targeted => self.apply_targeted_mod_to_item(modifier, projectee_item),
            SolModifierKind::Buff => self.apply_buff_mod_to_item(modifier, projectee_item),
            _ => false,
        }
    }
    pub(in crate::sol::svc::svce_calc) fn remove_mod_projection(
        &mut self,
        modifier: &SolModifier,
        projectee_item: &SolItem,
    ) -> bool {
        match modifier.kind {
            SolModifierKind::System => self.unapply_system_mod_from_item(modifier, projectee_item),
            SolModifierKind::Targeted => self.unapply_targeted_mod_from_item(modifier, projectee_item),
            SolModifierKind::Buff => self.unapply_buff_mod_from_item(modifier, projectee_item),
            _ => false,
        }
    }
    // Private - fit methods
    fn apply_mod_to_fits(&mut self, modifier: SolModifier, fit_ids: &Vec<SolFitId>) -> bool {
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolDomain::Everything => {
                    for fit_id in fit_ids.iter() {
                        self.buff_all.add_entry(*fit_id, modifier);
                    }
                    true
                }
                _ => match dom.try_into() {
                    Ok(loc) => {
                        for fit_id in fit_ids.iter() {
                            self.root.add_entry((*fit_id, loc), modifier);
                        }
                        true
                    }
                    _ => false,
                },
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolDomain::Everything => {
                    for fit_id in fit_ids.iter() {
                        self.loc.add_entry((*fit_id, SolLocationKind::Ship), modifier);
                        self.loc.add_entry((*fit_id, SolLocationKind::Structure), modifier);
                    }
                    true
                }
                _ => match dom.try_into() {
                    Ok(loc) => {
                        for fit_id in fit_ids.iter() {
                            self.loc.add_entry((*fit_id, loc), modifier);
                        }
                        true
                    }
                    _ => false,
                },
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolDomain::Everything => {
                    for fit_id in fit_ids.iter() {
                        self.loc_grp
                            .add_entry((*fit_id, SolLocationKind::Ship, grp_id), modifier);
                        self.loc_grp
                            .add_entry((*fit_id, SolLocationKind::Structure, grp_id), modifier);
                    }
                    true
                }
                _ => match dom.try_into() {
                    Ok(loc) => {
                        for fit_id in fit_ids.iter() {
                            self.loc_grp.add_entry((*fit_id, loc, grp_id), modifier);
                        }
                        true
                    }
                    _ => false,
                },
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolDomain::Everything => {
                    for fit_id in fit_ids.iter() {
                        self.loc_srq
                            .add_entry((*fit_id, SolLocationKind::Ship, srq_id), modifier);
                        self.loc_srq
                            .add_entry((*fit_id, SolLocationKind::Structure, srq_id), modifier);
                    }
                    true
                }
                _ => match dom.try_into() {
                    Ok(loc) => {
                        for fit_id in fit_ids.iter() {
                            self.loc_srq.add_entry((*fit_id, loc, srq_id), modifier);
                        }
                        true
                    }
                    _ => false,
                },
            },
            SolAffecteeFilter::OwnSrq(srq_id) => {
                for fit_id in fit_ids.iter() {
                    self.own_srq.add_entry((*fit_id, srq_id), modifier);
                }
                true
            }
        }
    }
    fn unapply_mod_from_fits(&mut self, modifier: &SolModifier, fit_ids: &Vec<SolFitId>) -> bool {
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolDomain::Everything => {
                    for fit_id in fit_ids.iter() {
                        self.buff_all.remove_entry(fit_id, &modifier);
                    }
                    true
                }
                _ => match dom.try_into() {
                    Ok(loc) => {
                        for fit_id in fit_ids.iter() {
                            self.root.remove_entry(&(*fit_id, loc), &modifier);
                        }
                        true
                    }
                    _ => false,
                },
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolDomain::Everything => {
                    for fit_id in fit_ids.iter() {
                        self.loc.remove_entry(&(*fit_id, SolLocationKind::Ship), &modifier);
                        self.loc.remove_entry(&(*fit_id, SolLocationKind::Structure), &modifier);
                    }
                    true
                }
                _ => match dom.try_into() {
                    Ok(loc) => {
                        for fit_id in fit_ids.iter() {
                            self.loc.remove_entry(&(*fit_id, loc), &modifier);
                        }
                        true
                    }
                    _ => false,
                },
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolDomain::Everything => {
                    for fit_id in fit_ids.iter() {
                        self.loc_grp
                            .remove_entry(&(*fit_id, SolLocationKind::Ship, grp_id), &modifier);
                        self.loc_grp
                            .remove_entry(&(*fit_id, SolLocationKind::Structure, grp_id), &modifier);
                    }
                    true
                }
                _ => match dom.try_into() {
                    Ok(loc) => {
                        for fit_id in fit_ids.iter() {
                            self.loc_grp.remove_entry(&(*fit_id, loc, grp_id), &modifier);
                        }
                        true
                    }
                    _ => false,
                },
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolDomain::Everything => {
                    for fit_id in fit_ids.iter() {
                        self.loc_srq
                            .remove_entry(&(*fit_id, SolLocationKind::Ship, srq_id), &modifier);
                        self.loc_srq
                            .remove_entry(&(*fit_id, SolLocationKind::Structure, srq_id), &modifier);
                    }
                    true
                }
                _ => match dom.try_into() {
                    Ok(loc) => {
                        for fit_id in fit_ids.iter() {
                            self.loc_srq.remove_entry(&(*fit_id, loc, srq_id), &modifier);
                        }
                        true
                    }
                    _ => false,
                },
            },
            SolAffecteeFilter::OwnSrq(srq) => {
                for fit_id in fit_ids.iter() {
                    self.own_srq.remove_entry(&(*fit_id, srq), &modifier);
                }
                true
            }
        }
    }
    // Private - system-to-item methods
    fn apply_system_mod_to_item(&mut self, modifier: SolModifier, projectee_item: &SolItem) -> bool {
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        self.root
                            .add_entry((projectee_ship.fit_id, SolLocationKind::Ship), modifier);
                        true
                    }
                    _ => false,
                },
                SolDomain::Structure => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Structure) => {
                        self.root
                            .add_entry((projectee_ship.fit_id, SolLocationKind::Structure), modifier);
                        true
                    }
                    _ => false,
                },
                SolDomain::Char => match projectee_item {
                    SolItem::Ship(projectee_ship) => {
                        self.root
                            .add_entry((projectee_ship.fit_id, SolLocationKind::Character), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        self.loc
                            .add_entry((projectee_ship.fit_id, SolLocationKind::Ship), modifier);
                        true
                    }
                    _ => false,
                },
                SolDomain::Structure => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Structure) => {
                        self.loc
                            .add_entry((projectee_ship.fit_id, SolLocationKind::Structure), modifier);
                        true
                    }
                    _ => false,
                },
                SolDomain::Char => match projectee_item {
                    SolItem::Ship(projectee_ship) => {
                        self.loc
                            .add_entry((projectee_ship.fit_id, SolLocationKind::Character), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        self.loc_grp
                            .add_entry((projectee_ship.fit_id, SolLocationKind::Ship, grp_id), modifier);
                        true
                    }
                    _ => false,
                },
                SolDomain::Structure => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Structure) => {
                        self.loc_grp
                            .add_entry((projectee_ship.fit_id, SolLocationKind::Structure, grp_id), modifier);
                        true
                    }
                    _ => false,
                },
                SolDomain::Char => match projectee_item {
                    SolItem::Ship(projectee_ship) => {
                        self.loc_grp
                            .add_entry((projectee_ship.fit_id, SolLocationKind::Character, grp_id), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        self.loc_srq
                            .add_entry((projectee_ship.fit_id, SolLocationKind::Ship, srq_id), modifier);
                        true
                    }
                    _ => false,
                },
                SolDomain::Structure => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Structure) => {
                        self.loc_srq
                            .add_entry((projectee_ship.fit_id, SolLocationKind::Structure, srq_id), modifier);
                        true
                    }
                    _ => false,
                },
                SolDomain::Char => match projectee_item {
                    SolItem::Ship(projectee_ship) => {
                        self.loc_srq
                            .add_entry((projectee_ship.fit_id, SolLocationKind::Character, srq_id), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::OwnSrq(srq_id) => match projectee_item {
                SolItem::Ship(projectee_ship) => {
                    self.own_srq.add_entry((projectee_ship.fit_id, srq_id), modifier);
                    true
                }
                _ => false,
            },
        }
    }
    fn unapply_system_mod_from_item(&mut self, modifier: &SolModifier, projectee_item: &SolItem) -> bool {
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        self.root
                            .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Ship), modifier);
                        true
                    }
                    _ => false,
                },
                SolDomain::Structure => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Structure) => {
                        self.root
                            .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Structure), modifier);
                        true
                    }
                    _ => false,
                },
                SolDomain::Char => match projectee_item {
                    SolItem::Ship(projectee_ship) => {
                        self.root
                            .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Character), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        self.loc
                            .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Ship), modifier);
                        true
                    }
                    _ => false,
                },
                SolDomain::Structure => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Structure) => {
                        self.loc
                            .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Structure), modifier);
                        true
                    }
                    _ => false,
                },
                SolDomain::Char => match projectee_item {
                    SolItem::Ship(projectee_ship) => {
                        self.loc
                            .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Character), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        self.loc_grp
                            .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Ship, grp_id), modifier);
                        true
                    }
                    _ => false,
                },
                SolDomain::Structure => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Structure) => {
                        self.loc_grp
                            .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Structure, grp_id), modifier);
                        true
                    }
                    _ => false,
                },
                SolDomain::Char => match projectee_item {
                    SolItem::Ship(projectee_ship) => {
                        self.loc_grp
                            .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Character, grp_id), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        self.loc_srq
                            .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Ship, srq_id), modifier);
                        true
                    }
                    _ => false,
                },
                SolDomain::Structure => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Structure) => {
                        self.loc_srq
                            .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Structure, srq_id), modifier);
                        true
                    }
                    _ => false,
                },
                SolDomain::Char => match projectee_item {
                    SolItem::Ship(projectee_ship) => {
                        self.loc_srq
                            .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Character, srq_id), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::OwnSrq(srq_id) => match projectee_item {
                SolItem::Ship(projectee_ship) => {
                    self.own_srq.remove_entry(&(projectee_ship.fit_id, srq_id), modifier);
                    true
                }
                _ => false,
            },
        }
    }
    // Private - targeted-to-item methods
    fn apply_targeted_mod_to_item(&mut self, modifier: SolModifier, projectee_item: &SolItem) -> bool {
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolDomain::Target => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.kind {
                        SolShipKind::Ship => {
                            self.root
                                .add_entry((projectee_ship.fit_id, SolLocationKind::Ship), modifier);
                            true
                        }
                        SolShipKind::Structure => {
                            self.root
                                .add_entry((projectee_ship.fit_id, SolLocationKind::Structure), modifier);
                            true
                        }
                        _ => {
                            self.direct.add_entry(projectee_ship.id, modifier);
                            true
                        }
                    },
                    _ => {
                        self.direct.add_entry(projectee_item.get_id(), modifier);
                        true
                    }
                },
                _ => false,
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolDomain::Target => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.kind {
                        SolShipKind::Ship => {
                            self.loc
                                .add_entry((projectee_ship.fit_id, SolLocationKind::Ship), modifier);
                            true
                        }
                        SolShipKind::Structure => {
                            self.loc
                                .add_entry((projectee_ship.fit_id, SolLocationKind::Structure), modifier);
                            true
                        }
                        _ => false,
                    },
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolDomain::Target => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.kind {
                        SolShipKind::Ship => {
                            self.loc_grp
                                .add_entry((projectee_ship.fit_id, SolLocationKind::Ship, grp_id), modifier);
                            true
                        }
                        SolShipKind::Structure => {
                            self.loc_grp
                                .add_entry((projectee_ship.fit_id, SolLocationKind::Structure, grp_id), modifier);
                            true
                        }
                        _ => false,
                    },
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolDomain::Target => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.kind {
                        SolShipKind::Ship => {
                            self.loc_srq
                                .add_entry((projectee_ship.fit_id, SolLocationKind::Ship, srq_id), modifier);
                            true
                        }
                        SolShipKind::Structure => {
                            self.loc_srq
                                .add_entry((projectee_ship.fit_id, SolLocationKind::Structure, srq_id), modifier);
                            true
                        }
                        _ => false,
                    },
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::OwnSrq(srq_id) => match projectee_item {
                SolItem::Ship(projectee_ship) => {
                    self.own_srq.add_entry((projectee_ship.fit_id, srq_id), modifier);
                    true
                }
                _ => false,
            },
        }
    }
    fn unapply_targeted_mod_from_item(&mut self, modifier: &SolModifier, projectee_item: &SolItem) -> bool {
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolDomain::Target => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.kind {
                        SolShipKind::Ship => {
                            self.root
                                .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Ship), modifier);
                            true
                        }
                        SolShipKind::Structure => {
                            self.root
                                .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Structure), modifier);
                            true
                        }
                        _ => {
                            self.direct.remove_entry(&projectee_ship.id, modifier);
                            true
                        }
                    },
                    _ => {
                        self.direct.remove_entry(&projectee_item.get_id(), modifier);
                        true
                    }
                },
                _ => false,
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolDomain::Target => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.kind {
                        SolShipKind::Ship => {
                            self.loc
                                .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Ship), modifier);
                            true
                        }
                        SolShipKind::Structure => {
                            self.loc
                                .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Structure), modifier);
                            true
                        }
                        _ => false,
                    },
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolDomain::Target => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.kind {
                        SolShipKind::Ship => {
                            self.loc_grp
                                .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Ship, grp_id), modifier);
                            true
                        }
                        SolShipKind::Structure => {
                            self.loc_grp
                                .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Structure, grp_id), modifier);
                            true
                        }
                        _ => false,
                    },
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolDomain::Target => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.kind {
                        SolShipKind::Ship => {
                            self.loc_srq
                                .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Ship, srq_id), modifier);
                            true
                        }
                        SolShipKind::Structure => {
                            self.loc_srq
                                .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Structure, srq_id), modifier);
                            true
                        }
                        _ => false,
                    },
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::OwnSrq(srq_id) => match projectee_item {
                SolItem::Ship(projectee_ship) => {
                    self.own_srq.remove_entry(&(projectee_ship.fit_id, srq_id), modifier);
                    true
                }
                _ => false,
            },
        }
    }
    // Private - buff-to-item methods
    fn apply_buff_mod_to_item(&mut self, modifier: SolModifier, projectee_item: &SolItem) -> bool {
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolDomain::Everything => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.kind {
                        SolShipKind::Ship => {
                            self.root
                                .add_entry((projectee_ship.fit_id, SolLocationKind::Ship), modifier);
                            true
                        }
                        SolShipKind::Structure => {
                            self.root
                                .add_entry((projectee_ship.fit_id, SolLocationKind::Structure), modifier);
                            true
                        }
                        _ => {
                            self.direct.add_entry(projectee_ship.id, modifier);
                            true
                        }
                    },
                    _ if projectee_item.is_buff_modifiable() => {
                        self.direct.add_entry(projectee_item.get_id(), modifier);
                        true
                    }
                    _ => false,
                },
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        self.root
                            .add_entry((projectee_ship.fit_id, SolLocationKind::Ship), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolDomain::Everything => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.kind {
                        SolShipKind::Ship => {
                            self.loc
                                .add_entry((projectee_ship.fit_id, SolLocationKind::Ship), modifier);
                            true
                        }
                        SolShipKind::Structure => {
                            self.loc
                                .add_entry((projectee_ship.fit_id, SolLocationKind::Structure), modifier);
                            true
                        }
                        _ => false,
                    },
                    _ => false,
                },
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        self.loc
                            .add_entry((projectee_ship.fit_id, SolLocationKind::Ship), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolDomain::Everything => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.kind {
                        SolShipKind::Ship => {
                            self.loc_grp
                                .add_entry((projectee_ship.fit_id, SolLocationKind::Ship, grp_id), modifier);
                            true
                        }
                        SolShipKind::Structure => {
                            self.loc_grp
                                .add_entry((projectee_ship.fit_id, SolLocationKind::Structure, grp_id), modifier);
                            true
                        }
                        _ => false,
                    },
                    _ => false,
                },
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        self.loc_grp
                            .add_entry((projectee_ship.fit_id, SolLocationKind::Ship, grp_id), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolDomain::Everything => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.kind {
                        SolShipKind::Ship => {
                            self.loc_srq
                                .add_entry((projectee_ship.fit_id, SolLocationKind::Ship, srq_id), modifier);
                            true
                        }
                        SolShipKind::Structure => {
                            self.loc_srq
                                .add_entry((projectee_ship.fit_id, SolLocationKind::Structure, srq_id), modifier);
                            true
                        }
                        _ => false,
                    },
                    _ => false,
                },
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        self.loc_srq
                            .add_entry((projectee_ship.fit_id, SolLocationKind::Ship, srq_id), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            _ => false,
        }
    }
    fn unapply_buff_mod_from_item(&mut self, modifier: &SolModifier, projectee_item: &SolItem) -> bool {
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolDomain::Everything => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.kind {
                        SolShipKind::Ship => {
                            self.root
                                .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Ship), modifier);
                            true
                        }
                        SolShipKind::Structure => {
                            self.root
                                .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Structure), modifier);
                            true
                        }
                        _ => {
                            self.direct.remove_entry(&projectee_ship.id, modifier);
                            true
                        }
                    },
                    _ if projectee_item.is_buff_modifiable() => {
                        self.direct.remove_entry(&projectee_item.get_id(), modifier);
                        true
                    }
                    _ => false,
                },
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        self.root
                            .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Ship), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolDomain::Everything => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.kind {
                        SolShipKind::Ship => {
                            self.loc
                                .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Ship), modifier);
                            true
                        }
                        SolShipKind::Structure => {
                            self.loc
                                .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Structure), modifier);
                            true
                        }
                        _ => false,
                    },
                    _ => false,
                },
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        self.loc
                            .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Ship), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolDomain::Everything => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.kind {
                        SolShipKind::Ship => {
                            self.loc_grp
                                .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Ship, grp_id), modifier);
                            true
                        }
                        SolShipKind::Structure => {
                            self.loc_grp
                                .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Structure, grp_id), modifier);
                            true
                        }
                        _ => false,
                    },
                    _ => false,
                },
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        self.loc_grp
                            .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Ship, grp_id), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolDomain::Everything => match projectee_item {
                    SolItem::Ship(projectee_ship) => match projectee_ship.kind {
                        SolShipKind::Ship => {
                            self.loc_srq
                                .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Ship, srq_id), modifier);
                            true
                        }
                        SolShipKind::Structure => {
                            self.loc_srq
                                .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Structure, srq_id), modifier);
                            true
                        }
                        _ => false,
                    },
                    _ => false,
                },
                SolDomain::Ship => match projectee_item {
                    SolItem::Ship(projectee_ship) if matches!(projectee_ship.kind, SolShipKind::Ship) => {
                        self.loc_srq
                            .remove_entry(&(projectee_ship.fit_id, SolLocationKind::Ship, srq_id), modifier);
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            _ => false,
        }
    }
    // Private - misc methods
    fn get_fleet_updates(&self, fleet: &SolFleet, fit_id: &SolFitId) -> SolFleetUpdates {
        let mut updates = SolFleetUpdates::new();
        updates.outgoing.extend(self.fleet_fit.get(fit_id));
        for fleet_fit_id in fleet.iter_fits() {
            if fleet_fit_id == fit_id {
                continue;
            }
            updates.incoming.extend(self.fleet_fit.get(fleet_fit_id));
        }
        updates
    }
}

fn compare_loc_dom(loc: SolLocationKind, dom: SolDomain) -> bool {
    match (loc, dom) {
        (SolLocationKind::Ship, SolDomain::Everything) => true,
        (SolLocationKind::Ship, SolDomain::Ship) => true,
        (SolLocationKind::Structure, SolDomain::Everything) => true,
        (SolLocationKind::Structure, SolDomain::Structure) => true,
        (SolLocationKind::Character, SolDomain::Char) => true,
        _ => false,
    }
}

fn filter_and_extend<K: Eq + Hash>(
    vec: &mut Vec<SolModifier>,
    storage: &StMapSetL1<K, SolModifier>,
    key: &K,
    attr_id: &EAttrId,
) {
    vec.extend(storage.get(key).filter(|v| &v.affectee_attr_id == attr_id).map(|v| *v))
}

fn fill_fleet_fits(fit_ids: &mut Vec<SolFitId>, sol_view: &SolView, fit_id: SolFitId) {
    let affector_fit = sol_view.fits.get_fit(&fit_id).unwrap();
    fit_ids.clear();
    match affector_fit.fleet {
        Some(fleet_id) => {
            let fleet = sol_view.fleets.get_fleet(&fleet_id).unwrap();
            fit_ids.extend(fleet.iter_fits());
        }
        None => fit_ids.push(fit_id),
    }
}
