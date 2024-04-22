use std::{convert::TryInto, hash::Hash};

use itertools::Itertools;

use crate::{
    defs::{EAttrId, EItemGrpId, EItemId, SolFitId, SolItemId},
    sol::{
        fit::SolFits,
        fleet::SolFleet,
        item::{SolItem, SolItems},
        svc::svce_calc::{SolAffecteeFilter, SolAttrMod, SolFleetUpdates, SolLocType, SolModDomain, SolModType},
        SolView,
    },
    util::{StMapSetL1, StSet},
};

use super::ActiveLocations;

pub(in crate::sol::svc::svce_calc) struct SolModifierRegister {
    // Modifiers registered for an item
    // Contains: KeyedStorage<modifier item ID, modifiers>
    pub(super) mods: StMapSetL1<SolItemId, SolAttrMod>,
    // Modifiers which modify item directly
    // Contains: KeyedStorage<modifier item ID, modifiers>
    pub(super) mods_direct: StMapSetL1<SolItemId, SolAttrMod>,
    // All modifiers which modify top-level entities (via ship or character reference) are kept here
    // Contains: KeyedStorage<(target's fit ID, target's location type), modifiers>
    pub(super) mods_toploc: StMapSetL1<(SolFitId, SolLocType), SolAttrMod>,
    // Modifiers which modify 'other' domain are always stored here, regardless if they actually
    // modify something or not
    // Contains: KeyedStorage<modifier item ID, modifiers>
    pub(super) mods_other: StMapSetL1<SolItemId, SolAttrMod>,
    // Modifiers influencing all items belonging to certain fit and location type
    // Contains: KeyedStorage<(target's fit ID, target's location type), modifiers>
    pub(super) mods_parloc: StMapSetL1<(SolFitId, SolLocType), SolAttrMod>,
    // Modifiers influencing items belonging to certain fit, location and group
    // Contains: KeyedStorage<(target's fit ID, target's location, target's group ID), modifiers>
    pub(super) mods_parloc_grp: StMapSetL1<(SolFitId, SolLocType, EItemGrpId), SolAttrMod>,
    // Modifiers influencing items belonging to certain fit and location, and having certain skill requirement
    // Contains: KeyedStorage<(target's fit ID, target's location, target's skillreq type ID), modifiers>
    pub(super) mods_parloc_srq: StMapSetL1<(SolFitId, SolLocType, EItemId), SolAttrMod>,
    // Modifiers influencing owner-modifiable items belonging to certain fit and having certain skill requirement
    // Contains: KeyedStorage<(target's fit ID, target's skillreq type ID), modifiers>
    pub(super) mods_own_srq: StMapSetL1<(SolFitId, EItemId), SolAttrMod>,
    // Modifiers influencing all buff-modifiable items
    // Contains: KeyedStorage<target's fit ID, modifiers>
    pub(super) mods_buff_all: StMapSetL1<SolFitId, SolAttrMod>,
    // Fleet modifiers on a per-fit basis
    // Contains: KeyedStorage<source fit ID, modifiers>
    pub(super) mods_fleet_fit: StMapSetL1<SolFitId, SolAttrMod>,
    // System-wide modifiers
    pub(super) sw_mods: StSet<SolAttrMod>,
}
impl SolModifierRegister {
    pub(in crate::sol::svc::svce_calc) fn new() -> Self {
        Self {
            mods: StMapSetL1::new(),
            mods_direct: StMapSetL1::new(),
            mods_toploc: StMapSetL1::new(),
            mods_other: StMapSetL1::new(),
            mods_parloc: StMapSetL1::new(),
            mods_parloc_grp: StMapSetL1::new(),
            mods_parloc_srq: StMapSetL1::new(),
            mods_own_srq: StMapSetL1::new(),
            mods_buff_all: StMapSetL1::new(),
            mods_fleet_fit: StMapSetL1::new(),
            sw_mods: StSet::new(),
        }
    }
    // Query methods
    pub(in crate::sol::svc::svce_calc) fn get_mods_for_tgt(
        &self,
        tgt_item: &SolItem,
        tgt_attr_id: &EAttrId,
        fits: &SolFits,
    ) -> Vec<SolAttrMod> {
        let tgt_item_id = tgt_item.get_id();
        let tgt_fit_opt = tgt_item.get_fit_id().map(|v| fits.get_fit(&v).ok()).flatten();
        let tgt_toploc_opt = tgt_item.get_root_loc_type();
        let tgt_grp_id_res = tgt_item.get_group_id();
        let tgt_srqs_res = tgt_item.get_skill_reqs();
        let mut mods = Vec::new();
        filter_and_extend(&mut mods, &self.mods_direct, &tgt_item_id, tgt_attr_id);
        if let (Some(tgt_fit), Some(tgt_toploc)) = (tgt_fit_opt, tgt_toploc_opt) {
            filter_and_extend(&mut mods, &self.mods_toploc, &(tgt_fit.id, tgt_toploc), tgt_attr_id);
        }
        if let Some(other_item_id) = tgt_item.get_other() {
            filter_and_extend(&mut mods, &self.mods_other, &other_item_id, tgt_attr_id);
        }
        if let Some(tgt_fit) = tgt_fit_opt {
            for loc_type in ActiveLocations::new(tgt_item, tgt_fit) {
                filter_and_extend(&mut mods, &self.mods_parloc, &(tgt_fit.id, loc_type), tgt_attr_id);
            }
        }
        if let (Some(tgt_fit), Ok(tgt_grp_id)) = (tgt_fit_opt, tgt_grp_id_res) {
            for loc_type in ActiveLocations::new(tgt_item, tgt_fit) {
                filter_and_extend(
                    &mut mods,
                    &self.mods_parloc_grp,
                    &(tgt_fit.id, loc_type, tgt_grp_id),
                    tgt_attr_id,
                );
            }
        }
        if let (Some(tgt_fit), Ok(tgt_srqs)) = (tgt_fit_opt, &tgt_srqs_res) {
            for loc_type in ActiveLocations::new(tgt_item, tgt_fit) {
                for skill_a_item_id in tgt_srqs.keys() {
                    filter_and_extend(
                        &mut mods,
                        &self.mods_parloc_srq,
                        &(tgt_fit.id, loc_type, *skill_a_item_id),
                        tgt_attr_id,
                    );
                }
            }
        }
        if tgt_item.is_owner_modifiable() {
            if let (Some(tgt_fit), Ok(tgt_srqs)) = (tgt_fit_opt, &tgt_srqs_res) {
                for skill_a_item_id in tgt_srqs.keys() {
                    filter_and_extend(
                        &mut mods,
                        &self.mods_own_srq,
                        &(tgt_fit.id, *skill_a_item_id),
                        tgt_attr_id,
                    );
                }
            }
        }
        if tgt_item.is_buff_modifiable() {
            if let Some(tgt_fit) = tgt_fit_opt {
                filter_and_extend(&mut mods, &self.mods_buff_all, &tgt_fit.id, tgt_attr_id);
            }
        }
        mods
    }
    pub(in crate::sol::svc::svce_calc) fn get_mods_for_changed_location_owner(
        &mut self,
        item: &SolItem,
        items: &SolItems,
    ) -> Vec<SolAttrMod> {
        let mut mods = Vec::new();
        if let (Some(_), Some(loc_type)) = (item.get_fit_id(), item.get_root_loc_type()) {
            for (sub_item_id, sub_mods) in self.mods.iter() {
                if let Ok(_) = items.get_item(sub_item_id) {
                    // TODO: This should be refined/optimized. It should pick only modifiers which
                    // TODO: target fit of item being changed.
                    for sub_mod in sub_mods {
                        if match sub_mod.affectee_filter {
                            SolAffecteeFilter::Loc(sub_dom) => compare_loc_dom(loc_type, sub_dom),
                            SolAffecteeFilter::LocGrp(sub_dom, _) => compare_loc_dom(loc_type, sub_dom),
                            SolAffecteeFilter::LocSrq(sub_dom, _) => compare_loc_dom(loc_type, sub_dom),
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
    pub(in crate::sol::svc::svce_calc) fn iter_mods_for_src(
        &self,
        src_item_id: &SolItemId,
    ) -> impl ExactSizeIterator<Item = &SolAttrMod> {
        self.mods.get(src_item_id)
    }
    // Modification methods
    pub(in crate::sol::svc::svce_calc) fn reg_fit(&mut self, fit_id: &SolFitId) {
        let sw_mods = self.sw_mods.iter().map(|v| *v).collect_vec();
        if !sw_mods.is_empty() {
            let tgt_fit_ids = vec![*fit_id];
            for modifier in sw_mods.iter() {
                self.apply_mod_to_fits(*modifier, &tgt_fit_ids);
            }
        }
    }
    pub(in crate::sol::svc::svce_calc) fn unreg_fit(&mut self, fit_id: &SolFitId) {
        let sw_mods = self.sw_mods.iter().map(|v| *v).collect_vec();
        if !sw_mods.is_empty() {
            let tgt_fit_ids = vec![*fit_id];
            for modifier in sw_mods.iter() {
                self.unapply_mod_from_fits(modifier, &tgt_fit_ids);
            }
        }
    }
    pub(in crate::sol::svc::svce_calc) fn reg_fleet_for_fit(
        &mut self,
        sol_view: &SolView,
        fleet: &SolFleet,
        fit_id: &SolFitId,
    ) -> SolFleetUpdates {
        let updates = self.get_fleet_updates(fleet, fit_id);
        if !updates.incoming.is_empty() {
            let tgt_fit_ids = vec![*fit_id];
            for modifier in updates.incoming.iter() {
                self.apply_mod_to_fits(*modifier, &tgt_fit_ids);
            }
        }
        if !updates.outgoing.is_empty() {
            let tgt_fit_ids = sol_view
                .fits
                .iter_fit_ids()
                .map(|v| *v)
                .filter(|v| v != fit_id)
                .collect();
            for modifier in updates.outgoing.iter() {
                self.apply_mod_to_fits(*modifier, &tgt_fit_ids);
            }
        }
        updates
    }
    pub(in crate::sol::svc::svce_calc) fn unreg_fleet_for_fit(
        &mut self,
        sol_view: &SolView,
        fleet: &SolFleet,
        fit_id: &SolFitId,
    ) -> SolFleetUpdates {
        let updates = self.get_fleet_updates(fleet, fit_id);
        if !updates.incoming.is_empty() {
            let tgt_fit_ids = vec![*fit_id];
            for modifier in updates.incoming.iter() {
                self.unapply_mod_from_fits(modifier, &tgt_fit_ids);
            }
        }
        if !updates.outgoing.is_empty() {
            let tgt_fit_ids = sol_view
                .fits
                .iter_fit_ids()
                .map(|v| *v)
                .filter(|v| v != fit_id)
                .collect();
            for modifier in updates.outgoing.iter() {
                self.unapply_mod_from_fits(modifier, &tgt_fit_ids);
            }
        }
        updates
    }
    pub(in crate::sol::svc::svce_calc) fn reg_mod(
        &mut self,
        sol_view: &SolView,
        mod_item: &SolItem,
        modifier: SolAttrMod,
    ) {
        self.mods.add_entry(modifier.affector_item_id, modifier);
        match modifier.mod_type {
            SolModType::SystemWide => {
                self.sw_mods.insert(modifier);
                ()
            }
            SolModType::Fleet => {
                if let Some(fit_id) = mod_item.get_fit_id() {
                    self.mods_fleet_fit.add_entry(fit_id, modifier);
                }
            }
            _ => (),
        };
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolModDomain::Item => self.mods_direct.add_entry(modifier.affector_item_id, modifier),
                SolModDomain::Other => self.mods_other.add_entry(modifier.affector_item_id, modifier),
                _ => (),
            },
            _ => (),
        }
        let mut tgt_fit_ids = Vec::new();
        match modifier.mod_type {
            SolModType::Local | SolModType::FitWide => {
                if let Some(tgt_fit_id) = mod_item.get_fit_id() {
                    tgt_fit_ids.push(tgt_fit_id);
                }
            }
            SolModType::SystemWide => tgt_fit_ids.extend(sol_view.fits.iter_fit_ids()),
            SolModType::Projected => (),
            SolModType::Targeted => (),
            SolModType::Fleet => {
                if let Some(src_fit_id) = mod_item.get_fit_id() {
                    let src_fit = sol_view.fits.get_fit(&src_fit_id).unwrap();
                    match src_fit.fleet {
                        Some(fleet_id) => {
                            let fleet = sol_view.fleets.get_fleet(&fleet_id).unwrap();
                            tgt_fit_ids.extend(fleet.iter_fits());
                        }
                        None => tgt_fit_ids.push(src_fit_id),
                    }
                }
            }
        }
        self.apply_mod_to_fits(modifier, &tgt_fit_ids);
    }
    pub(in crate::sol::svc::svce_calc) fn unreg_mod(
        &mut self,
        sol_view: &SolView,
        mod_item: &SolItem,
        modifier: &SolAttrMod,
    ) {
        self.mods.remove_entry(&modifier.affector_item_id, &modifier);
        match modifier.mod_type {
            SolModType::SystemWide => {
                self.sw_mods.remove(modifier);
                ()
            }
            SolModType::Fleet => {
                if let Some(fit_id) = mod_item.get_fit_id() {
                    self.mods_fleet_fit.remove_entry(&fit_id, &modifier);
                }
            }
            _ => (),
        };
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolModDomain::Item => self.mods_direct.remove_entry(&modifier.affector_item_id, &modifier),
                SolModDomain::Other => self.mods_other.remove_entry(&modifier.affector_item_id, &modifier),
                _ => (),
            },
            _ => (),
        }
        let mut tgt_fit_ids = Vec::new();
        match modifier.mod_type {
            SolModType::Local | SolModType::FitWide => {
                if let Some(tgt_fit_id) = mod_item.get_fit_id() {
                    tgt_fit_ids.push(tgt_fit_id);
                }
            }
            SolModType::SystemWide => tgt_fit_ids.extend(sol_view.fits.iter_fit_ids()),
            SolModType::Projected => (),
            SolModType::Targeted => (),
            SolModType::Fleet => {
                if let Some(src_fit_id) = mod_item.get_fit_id() {
                    let src_fit = sol_view.fits.get_fit(&src_fit_id).unwrap();
                    match src_fit.fleet {
                        Some(fleet_id) => {
                            let fleet = sol_view.fleets.get_fleet(&fleet_id).unwrap();
                            tgt_fit_ids.extend(fleet.iter_fits());
                        }
                        None => tgt_fit_ids.push(src_fit_id),
                    }
                }
            }
        }
        self.unapply_mod_from_fits(modifier, &tgt_fit_ids);
    }
    pub(in crate::sol::svc::svce_calc) fn add_mod_tgt(
        &mut self,
        mod_item: &SolItem,
        modifier: SolAttrMod,
        tgt_item: &SolItem,
    ) -> bool {
        if matches!(modifier.mod_type, SolModType::Targeted) {
            return self.apply_mod_to_item(modifier, tgt_item);
        }
        match (mod_item, tgt_item) {
            (SolItem::ProjEffect(_), SolItem::Ship(ship)) if !is_mod_direct_everything(&modifier) => {
                self.apply_mod_to_fits(modifier, &vec![ship.fit_id]);
                return true;
            }
            (SolItem::ProjEffect(_), SolItem::Structure(structure)) if !is_mod_direct_everything(&modifier) => {
                self.apply_mod_to_fits(modifier, &vec![structure.fit_id]);
                return true;
            }
            (SolItem::ProjEffect(_), _) => return self.apply_mod_to_item(modifier, tgt_item),
            _ => false,
        }
    }
    pub(in crate::sol::svc::svce_calc) fn rm_mod_tgt(
        &mut self,
        mod_item: &SolItem,
        modifier: &SolAttrMod,
        tgt_item: &SolItem,
    ) -> bool {
        if matches!(modifier.mod_type, SolModType::Targeted) {
            return self.unapply_mod_from_item(modifier, tgt_item);
        }
        match (mod_item, tgt_item) {
            (SolItem::ProjEffect(_), SolItem::Ship(ship)) if !is_mod_direct_everything(&modifier) => {
                self.unapply_mod_from_fits(modifier, &vec![ship.fit_id]);
                true
            }
            (SolItem::ProjEffect(_), SolItem::Structure(structure)) if !is_mod_direct_everything(&modifier) => {
                self.unapply_mod_from_fits(modifier, &vec![structure.fit_id]);
                true
            }
            (SolItem::ProjEffect(_), _) => self.unapply_mod_from_item(modifier, tgt_item),
            _ => false,
        }
    }
    // Private methods
    fn apply_mod_to_fits(&mut self, modifier: SolAttrMod, tgt_fit_ids: &Vec<SolFitId>) {
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolModDomain::Everything => {
                    for tgt_fit_id in tgt_fit_ids.iter() {
                        self.mods_buff_all.add_entry(*tgt_fit_id, modifier);
                    }
                }
                SolModDomain::Char => {
                    for tgt_fit_id in tgt_fit_ids.iter() {
                        self.mods_toploc
                            .add_entry((*tgt_fit_id, SolLocType::Character), modifier);
                    }
                }
                SolModDomain::Ship => {
                    for tgt_fit_id in tgt_fit_ids.iter() {
                        self.mods_toploc.add_entry((*tgt_fit_id, SolLocType::Ship), modifier);
                    }
                }
                SolModDomain::Structure => {
                    for tgt_fit_id in tgt_fit_ids.iter() {
                        self.mods_toploc
                            .add_entry((*tgt_fit_id, SolLocType::Structure), modifier);
                    }
                }
                _ => (),
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolModDomain::Everything => {
                    for tgt_fit_id in tgt_fit_ids.iter() {
                        self.mods_parloc.add_entry((*tgt_fit_id, SolLocType::Ship), modifier);
                        self.mods_parloc
                            .add_entry((*tgt_fit_id, SolLocType::Structure), modifier);
                    }
                }
                _ => {
                    if let Ok(loc) = dom.try_into() {
                        for tgt_fit_id in tgt_fit_ids.iter() {
                            self.mods_parloc.add_entry((*tgt_fit_id, loc), modifier);
                        }
                    }
                }
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolModDomain::Everything => {
                    for tgt_fit_id in tgt_fit_ids.iter() {
                        self.mods_parloc_grp
                            .add_entry((*tgt_fit_id, SolLocType::Ship, grp_id), modifier);
                        self.mods_parloc_grp
                            .add_entry((*tgt_fit_id, SolLocType::Structure, grp_id), modifier);
                    }
                }
                _ => {
                    if let Ok(loc) = dom.try_into() {
                        for tgt_fit_id in tgt_fit_ids.iter() {
                            self.mods_parloc_grp.add_entry((*tgt_fit_id, loc, grp_id), modifier);
                        }
                    }
                }
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolModDomain::Everything => {
                    for tgt_fit_id in tgt_fit_ids.iter() {
                        self.mods_parloc_srq
                            .add_entry((*tgt_fit_id, SolLocType::Ship, srq_id), modifier);
                        self.mods_parloc_srq
                            .add_entry((*tgt_fit_id, SolLocType::Structure, srq_id), modifier);
                    }
                }
                _ => {
                    if let Ok(loc) = dom.try_into() {
                        for tgt_fit_id in tgt_fit_ids.iter() {
                            self.mods_parloc_srq.add_entry((*tgt_fit_id, loc, srq_id), modifier);
                        }
                    }
                }
            },
            SolAffecteeFilter::OwnSrq(srq_id) => {
                for tgt_fit_id in tgt_fit_ids.iter() {
                    self.mods_own_srq.add_entry((*tgt_fit_id, srq_id), modifier);
                }
            }
        }
    }
    fn unapply_mod_from_fits(&mut self, modifier: &SolAttrMod, tgt_fit_ids: &Vec<SolFitId>) {
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolModDomain::Everything => {
                    for tgt_fit_id in tgt_fit_ids.iter() {
                        self.mods_buff_all.remove_entry(tgt_fit_id, &modifier);
                    }
                }
                SolModDomain::Char => {
                    for tgt_fit_id in tgt_fit_ids.iter() {
                        self.mods_toploc
                            .remove_entry(&(*tgt_fit_id, SolLocType::Character), &modifier);
                    }
                }
                SolModDomain::Ship => {
                    for tgt_fit_id in tgt_fit_ids.iter() {
                        self.mods_toploc
                            .remove_entry(&(*tgt_fit_id, SolLocType::Ship), &modifier);
                    }
                }
                SolModDomain::Structure => {
                    for tgt_fit_id in tgt_fit_ids.iter() {
                        self.mods_toploc
                            .remove_entry(&(*tgt_fit_id, SolLocType::Structure), &modifier);
                    }
                }
                _ => (),
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolModDomain::Everything => {
                    for tgt_fit_id in tgt_fit_ids.iter() {
                        self.mods_parloc
                            .remove_entry(&(*tgt_fit_id, SolLocType::Ship), &modifier);
                        self.mods_parloc
                            .remove_entry(&(*tgt_fit_id, SolLocType::Structure), &modifier);
                    }
                }
                _ => {
                    if let Ok(loc) = dom.try_into() {
                        for tgt_fit_id in tgt_fit_ids.iter() {
                            self.mods_parloc.remove_entry(&(*tgt_fit_id, loc), &modifier);
                        }
                    }
                }
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolModDomain::Everything => {
                    for tgt_fit_id in tgt_fit_ids.iter() {
                        self.mods_parloc_grp
                            .remove_entry(&(*tgt_fit_id, SolLocType::Ship, grp_id), &modifier);
                        self.mods_parloc_grp
                            .remove_entry(&(*tgt_fit_id, SolLocType::Structure, grp_id), &modifier);
                    }
                }
                _ => {
                    if let Ok(loc) = dom.try_into() {
                        for tgt_fit_id in tgt_fit_ids.iter() {
                            self.mods_parloc_grp
                                .remove_entry(&(*tgt_fit_id, loc, grp_id), &modifier);
                        }
                    }
                }
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolModDomain::Everything => {
                    for tgt_fit_id in tgt_fit_ids.iter() {
                        self.mods_parloc_srq
                            .remove_entry(&(*tgt_fit_id, SolLocType::Ship, srq_id), &modifier);
                        self.mods_parloc_srq
                            .remove_entry(&(*tgt_fit_id, SolLocType::Structure, srq_id), &modifier);
                    }
                }
                _ => {
                    if let Ok(loc) = dom.try_into() {
                        for tgt_fit_id in tgt_fit_ids.iter() {
                            self.mods_parloc_srq
                                .remove_entry(&(*tgt_fit_id, loc, srq_id), &modifier);
                        }
                    }
                }
            },
            SolAffecteeFilter::OwnSrq(srq) => {
                for tgt_fit_id in tgt_fit_ids.iter() {
                    self.mods_own_srq.remove_entry(&(*tgt_fit_id, srq), &modifier);
                }
            }
        }
    }
    fn apply_mod_to_item(&mut self, modifier: SolAttrMod, tgt_item: &SolItem) -> bool {
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolModDomain::Everything if tgt_item.is_buff_modifiable() => {
                    self.mods_direct.add_entry(tgt_item.get_id(), modifier);
                    true
                }
                SolModDomain::Target if tgt_item.is_targetable() => {
                    // Could do parent location container here, but it's not really needed, since
                    // there is no scenario where modifier needs to target item with it being absent
                    self.mods_direct.add_entry(tgt_item.get_id(), modifier);
                    true
                }
                _ => false,
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolModDomain::Target if tgt_item.is_targetable() => match tgt_item {
                    SolItem::Ship(_) => {
                        if let Some(tgt_fit_id) = tgt_item.get_fit_id() {
                            self.mods_parloc.add_entry((tgt_fit_id, SolLocType::Ship), modifier);
                            true
                        } else {
                            false
                        }
                    }
                    SolItem::Structure(_) => {
                        if let Some(tgt_fit_id) = tgt_item.get_fit_id() {
                            self.mods_parloc
                                .add_entry((tgt_fit_id, SolLocType::Structure), modifier);
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolModDomain::Target if tgt_item.is_targetable() => match tgt_item {
                    SolItem::Ship(_) => {
                        if let Some(tgt_fit_id) = tgt_item.get_fit_id() {
                            self.mods_parloc_grp
                                .add_entry((tgt_fit_id, SolLocType::Ship, grp_id), modifier);
                            true
                        } else {
                            false
                        }
                    }
                    SolItem::Structure(_) => {
                        if let Some(tgt_fit_id) = tgt_item.get_fit_id() {
                            self.mods_parloc_grp
                                .add_entry((tgt_fit_id, SolLocType::Structure, grp_id), modifier);
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolModDomain::Target if tgt_item.is_targetable() => match tgt_item {
                    SolItem::Ship(_) => {
                        if let Some(tgt_fit_id) = tgt_item.get_fit_id() {
                            self.mods_parloc_srq
                                .add_entry((tgt_fit_id, SolLocType::Ship, srq_id), modifier);
                            true
                        } else {
                            false
                        }
                    }
                    SolItem::Structure(_) => {
                        if let Some(tgt_fit_id) = tgt_item.get_fit_id() {
                            self.mods_parloc_srq
                                .add_entry((tgt_fit_id, SolLocType::Structure, srq_id), modifier);
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                },
                _ => false,
            },
            _ => false,
        }
    }
    fn unapply_mod_from_item(&mut self, modifier: &SolAttrMod, tgt_item: &SolItem) -> bool {
        match modifier.affectee_filter {
            SolAffecteeFilter::Direct(dom) => match dom {
                SolModDomain::Everything if tgt_item.is_buff_modifiable() => {
                    self.mods_direct.remove_entry(&tgt_item.get_id(), modifier);
                    true
                }
                SolModDomain::Target if tgt_item.is_targetable() => {
                    // Could do parent location container here, but it's not really needed, since
                    // there is no scenario where modifier needs to target item with it being absent
                    self.mods_direct.remove_entry(&tgt_item.get_id(), modifier);
                    true
                }
                _ => false,
            },
            SolAffecteeFilter::Loc(dom) => match dom {
                SolModDomain::Target if tgt_item.is_targetable() => match tgt_item {
                    SolItem::Ship(_) => {
                        if let Some(tgt_fit_id) = tgt_item.get_fit_id() {
                            self.mods_parloc.remove_entry(&(tgt_fit_id, SolLocType::Ship), modifier);
                            true
                        } else {
                            false
                        }
                    }
                    SolItem::Structure(_) => {
                        if let Some(tgt_fit_id) = tgt_item.get_fit_id() {
                            self.mods_parloc
                                .remove_entry(&(tgt_fit_id, SolLocType::Structure), modifier);
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::LocGrp(dom, grp_id) => match dom {
                SolModDomain::Target if tgt_item.is_targetable() => match tgt_item {
                    SolItem::Ship(_) => {
                        if let Some(tgt_fit_id) = tgt_item.get_fit_id() {
                            self.mods_parloc_grp
                                .remove_entry(&(tgt_fit_id, SolLocType::Ship, grp_id), modifier);
                            true
                        } else {
                            false
                        }
                    }
                    SolItem::Structure(_) => {
                        if let Some(tgt_fit_id) = tgt_item.get_fit_id() {
                            self.mods_parloc_grp
                                .remove_entry(&(tgt_fit_id, SolLocType::Structure, grp_id), modifier);
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                },
                _ => false,
            },
            SolAffecteeFilter::LocSrq(dom, srq_id) => match dom {
                SolModDomain::Target if tgt_item.is_targetable() => match tgt_item {
                    SolItem::Ship(_) => {
                        if let Some(tgt_fit_id) = tgt_item.get_fit_id() {
                            self.mods_parloc_srq
                                .remove_entry(&(tgt_fit_id, SolLocType::Ship, srq_id), modifier);
                            true
                        } else {
                            false
                        }
                    }
                    SolItem::Structure(_) => {
                        if let Some(tgt_fit_id) = tgt_item.get_fit_id() {
                            self.mods_parloc_srq
                                .remove_entry(&(tgt_fit_id, SolLocType::Structure, srq_id), modifier);
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                },
                _ => false,
            },
            _ => false,
        }
    }
    fn get_fleet_updates(&self, fleet: &SolFleet, fit_id: &SolFitId) -> SolFleetUpdates {
        let mut updates = SolFleetUpdates::new();
        updates.outgoing.extend(self.mods_fleet_fit.get(fit_id));
        for fleet_fit_id in fleet.iter_fits() {
            if fleet_fit_id == fit_id {
                continue;
            }
            updates.incoming.extend(self.mods_fleet_fit.get(fleet_fit_id));
        }
        updates
    }
}

fn compare_loc_dom(loc: SolLocType, dom: SolModDomain) -> bool {
    match (loc, dom) {
        (SolLocType::Ship, SolModDomain::Everything) => true,
        (SolLocType::Ship, SolModDomain::Ship) => true,
        (SolLocType::Structure, SolModDomain::Everything) => true,
        (SolLocType::Structure, SolModDomain::Structure) => true,
        (SolLocType::Character, SolModDomain::Char) => true,
        _ => false,
    }
}

fn filter_and_extend<K: Eq + Hash>(
    vec: &mut Vec<SolAttrMod>,
    storage: &StMapSetL1<K, SolAttrMod>,
    key: &K,
    attr_id: &EAttrId,
) {
    vec.extend(storage.get(key).filter(|v| &v.affectee_attr_id == attr_id).map(|v| *v))
}

fn is_mod_direct_everything(modifier: &SolAttrMod) -> bool {
    match modifier.affectee_filter {
        SolAffecteeFilter::Direct(dom) => matches!(dom, SolModDomain::Everything),
        _ => false,
    }
}
