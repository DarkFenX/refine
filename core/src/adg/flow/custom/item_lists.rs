use crate::{
    ac,
    ad::{AData, AItemCatId, AItemList, AItemListId},
};

pub(in crate::adg::flow::custom) fn customize_item_lists(a_data: &mut AData) {
    make_and_add_item_list(a_data, ac::itemlists::SHIPS, &[ac::itemcats::SHIP]);
    make_and_add_item_list(
        a_data,
        ac::itemlists::SHIPS_DRONES_FIGHTERS_NPCS,
        &[ac::itemcats::SHIP, ac::itemcats::DRONE, ac::itemcats::FIGHTER],
    );
}

fn make_and_add_item_list(a_data: &mut AData, a_item_list_id: AItemListId, a_item_cats: &[AItemCatId]) {
    let a_item_list = AItemList {
        id: a_item_list_id,
        item_ids: a_data
            .items
            .values()
            .filter_map(|v| match a_item_cats.contains(&v.cat_id) {
                true => Some(v.id),
                false => None,
            })
            .collect(),
    };
    a_data.item_lists.insert(a_item_list_id, a_item_list);
}
