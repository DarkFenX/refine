use crate::{
    ad::{AData, AEveItemListId, AItemListId},
    adg::GSupport,
    nd::{N_EFFECT_MAP, NEffectProjecteeFilter},
    util::RSet,
};

pub(in crate::adg::flow::conv_post) fn fill_effect_projectee_filters(a_data: &mut AData, g_supp: &GSupport) {
    for a_item in a_data.items.values_mut() {
        for (a_effect_id, a_effect_data) in a_item.effect_datas.iter_mut() {
            if let Some(n_effect) = N_EFFECT_MAP.get(a_effect_id)
                && let Some(n_projectee_filter) = n_effect.hc.projectee_filter
            {
                let a_item_list_id = match n_projectee_filter {
                    NEffectProjecteeFilter::ItemList(a_item_list_id) => a_item_list_id,
                    NEffectProjecteeFilter::ItemListAttr(a_attr_id) => match a_item.attrs.get(&a_attr_id) {
                        Some(&a_item_list_id) => AItemListId::Eve(a_item_list_id.round() as AEveItemListId),
                        None => continue,
                    },
                };
                let item_set = match g_supp.item_lists.get(&a_item_list_id) {
                    Some(g_item_list) => g_item_list.item_ids.clone(),
                    None => RSet::new(),
                };
                a_effect_data.projectee_filter = Some(item_set);
            }
        }
    }
}
