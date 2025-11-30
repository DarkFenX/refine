use crate::{
    ac,
    ad::{AData, AItemCatId, AItemList, AItemListId},
};

pub(in crate::adg::flow::s7_custom) fn customize_item_lists(a_data: &mut AData) {
    mk_list_by_cats(a_data, ac::itemlists::SHIPS, &[ac::itemcats::SHIP]);
    mk_list_by_cats(
        a_data,
        ac::itemlists::SHIPS_DRONES_FIGHTERS_NPCS,
        &[ac::itemcats::SHIP, ac::itemcats::DRONE, ac::itemcats::FIGHTER],
    );
    mk_capitals_freighters_list(a_data);
}

fn mk_list_by_cats(a_data: &mut AData, a_item_list_id: AItemListId, a_item_cats: &[AItemCatId]) {
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

fn mk_capitals_freighters_list(a_data: &mut AData) {
    let a_item_list = AItemList {
        id: ac::itemlists::CAPITALS_FREIGHTERS,
        item_ids: a_data
            .items
            .values()
            .filter_map(|v| {
                match v.srqs.contains_key(&ac::items::CAPITAL_SHIPS)
                    || [ac::itemgrps::FREIGHTER, ac::itemgrps::JUMP_FREIGHTER].contains(&v.grp_id)
                {
                    true => Some(v.id),
                    false => None,
                }
            })
            .collect(),
    };
    a_data.item_lists.insert(a_item_list.id, a_item_list);
}
