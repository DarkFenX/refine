use std::{convert::TryInto, hash::Hash};

use itertools::Itertools;

use crate::{
    defs::{EAttrId, EItemGrpId, EItemId, SsFitId, SsItemId},
    ss::{
        fit::SsFits,
        fleet::SsFleet,
        item::{SsItem, SsItems},
        svc::svce_calc::{SsAttrMod, SsFleetUpdates, SsLocType, SsModDomain, SsModTgtFilter, SsModType},
        SsView,
    },
    util::{StMapSetL1, StSet},
};

use super::LocsAct;

pub(in crate::ss::svc::svce_calc) struct SsModifierRegister {
    // Modifiers registered for an item
    // Contains: KeyedStorage<modifier item ID, modifiers>
    pub(super) mods: StMapSetL1<SsItemId, SsAttrMod>,
    // Modifiers which modify item directly
    // Contains: KeyedStorage<modifier item ID, modifiers>
    pub(super) mods_direct: StMapSetL1<SsItemId, SsAttrMod>,
    // All modifiers which modify top-level entities (via ship or character reference) are kept here
    // Contains: KeyedStorage<(target's fit ID, target's location type), modifiers>
    pub(super) mods_toploc: StMapSetL1<(SsFitId, SsLocType), SsAttrMod>,
    // Modifiers which modify 'other' domain are always stored here, regardless if they actually
    // modify something or not
    // Contains: KeyedStorage<modifier item ID, modifiers>
    pub(super) mods_other: StMapSetL1<SsItemId, SsAttrMod>,
    // Modifiers influencing all items belonging to certain fit and location type
    // Contains: KeyedStorage<(target's fit ID, target's location type), modifiers>
    pub(super) mods_parloc: StMapSetL1<(SsFitId, SsLocType), SsAttrMod>,
    // Modifiers influencing items belonging to certain fit, location and group
    // Contains: KeyedStorage<(target's fit ID, target's location, target's group ID), modifiers>
    pub(super) mods_parloc_grp: StMapSetL1<(SsFitId, SsLocType, EItemGrpId), SsAttrMod>,
    // Modifiers influencing items belonging to certain fit and location, and having certain skill requirement
    // Contains: KeyedStorage<(target's fit ID, target's location, target's skillreq type ID), modifiers>
    pub(super) mods_parloc_srq: StMapSetL1<(SsFitId, SsLocType, EItemId), SsAttrMod>,
    // Modifiers influencing owner-modifiable items belonging to certain fit and having certain skill requirement
    // Contains: KeyedStorage<(target's fit ID, target's skillreq type ID), modifiers>
    pub(super) mods_own_srq: StMapSetL1<(SsFitId, EItemId), SsAttrMod>,
    // Modifiers influencing all buff-modifiable items
    // Contains: KeyedStorage<target's fit ID, modifiers>
    pub(super) mods_buff_all: StMapSetL1<SsFitId, SsAttrMod>,
    // Fleet modifiers on a per-fit basis
    // Contains: KeyedStorage<source fit ID, modifiers>
    pub(super) mods_fleet_fit: StMapSetL1<SsFitId, SsAttrMod>,
    // System-wide modifiers
    pub(super) sw_mods: StSet<SsAttrMod>,
}
impl SsModifierRegister {
    pub(in crate::ss::svc::svce_calc) fn new() -> Self {
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
    pub(in crate::ss::svc::svce_calc) fn get_mods_for_tgt(
        &self,
        tgt_item: &SsItem,
        tgt_attr_id: &EAttrId,
        fits: &SsFits,
    ) -> Vec<SsAttrMod> {
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
            for loc_type in LocsAct::new(tgt_item, tgt_fit) {
                filter_and_extend(&mut mods, &self.mods_parloc, &(tgt_fit.id, loc_type), tgt_attr_id);
            }
        }
        if let (Some(tgt_fit), Ok(tgt_grp_id)) = (tgt_fit_opt, tgt_grp_id_res) {
            for loc_type in LocsAct::new(tgt_item, tgt_fit) {
                filter_and_extend(
                    &mut mods,
                    &self.mods_parloc_grp,
                    &(tgt_fit.id, loc_type, tgt_grp_id),
                    tgt_attr_id,
                );
            }
        }
        if let (Some(tgt_fit), Ok(tgt_srqs)) = (tgt_fit_opt, &tgt_srqs_res) {
            for loc_type in LocsAct::new(tgt_item, tgt_fit) {
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
    pub(in crate::ss::svc::svce_calc) fn get_mods_for_changed_location_owner(
        &mut self,
        item: &SsItem,
        items: &SsItems,
    ) -> Vec<SsAttrMod> {
        let mut mods = Vec::new();
        if let (Some(_), Some(loc_type)) = (item.get_fit_id(), item.get_root_loc_type()) {
            for (sub_item_id, sub_mods) in self.mods.iter() {
                if let Ok(_) = items.get_item(sub_item_id) {
                    // TODO: This should be refined/optimized. It should pick only modifiers which
                    // TODO: target fit of item being changed.
                    for sub_mod in sub_mods {
                        if match sub_mod.tgt_filter {
                            SsModTgtFilter::Loc(sub_dom) => compare_loc_dom(loc_type, sub_dom),
                            SsModTgtFilter::LocGrp(sub_dom, _) => compare_loc_dom(loc_type, sub_dom),
                            SsModTgtFilter::LocSrq(sub_dom, _) => compare_loc_dom(loc_type, sub_dom),
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
    pub(in crate::ss::svc::svce_calc) fn iter_mods_for_src(
        &self,
        src_item_id: &SsItemId,
    ) -> impl ExactSizeIterator<Item = &SsAttrMod> {
        self.mods.get(src_item_id)
    }
    // Modification methods
    pub(in crate::ss::svc::svce_calc) fn reg_fit(&mut self, fit_id: &SsFitId) {
        let sw_mods = self.sw_mods.iter().map(|v| *v).collect_vec();
        if !sw_mods.is_empty() {
            let tgt_fit_ids = vec![*fit_id];
            for modifier in sw_mods.iter() {
                self.apply_mod_to_fits(*modifier, &tgt_fit_ids);
            }
        }
    }
    pub(in crate::ss::svc::svce_calc) fn unreg_fit(&mut self, fit_id: &SsFitId) {
        let sw_mods = self.sw_mods.iter().map(|v| *v).collect_vec();
        if !sw_mods.is_empty() {
            let tgt_fit_ids = vec![*fit_id];
            for modifier in sw_mods.iter() {
                self.unapply_mod_from_fits(modifier, &tgt_fit_ids);
            }
        }
    }
    pub(in crate::ss::svc::svce_calc) fn reg_fleet_for_fit(
        &mut self,
        ss_view: &SsView,
        fleet: &SsFleet,
        fit_id: &SsFitId,
    ) -> SsFleetUpdates {
        let updates = self.get_fleet_updates(fleet, fit_id);
        if !updates.incoming.is_empty() {
            let tgt_fit_ids = vec![*fit_id];
            for ss_mod in updates.incoming.iter() {
                self.apply_mod_to_fits(*ss_mod, &tgt_fit_ids);
            }
        }
        if !updates.outgoing.is_empty() {
            let tgt_fit_ids = ss_view
                .fits
                .iter_fit_ids()
                .map(|v| *v)
                .filter(|v| v != fit_id)
                .collect();
            for ss_mod in updates.outgoing.iter() {
                self.apply_mod_to_fits(*ss_mod, &tgt_fit_ids);
            }
        }
        updates
    }
    pub(in crate::ss::svc::svce_calc) fn unreg_fleet_for_fit(
        &mut self,
        ss_view: &SsView,
        fleet: &SsFleet,
        fit_id: &SsFitId,
    ) -> SsFleetUpdates {
        let updates = self.get_fleet_updates(fleet, fit_id);
        if !updates.incoming.is_empty() {
            let tgt_fit_ids = vec![*fit_id];
            for ss_mod in updates.incoming.iter() {
                self.unapply_mod_from_fits(ss_mod, &tgt_fit_ids);
            }
        }
        if !updates.outgoing.is_empty() {
            let tgt_fit_ids = ss_view
                .fits
                .iter_fit_ids()
                .map(|v| *v)
                .filter(|v| v != fit_id)
                .collect();
            for ss_mod in updates.outgoing.iter() {
                self.unapply_mod_from_fits(ss_mod, &tgt_fit_ids);
            }
        }
        updates
    }
    pub(in crate::ss::svc::svce_calc) fn reg_mod(&mut self, ss_view: &SsView, mod_item: &SsItem, modifier: SsAttrMod) {
        self.mods.add_entry(modifier.src_item_id, modifier);
        match modifier.mod_type {
            SsModType::SystemWide => {
                self.sw_mods.insert(modifier);
                ()
            }
            SsModType::Fleet => {
                if let Some(fit_id) = mod_item.get_fit_id() {
                    self.mods_fleet_fit.add_entry(fit_id, modifier);
                }
            }
            _ => (),
        };
        match modifier.tgt_filter {
            SsModTgtFilter::Direct(dom) => match dom {
                SsModDomain::Item => self.mods_direct.add_entry(modifier.src_item_id, modifier),
                SsModDomain::Other => self.mods_other.add_entry(modifier.src_item_id, modifier),
                _ => (),
            },
            _ => (),
        }
        let mut tgt_fit_ids = Vec::new();
        match modifier.mod_type {
            SsModType::Local | SsModType::FitWide => {
                if let Some(tgt_fit_id) = mod_item.get_fit_id() {
                    tgt_fit_ids.push(tgt_fit_id);
                }
            }
            SsModType::SystemWide => tgt_fit_ids.extend(ss_view.fits.iter_fit_ids()),
            SsModType::Projected => (),
            SsModType::Targeted => (),
            SsModType::Fleet => {
                if let Some(src_fit_id) = mod_item.get_fit_id() {
                    let src_fit = ss_view.fits.get_fit(&src_fit_id).unwrap();
                    match src_fit.fleet {
                        Some(fleet_id) => {
                            let fleet = ss_view.fleets.get_fleet(&fleet_id).unwrap();
                            tgt_fit_ids.extend(fleet.iter_fits());
                        }
                        None => tgt_fit_ids.push(src_fit_id),
                    }
                }
            }
        }
        self.apply_mod_to_fits(modifier, &tgt_fit_ids);
    }
    pub(in crate::ss::svc::svce_calc) fn unreg_mod(
        &mut self,
        ss_view: &SsView,
        mod_item: &SsItem,
        modifier: &SsAttrMod,
    ) {
        self.mods.remove_entry(&modifier.src_item_id, &modifier);
        match modifier.mod_type {
            SsModType::SystemWide => {
                self.sw_mods.remove(modifier);
                ()
            }
            SsModType::Fleet => {
                if let Some(fit_id) = mod_item.get_fit_id() {
                    self.mods_fleet_fit.remove_entry(&fit_id, &modifier);
                }
            }
            _ => (),
        };
        match modifier.tgt_filter {
            SsModTgtFilter::Direct(dom) => match dom {
                SsModDomain::Item => self.mods_direct.remove_entry(&modifier.src_item_id, &modifier),
                SsModDomain::Other => self.mods_other.remove_entry(&modifier.src_item_id, &modifier),
                _ => (),
            },
            _ => (),
        }
        let mut tgt_fit_ids = Vec::new();
        match modifier.mod_type {
            SsModType::Local | SsModType::FitWide => {
                if let Some(tgt_fit_id) = mod_item.get_fit_id() {
                    tgt_fit_ids.push(tgt_fit_id);
                }
            }
            SsModType::SystemWide => tgt_fit_ids.extend(ss_view.fits.iter_fit_ids()),
            SsModType::Projected => (),
            SsModType::Targeted => (),
            SsModType::Fleet => {
                if let Some(src_fit_id) = mod_item.get_fit_id() {
                    let src_fit = ss_view.fits.get_fit(&src_fit_id).unwrap();
                    match src_fit.fleet {
                        Some(fleet_id) => {
                            let fleet = ss_view.fleets.get_fleet(&fleet_id).unwrap();
                            tgt_fit_ids.extend(fleet.iter_fits());
                        }
                        None => tgt_fit_ids.push(src_fit_id),
                    }
                }
            }
        }
        self.unapply_mod_from_fits(modifier, &tgt_fit_ids);
    }
    pub(in crate::ss::svc::svce_calc) fn add_mod_tgt(
        &mut self,
        mod_item: &SsItem,
        modifier: SsAttrMod,
        tgt_item: &SsItem,
    ) -> bool {
        if matches!(modifier.mod_type, SsModType::Targeted) {
            return self.apply_mod_to_item(modifier, tgt_item);
        }
        match (mod_item, tgt_item) {
            (SsItem::ProjEffect(_), SsItem::Ship(ship)) if !is_mod_direct_everything(&modifier) => {
                self.apply_mod_to_fits(modifier, &vec![ship.fit_id]);
                return true;
            }
            (SsItem::ProjEffect(_), SsItem::Structure(structure)) if !is_mod_direct_everything(&modifier) => {
                self.apply_mod_to_fits(modifier, &vec![structure.fit_id]);
                return true;
            }
            (SsItem::ProjEffect(_), _) => return self.apply_mod_to_item(modifier, tgt_item),
            _ => false,
        }
    }
    pub(in crate::ss::svc::svce_calc) fn rm_mod_tgt(
        &mut self,
        mod_item: &SsItem,
        modifier: &SsAttrMod,
        tgt_item: &SsItem,
    ) -> bool {
        if matches!(modifier.mod_type, SsModType::Targeted) {
            return self.unapply_mod_from_item(modifier, tgt_item);
        }
        match (mod_item, tgt_item) {
            (SsItem::ProjEffect(_), SsItem::Ship(ship)) if !is_mod_direct_everything(&modifier) => {
                self.unapply_mod_from_fits(modifier, &vec![ship.fit_id]);
                true
            }
            (SsItem::ProjEffect(_), SsItem::Structure(structure)) if !is_mod_direct_everything(&modifier) => {
                self.unapply_mod_from_fits(modifier, &vec![structure.fit_id]);
                true
            }
            (SsItem::ProjEffect(_), _) => self.unapply_mod_from_item(modifier, tgt_item),
            _ => false,
        }
    }
    // Private methods
    fn apply_mod_to_fits(&mut self, modifier: SsAttrMod, tgt_fit_ids: &Vec<SsFitId>) {
        match modifier.tgt_filter {
            SsModTgtFilter::Direct(dom) => match dom {
                SsModDomain::Everything => {
                    for tgt_fit_id in tgt_fit_ids.iter() {
                        self.mods_buff_all.add_entry(*tgt_fit_id, modifier);
                    }
                }
                SsModDomain::Char => {
                    for tgt_fit_id in tgt_fit_ids.iter() {
                        self.mods_toploc
                            .add_entry((*tgt_fit_id, SsLocType::Character), modifier);
                    }
                }
                SsModDomain::Ship => {
                    for tgt_fit_id in tgt_fit_ids.iter() {
                        self.mods_toploc.add_entry((*tgt_fit_id, SsLocType::Ship), modifier);
                    }
                }
                SsModDomain::Structure => {
                    for tgt_fit_id in tgt_fit_ids.iter() {
                        self.mods_toploc
                            .add_entry((*tgt_fit_id, SsLocType::Structure), modifier);
                    }
                }
                _ => (),
            },
            SsModTgtFilter::Loc(dom) => match dom {
                SsModDomain::Everything => {
                    for tgt_fit_id in tgt_fit_ids.iter() {
                        self.mods_parloc.add_entry((*tgt_fit_id, SsLocType::Ship), modifier);
                        self.mods_parloc
                            .add_entry((*tgt_fit_id, SsLocType::Structure), modifier);
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
            SsModTgtFilter::LocGrp(dom, grp_id) => match dom {
                SsModDomain::Everything => {
                    for tgt_fit_id in tgt_fit_ids.iter() {
                        self.mods_parloc_grp
                            .add_entry((*tgt_fit_id, SsLocType::Ship, grp_id), modifier);
                        self.mods_parloc_grp
                            .add_entry((*tgt_fit_id, SsLocType::Structure, grp_id), modifier);
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
            SsModTgtFilter::LocSrq(dom, srq_id) => match dom {
                SsModDomain::Everything => {
                    for tgt_fit_id in tgt_fit_ids.iter() {
                        self.mods_parloc_srq
                            .add_entry((*tgt_fit_id, SsLocType::Ship, srq_id), modifier);
                        self.mods_parloc_srq
                            .add_entry((*tgt_fit_id, SsLocType::Structure, srq_id), modifier);
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
            SsModTgtFilter::OwnSrq(srq_id) => {
                for tgt_fit_id in tgt_fit_ids.iter() {
                    self.mods_own_srq.add_entry((*tgt_fit_id, srq_id), modifier);
                }
            }
        }
    }
    fn unapply_mod_from_fits(&mut self, modifier: &SsAttrMod, tgt_fit_ids: &Vec<SsFitId>) {
        match modifier.tgt_filter {
            SsModTgtFilter::Direct(dom) => match dom {
                SsModDomain::Everything => {
                    for tgt_fit_id in tgt_fit_ids.iter() {
                        self.mods_buff_all.remove_entry(tgt_fit_id, &modifier);
                    }
                }
                SsModDomain::Char => {
                    for tgt_fit_id in tgt_fit_ids.iter() {
                        self.mods_toploc
                            .remove_entry(&(*tgt_fit_id, SsLocType::Character), &modifier);
                    }
                }
                SsModDomain::Ship => {
                    for tgt_fit_id in tgt_fit_ids.iter() {
                        self.mods_toploc
                            .remove_entry(&(*tgt_fit_id, SsLocType::Ship), &modifier);
                    }
                }
                SsModDomain::Structure => {
                    for tgt_fit_id in tgt_fit_ids.iter() {
                        self.mods_toploc
                            .remove_entry(&(*tgt_fit_id, SsLocType::Structure), &modifier);
                    }
                }
                _ => (),
            },
            SsModTgtFilter::Loc(dom) => match dom {
                SsModDomain::Everything => {
                    for tgt_fit_id in tgt_fit_ids.iter() {
                        self.mods_parloc
                            .remove_entry(&(*tgt_fit_id, SsLocType::Ship), &modifier);
                        self.mods_parloc
                            .remove_entry(&(*tgt_fit_id, SsLocType::Structure), &modifier);
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
            SsModTgtFilter::LocGrp(dom, grp_id) => match dom {
                SsModDomain::Everything => {
                    for tgt_fit_id in tgt_fit_ids.iter() {
                        self.mods_parloc_grp
                            .remove_entry(&(*tgt_fit_id, SsLocType::Ship, grp_id), &modifier);
                        self.mods_parloc_grp
                            .remove_entry(&(*tgt_fit_id, SsLocType::Structure, grp_id), &modifier);
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
            SsModTgtFilter::LocSrq(dom, srq_id) => match dom {
                SsModDomain::Everything => {
                    for tgt_fit_id in tgt_fit_ids.iter() {
                        self.mods_parloc_srq
                            .remove_entry(&(*tgt_fit_id, SsLocType::Ship, srq_id), &modifier);
                        self.mods_parloc_srq
                            .remove_entry(&(*tgt_fit_id, SsLocType::Structure, srq_id), &modifier);
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
            SsModTgtFilter::OwnSrq(srq) => {
                for tgt_fit_id in tgt_fit_ids.iter() {
                    self.mods_own_srq.remove_entry(&(*tgt_fit_id, srq), &modifier);
                }
            }
        }
    }
    fn apply_mod_to_item(&mut self, modifier: SsAttrMod, tgt_item: &SsItem) -> bool {
        match modifier.tgt_filter {
            SsModTgtFilter::Direct(dom) => match dom {
                SsModDomain::Everything if tgt_item.is_buff_modifiable() => {
                    self.mods_direct.add_entry(tgt_item.get_id(), modifier);
                    true
                }
                SsModDomain::Target if tgt_item.is_targetable() => {
                    // Could do parent location container here, but it's not really needed, since
                    // there is no scenario where modifier needs to target item with it being absent
                    self.mods_direct.add_entry(tgt_item.get_id(), modifier);
                    true
                }
                _ => false,
            },
            SsModTgtFilter::Loc(dom) => match dom {
                SsModDomain::Target if tgt_item.is_targetable() => match tgt_item {
                    SsItem::Ship(_) => {
                        if let Some(tgt_fit_id) = tgt_item.get_fit_id() {
                            self.mods_parloc.add_entry((tgt_fit_id, SsLocType::Ship), modifier);
                            true
                        } else {
                            false
                        }
                    }
                    SsItem::Structure(_) => {
                        if let Some(tgt_fit_id) = tgt_item.get_fit_id() {
                            self.mods_parloc.add_entry((tgt_fit_id, SsLocType::Structure), modifier);
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                },
                _ => false,
            },
            SsModTgtFilter::LocGrp(dom, grp_id) => match dom {
                SsModDomain::Target if tgt_item.is_targetable() => match tgt_item {
                    SsItem::Ship(_) => {
                        if let Some(tgt_fit_id) = tgt_item.get_fit_id() {
                            self.mods_parloc_grp
                                .add_entry((tgt_fit_id, SsLocType::Ship, grp_id), modifier);
                            true
                        } else {
                            false
                        }
                    }
                    SsItem::Structure(_) => {
                        if let Some(tgt_fit_id) = tgt_item.get_fit_id() {
                            self.mods_parloc_grp
                                .add_entry((tgt_fit_id, SsLocType::Structure, grp_id), modifier);
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                },
                _ => false,
            },
            SsModTgtFilter::LocSrq(dom, srq_id) => match dom {
                SsModDomain::Target if tgt_item.is_targetable() => match tgt_item {
                    SsItem::Ship(_) => {
                        if let Some(tgt_fit_id) = tgt_item.get_fit_id() {
                            self.mods_parloc_srq
                                .add_entry((tgt_fit_id, SsLocType::Ship, srq_id), modifier);
                            true
                        } else {
                            false
                        }
                    }
                    SsItem::Structure(_) => {
                        if let Some(tgt_fit_id) = tgt_item.get_fit_id() {
                            self.mods_parloc_srq
                                .add_entry((tgt_fit_id, SsLocType::Structure, srq_id), modifier);
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
    fn unapply_mod_from_item(&mut self, modifier: &SsAttrMod, tgt_item: &SsItem) -> bool {
        match modifier.tgt_filter {
            SsModTgtFilter::Direct(dom) => match dom {
                SsModDomain::Everything if tgt_item.is_buff_modifiable() => {
                    self.mods_direct.remove_entry(&tgt_item.get_id(), modifier);
                    true
                }
                SsModDomain::Target if tgt_item.is_targetable() => {
                    // Could do parent location container here, but it's not really needed, since
                    // there is no scenario where modifier needs to target item with it being absent
                    self.mods_direct.remove_entry(&tgt_item.get_id(), modifier);
                    true
                }
                _ => false,
            },
            SsModTgtFilter::Loc(dom) => match dom {
                SsModDomain::Target if tgt_item.is_targetable() => match tgt_item {
                    SsItem::Ship(_) => {
                        if let Some(tgt_fit_id) = tgt_item.get_fit_id() {
                            self.mods_parloc.remove_entry(&(tgt_fit_id, SsLocType::Ship), modifier);
                            true
                        } else {
                            false
                        }
                    }
                    SsItem::Structure(_) => {
                        if let Some(tgt_fit_id) = tgt_item.get_fit_id() {
                            self.mods_parloc
                                .remove_entry(&(tgt_fit_id, SsLocType::Structure), modifier);
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                },
                _ => false,
            },
            SsModTgtFilter::LocGrp(dom, grp_id) => match dom {
                SsModDomain::Target if tgt_item.is_targetable() => match tgt_item {
                    SsItem::Ship(_) => {
                        if let Some(tgt_fit_id) = tgt_item.get_fit_id() {
                            self.mods_parloc_grp
                                .remove_entry(&(tgt_fit_id, SsLocType::Ship, grp_id), modifier);
                            true
                        } else {
                            false
                        }
                    }
                    SsItem::Structure(_) => {
                        if let Some(tgt_fit_id) = tgt_item.get_fit_id() {
                            self.mods_parloc_grp
                                .remove_entry(&(tgt_fit_id, SsLocType::Structure, grp_id), modifier);
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                },
                _ => false,
            },
            SsModTgtFilter::LocSrq(dom, srq_id) => match dom {
                SsModDomain::Target if tgt_item.is_targetable() => match tgt_item {
                    SsItem::Ship(_) => {
                        if let Some(tgt_fit_id) = tgt_item.get_fit_id() {
                            self.mods_parloc_srq
                                .remove_entry(&(tgt_fit_id, SsLocType::Ship, srq_id), modifier);
                            true
                        } else {
                            false
                        }
                    }
                    SsItem::Structure(_) => {
                        if let Some(tgt_fit_id) = tgt_item.get_fit_id() {
                            self.mods_parloc_srq
                                .remove_entry(&(tgt_fit_id, SsLocType::Structure, srq_id), modifier);
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
    fn get_fleet_updates(&self, fleet: &SsFleet, fit_id: &SsFitId) -> SsFleetUpdates {
        let mut updates = SsFleetUpdates::new();
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

fn compare_loc_dom(loc: SsLocType, dom: SsModDomain) -> bool {
    match (loc, dom) {
        (SsLocType::Ship, SsModDomain::Everything) => true,
        (SsLocType::Ship, SsModDomain::Ship) => true,
        (SsLocType::Structure, SsModDomain::Everything) => true,
        (SsLocType::Structure, SsModDomain::Structure) => true,
        (SsLocType::Character, SsModDomain::Char) => true,
        _ => false,
    }
}

fn filter_and_extend<K: Eq + Hash>(
    vec: &mut Vec<SsAttrMod>,
    storage: &StMapSetL1<K, SsAttrMod>,
    key: &K,
    attr_id: &EAttrId,
) {
    vec.extend(storage.get(key).filter(|v| &v.tgt_attr_id == attr_id).map(|v| *v))
}

fn is_mod_direct_everything(modifier: &SsAttrMod) -> bool {
    match modifier.tgt_filter {
        SsModTgtFilter::Direct(dom) => matches!(dom, SsModDomain::Everything),
        _ => false,
    }
}
