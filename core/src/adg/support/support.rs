use crate::{
    ac,
    ad::{AEffectBuffInfo, AItemListId},
    adg::GItemList,
    ed,
    nd::N_EFFECTS,
    util::{RMap, RMapRSet, RSet},
};

/// Container for auxiliary data.
pub(in crate::adg) struct GSupport {
    pub(in crate::adg) item_lists: RMap<AItemListId, GItemList>,
    pub(in crate::adg) grp_cat_map: RMap<ed::EItemGrpId, ed::EItemCatId>,
    pub(in crate::adg) attr_unit_map: RMap<ed::EAttrId, ed::EAttrUnitId>,
    pub(in crate::adg) eff_buff_map: RMap<ed::EEffectId, AEffectBuffInfo>,
    // Buffs which can be used, but are not attached to any effect
    pub(in crate::adg) standalone_buffs: Vec<AEffectBuffInfo>,
}
impl GSupport {
    pub(in crate::adg) fn new() -> Self {
        Self {
            item_lists: RMap::new(),
            grp_cat_map: RMap::new(),
            attr_unit_map: RMap::new(),
            eff_buff_map: RMap::new(),
            standalone_buffs: Vec::new(),
        }
    }
    pub(in crate::adg) fn fill(&mut self, e_data: &ed::EData) {
        self.fill_grp_cat_map(e_data);
        self.create_item_lists(e_data);
        self.fill_attr_units(e_data);
        self.fill_buff_data();
    }
    fn create_item_lists(&mut self, e_data: &ed::EData) {
        // Prepare data
        let mut types_by_grp = RMapRSet::new();
        for item in e_data.items.data.iter() {
            types_by_grp.add_entry(item.group_id, item.id);
        }
        let mut types_by_cat = RMapRSet::new();
        for group in e_data.groups.data.iter() {
            types_by_cat.extend_entries(group.category_id, types_by_grp.get(&group.id).copied());
        }
        // Convert EVE item lists
        for item_list in &e_data.item_lists.data {
            let mut includes = RSet::new();
            includes.extend(item_list.included_item_ids.iter().copied());
            for included_grp_id in item_list.included_grp_ids.iter() {
                includes.extend(types_by_grp.get(included_grp_id).copied());
            }
            for included_cat_id in item_list.included_cat_ids.iter() {
                includes.extend(types_by_cat.get(included_cat_id).copied());
            }
            let mut excludes = RSet::new();
            excludes.extend(item_list.excluded_item_ids.iter().copied());
            for excluded_grp_id in item_list.excluded_grp_ids.iter() {
                excludes.extend(types_by_grp.get(excluded_grp_id).copied());
            }
            for excluded_cat_id in item_list.excluded_cat_ids.iter() {
                excludes.extend(types_by_cat.get(excluded_cat_id).copied());
            }
            let item_list = GItemList {
                id: AItemListId::Eve(item_list.id),
                item_ids: includes.difference(&excludes).copied().collect(),
            };
            self.item_lists.insert(item_list.id, item_list);
        }
        // Add custom type lists
        let item_list = make_item_list(&types_by_cat, ac::itemlists::SHIPS, &[ac::itemcats::SHIP]);
        self.item_lists.insert(item_list.id, item_list);
        let item_list = make_item_list(
            &types_by_cat,
            ac::itemlists::SHIPS_DRONES_FIGHTERS_NPCS,
            &[ac::itemcats::SHIP, ac::itemcats::DRONE, ac::itemcats::FIGHTER],
        );
        self.item_lists.insert(item_list.id, item_list);
    }
    fn fill_grp_cat_map(&mut self, e_data: &ed::EData) {
        for grp in e_data.groups.data.iter() {
            self.grp_cat_map.insert(grp.id, grp.category_id);
        }
    }
    fn fill_buff_data(&mut self) {
        for n_effect in N_EFFECTS.iter() {
            if let Some(buff_info) = &n_effect.adg_buff_info
                && let Some(e_effect_id) = n_effect.eid
            {
                self.eff_buff_map.insert(e_effect_id, buff_info.clone());
            }
            if let Some(effect_maker) = n_effect.adg_make_effect_fn
                && let Some(buff_info) = effect_maker().buff_info
            {
                self.standalone_buffs.push(buff_info);
            }
        }
    }
    fn fill_attr_units(&mut self, e_data: &ed::EData) {
        for attr in e_data.attrs.data.iter() {
            if let Some(unit) = attr.unit_id {
                self.attr_unit_map.insert(attr.id, unit);
            }
        }
    }
}

fn make_item_list(
    cat_to_item_map: &RMapRSet<ed::EItemCatId, ed::EItemId>,
    item_list_id: AItemListId,
    include_cats: &[ed::EItemCatId],
) -> GItemList {
    let mut item_list = GItemList {
        id: item_list_id,
        item_ids: RSet::new(),
    };
    for item_cat in include_cats {
        item_list.item_ids.extend(cat_to_item_map.get(item_cat).copied());
    }
    item_list
}
