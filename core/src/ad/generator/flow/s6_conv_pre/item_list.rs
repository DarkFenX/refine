use crate::{
    ad::{AItemList, AItemListId},
    ed::EData,
    util::{RMap, RMapRSet, RSet},
};

pub(in crate::ad::generator::flow::s6_conv_pre) fn conv_item_lists(e_data: &EData) -> RMap<AItemListId, AItemList> {
    // Prepare helper data containers
    let mut types_by_grp = RMapRSet::new();
    for item in e_data.items.data.iter() {
        types_by_grp.add_entry(item.group_id, item.id);
    }
    let mut types_by_cat = RMapRSet::new();
    for group in e_data.groups.data.iter() {
        types_by_cat.extend_entries(group.category_id, types_by_grp.get(&group.id).copied());
    }
    // Convert item lists
    let mut result = RMap::with_capacity(e_data.item_lists.data.len());
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
        let item_list = AItemList {
            id: item_list.id.into(),
            item_ids: includes.difference(&excludes).copied().map(Into::into).collect(),
        };
        result.insert(item_list.id, item_list);
    }
    result
}
