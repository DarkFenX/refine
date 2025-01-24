use crate::{
    ad,
    defs::{EAttrId, EItemGrpId},
    ec,
    util::{StMap, StMapSetL1, StSet},
};

pub(in crate::adg) fn fill_extra_data(a_data: &mut ad::AData) {
    // Build data for item count in a group limit
    let grp_mutations = get_grp_mutations(a_data);
    let limited_fitted_grp_ids = get_item_grps_with_attr(&a_data.items, &grp_mutations, ec::attrs::MAX_GROUP_FITTED);
    let limited_online_grp_ids = get_item_grps_with_attr(&a_data.items, &grp_mutations, ec::attrs::MAX_GROUP_ONLINE);
    let limited_active_grp_ids = get_item_grps_with_attr(&a_data.items, &grp_mutations, ec::attrs::MAX_GROUP_ACTIVE);
    for a_item in a_data.items.iter_mut() {
        a_item.extras.fill(
            a_item.grp_id,
            a_item.cat_id,
            &a_item.attrs,
            &a_item.effect_datas,
            &limited_fitted_grp_ids,
            &limited_online_grp_ids,
            &limited_active_grp_ids,
        )
    }
}

fn get_grp_mutations(a_data: &ad::AData) -> StMapSetL1<EItemGrpId, EItemGrpId> {
    // Mutated items can potentially change their group ID during mutation; here, we compose a map
    // between base item group IDs and mutated item group IDs
    let mut mutations = StMapSetL1::new();
    let keyed_items = a_data.items.iter().map(|v| (v.id, v)).collect::<StMap<_, _>>();
    for a_muta in a_data.mutas.iter() {
        for (base_item_id, mutated_item_id) in a_muta.item_map.iter() {
            let base_grp_id = match keyed_items.get(base_item_id) {
                Some(base_item) => base_item.grp_id,
                None => continue,
            };
            let mutated_grp_id = match keyed_items.get(mutated_item_id) {
                Some(mutated_item) => mutated_item.grp_id,
                None => continue,
            };
            mutations.add_entry(base_grp_id, mutated_grp_id);
        }
    }
    mutations
}

fn get_item_grps_with_attr(
    a_items: &[ad::AItem],
    grp_mutations: &StMapSetL1<EItemGrpId, EItemGrpId>,
    attr_id: EAttrId,
) -> Vec<EItemGrpId> {
    let mut grp_ids = StSet::new();
    for a_item in a_items.iter() {
        if a_item.attrs.contains_key(&attr_id) {
            grp_ids.insert(a_item.grp_id);
            grp_ids.extend(grp_mutations.get(&a_item.grp_id).map(|v| *v))
        }
    }
    grp_ids.into_iter().collect()
}
