use crate::{
    ad::{ADataGenerator, AItemId, AItemList, AItemListId, AItemLists},
    util::{RMap, RMapRSet, RSet},
};

impl ADataGenerator {
    pub(super) fn conv_item_lists(&mut self) {
        // Prepare helper data containers
        let mut types_by_grp = RMapRSet::new();
        for item in self.e_data.items.data.iter() {
            types_by_grp.add_entry(item.group_id, item.id);
        }
        let mut types_by_cat = RMapRSet::new();
        for group in self.e_data.groups.data.iter() {
            types_by_cat.extend_entries(group.category_id, types_by_grp.get(&group.id).copied());
        }
        // Convert item lists
        let mut a_item_lists = RMap::with_capacity(self.e_data.item_lists.data.len());
        for e_item_list in &self.e_data.item_lists.data {
            let mut includes = RSet::new();
            includes.extend(e_item_list.included_item_ids.iter().copied());
            for included_grp_id in e_item_list.included_grp_ids.iter() {
                includes.extend(types_by_grp.get(included_grp_id).copied());
            }
            for included_cat_id in e_item_list.included_cat_ids.iter() {
                includes.extend(types_by_cat.get(included_cat_id).copied());
            }
            let mut excludes = RSet::new();
            excludes.extend(e_item_list.excluded_item_ids.iter().copied());
            for excluded_grp_id in e_item_list.excluded_grp_ids.iter() {
                excludes.extend(types_by_grp.get(excluded_grp_id).copied());
            }
            for excluded_cat_id in e_item_list.excluded_cat_ids.iter() {
                excludes.extend(types_by_cat.get(excluded_cat_id).copied());
            }
            let a_item_list = AItemList {
                id: AItemListId::from_eid(e_item_list.id),
                item_ids: includes.difference(&excludes).copied().map(AItemId::from_eid).collect(),
            };
            a_item_lists.insert(a_item_list.id, a_item_list);
        }
        self.a_data.item_lists = AItemLists { data: a_item_lists };
    }
}
