use crate::{
    ac, ad,
    adg::GSupport,
    util::{RMap, RMapRSet, RSet},
};

pub(in crate::adg) fn fill_extra_data(a_data: &mut ad::AData, g_supp: &GSupport) {
    // Data for item count in a group limit - need to do it here for efficiency, and to take into
    // account that mutated item can have the limit even if raw mutated type has no such limit
    let grp_mutations = get_grp_mutations(a_data);
    let limited_fitted_grp_ids = get_item_grps_with_attr(&a_data.items, &grp_mutations, ac::attrs::MAX_GROUP_FITTED);
    let limited_online_grp_ids = get_item_grps_with_attr(&a_data.items, &grp_mutations, ac::attrs::MAX_GROUP_ONLINE);
    let limited_active_grp_ids = get_item_grps_with_attr(&a_data.items, &grp_mutations, ac::attrs::MAX_GROUP_ACTIVE);
    // The rest
    for a_item in a_data.items.values_mut() {
        a_item.extras.fill(
            a_item.id,
            a_item.grp_id,
            a_item.cat_id,
            &a_item.attrs,
            &a_item.effect_datas,
            &a_item.srqs,
            &a_data.effects,
            &g_supp.rendered_type_lists,
            &limited_fitted_grp_ids,
            &limited_online_grp_ids,
            &limited_active_grp_ids,
        )
    }
}

fn get_grp_mutations(a_data: &ad::AData) -> RMapRSet<ad::AItemGrpId, ad::AItemGrpId> {
    // Mutated items can potentially change their group ID during mutation; here, we compose a map
    // between base item group IDs and mutated item group IDs
    let mut mutations = RMapRSet::new();
    for a_muta in a_data.mutas.values() {
        for (base_item_id, mutated_item_id) in a_muta.item_map.iter() {
            let base_grp_id = match a_data.items.get(base_item_id) {
                Some(base_item) => base_item.grp_id,
                None => continue,
            };
            let mutated_grp_id = match a_data.items.get(mutated_item_id) {
                Some(mutated_item) => mutated_item.grp_id,
                None => continue,
            };
            mutations.add_entry(base_grp_id, mutated_grp_id);
        }
    }
    mutations
}

fn get_item_grps_with_attr(
    a_items: &RMap<ad::AItemId, ad::AItem>,
    grp_mutations: &RMapRSet<ad::AItemGrpId, ad::AItemGrpId>,
    attr_id: ad::AAttrId,
) -> RSet<ad::AItemGrpId> {
    let mut grp_ids = RSet::new();
    for a_item in a_items.values() {
        if a_item.attrs.contains_key(&attr_id) {
            grp_ids.insert(a_item.grp_id);
            grp_ids.extend(grp_mutations.get(&a_item.grp_id).copied())
        }
    }
    grp_ids
}
