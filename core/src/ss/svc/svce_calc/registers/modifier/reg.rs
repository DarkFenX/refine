use std::{collections::HashSet, convert::TryInto, hash::Hash};

use crate::{
    defs::{EAttrId, EItemGrpId, EItemId, SsFitId, SsItemId},
    ss::{
        fit::SsFits,
        item::{SsItem, SsItems},
        svc::svce_calc::{
            modifier::{SsAttrMod, SsModDomain, SsModTgtFilter, SsModType},
            SsLocType,
        },
        SsView,
    },
    util::KeyedStorage1L,
};

use super::iter_loc_act::LocsAct;

pub(in crate::ss::svc::svce_calc) struct ModifierRegister {
    // Modifiers registered for an item
    // Contains: KeyedStorage<modifier item ID, modifiers>
    mods: KeyedStorage1L<SsItemId, SsAttrMod>,
    // Modifiers which modify item directly
    // Contains: KeyedStorage<modifier item ID, modifiers>
    mods_direct: KeyedStorage1L<SsItemId, SsAttrMod>,
    // All modifiers which modify top-level entities (via ship or character reference) are kept here
    // Contains: KeyedStorage<(target's fit ID, target's location type), modifiers>
    mods_toploc: KeyedStorage1L<(SsFitId, SsLocType), SsAttrMod>,
    // Modifiers which modify 'other' domain are always stored here, regardless if they actually
    // modify something or not
    // Contains: KeyedStorage<modifier item ID, modifiers>
    mods_other: KeyedStorage1L<SsItemId, SsAttrMod>,
    // Modifiers influencing all items belonging to certain fit and location type
    // Contains: KeyedStorage<(target's fit ID, target's location type), modifiers>
    mods_parloc: KeyedStorage1L<(SsFitId, SsLocType), SsAttrMod>,
    // Modifiers influencing items belonging to certain fit, location and group
    // Contains: KeyedStorage<(target's fit ID, target's location, target's group ID), modifiers>
    mods_parloc_grp: KeyedStorage1L<(SsFitId, SsLocType, EItemGrpId), SsAttrMod>,
    // Modifiers influencing items belonging to certain fit and location, and having certain skill requirement
    // Contains: KeyedStorage<(target's fit ID, target's location, target's skillreq type ID), modifiers>
    mods_parloc_srq: KeyedStorage1L<(SsFitId, SsLocType, EItemId), SsAttrMod>,
    // Modifiers influencing owner-modifiable items belonging to certain fit and having certain skill requirement
    // Contains: KeyedStorage<(target's fit ID, target's skillreq type ID), modifiers>
    mods_own_srq: KeyedStorage1L<(SsFitId, EItemId), SsAttrMod>,
    // Modifiers influencing all buff-modifiable items
    // Contains: KeyedStorage<target's fit ID, modifiers>
    mods_buff_all: KeyedStorage1L<SsFitId, SsAttrMod>,
    // System-wide modifiers
    sw_mods: HashSet<SsAttrMod>,
}
impl ModifierRegister {
    pub(in crate::ss::svc::svce_calc) fn new() -> Self {
        Self {
            mods: KeyedStorage1L::new(),
            mods_direct: KeyedStorage1L::new(),
            mods_toploc: KeyedStorage1L::new(),
            mods_other: KeyedStorage1L::new(),
            mods_parloc: KeyedStorage1L::new(),
            mods_parloc_grp: KeyedStorage1L::new(),
            mods_parloc_srq: KeyedStorage1L::new(),
            mods_own_srq: KeyedStorage1L::new(),
            mods_buff_all: KeyedStorage1L::new(),
            sw_mods: HashSet::new(),
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
        if let (Some(fit_id), Some(loc_type)) = (item.get_fit_id(), item.get_root_loc_type()) {
            for (sub_item_id, sub_mods) in self.mods.iter() {
                if let Ok(sub_item) = items.get_item(sub_item_id) {
                    // Local modifications which come from this fit or system-wide modifications may
                    // be affected
                    if sub_item.get_fit_id() == Some(fit_id) || matches!(sub_item, SsItem::SwEffect(_)) {
                        for sub_mod in sub_mods.iter() {
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
        }
        mods
    }
    pub(in crate::ss::svc::svce_calc) fn iter_mods_for_src(
        &self,
        src_item_id: &SsItemId,
    ) -> impl Iterator<Item = &SsAttrMod> {
        self.mods.get(src_item_id).into_iter().flatten()
    }
    // Modification methods
    pub(in crate::ss::svc::svce_calc) fn reg_fit(&mut self, fit_id: &SsFitId) {
        let sw_mods = self.sw_mods.clone();
        for modifier in sw_mods.iter() {
            self.apply_mod_to_fits(*modifier, vec![*fit_id]);
        }
    }
    pub(in crate::ss::svc::svce_calc) fn unreg_fit(&mut self, fit_id: &SsFitId) {
        let sw_mods = self.sw_mods.clone();
        for modifier in sw_mods.iter() {
            self.unapply_mods_from_fits(modifier, vec![*fit_id]);
        }
    }
    pub(in crate::ss::svc::svce_calc) fn reg_mod(&mut self, ss_view: &SsView, mod_item: &SsItem, modifier: SsAttrMod) {
        self.mods.add_entry(modifier.src_item_id, modifier);
        if matches!(modifier.mod_type, SsModType::SystemWide) {
            self.sw_mods.insert(modifier);
        }
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
            SsModType::Fleet => (),
        }
        self.apply_mod_to_fits(modifier, tgt_fit_ids);
    }
    pub(in crate::ss::svc::svce_calc) fn unreg_mod(
        &mut self,
        ss_view: &SsView,
        mod_item: &SsItem,
        modifier: &SsAttrMod,
    ) {
        self.mods.remove_entry(&modifier.src_item_id, &modifier);
        if matches!(modifier.mod_type, SsModType::SystemWide) {
            self.sw_mods.remove(modifier);
        }
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
            SsModType::Fleet => (),
        }
        self.unapply_mods_from_fits(modifier, tgt_fit_ids);
    }
    pub(in crate::ss::svc::svce_calc) fn add_mod_tgt(
        &mut self,
        mod_item: &SsItem,
        modifier: SsAttrMod,
        tgt_item: &SsItem,
    ) -> bool {
        match mod_item {
            SsItem::ProjEffect(_) => match tgt_item {
                SsItem::Ship(ship) => {
                    self.apply_mod_to_fits(modifier, vec![ship.fit_id]);
                    true
                }
                _ => false,
            },
            _ => false,
        }
    }
    pub(in crate::ss::svc::svce_calc) fn rm_mod_tgt(
        &mut self,
        mod_item: &SsItem,
        modifier: &SsAttrMod,
        tgt_item: &SsItem,
    ) -> bool {
        match mod_item {
            SsItem::ProjEffect(_) => match tgt_item {
                SsItem::Ship(ship) => {
                    self.unapply_mods_from_fits(modifier, vec![ship.fit_id]);
                    true
                }
                _ => false,
            },
            _ => false,
        }
    }
    // Private methods
    fn apply_mod_to_fits(&mut self, modifier: SsAttrMod, tgt_fit_ids: Vec<SsFitId>) {
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
    fn unapply_mods_from_fits(&mut self, modifier: &SsAttrMod, tgt_fit_ids: Vec<SsFitId>) {
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
    storage: &KeyedStorage1L<K, SsAttrMod>,
    key: &K,
    attr_id: &EAttrId,
) {
    match storage.get(key) {
        Some(v) => vec.extend(v.iter().filter(|v| &v.tgt_attr_id == attr_id).map(|v| v.clone())),
        _ => (),
    }
}
