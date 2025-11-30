use crate::{
    ad::{AData, AEveItemListId, AItemListId},
    nd::{N_EFFECT_MAP, NEffectProjecteeFilter},
};

pub(in crate::adg::flow::s8_conv_post) fn fill_effect_projectee_filters(a_data: &mut AData) {
    for a_item in a_data.items.values_mut() {
        for (a_effect_id, a_effect_data) in a_item.effect_datas.iter_mut() {
            if let Some(n_effect) = N_EFFECT_MAP.get(a_effect_id)
                && let Some(n_projectee_filter) = n_effect.hc.projectee_filter
            {
                let a_item_list_id = match n_projectee_filter {
                    NEffectProjecteeFilter::ItemList(a_item_list_id) => a_item_list_id,
                    NEffectProjecteeFilter::ItemListAttr(a_attr_id) => match a_item.attrs.get(&a_attr_id) {
                        Some(&attr_value) => match attr_value.round() as AEveItemListId {
                            0 => continue,
                            eve_item_list_id => AItemListId::Eve(eve_item_list_id),
                        },
                        None => continue,
                    },
                };
                a_effect_data.projectee_filter = Some(a_item_list_id);
            }
        }
    }
}
