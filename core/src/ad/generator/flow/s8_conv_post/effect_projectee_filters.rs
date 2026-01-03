use crate::{
    ad::{AData, AEveItemListId, AItemListId},
    nd::{N_EFFECT_MAP, NEffectProjecteeFilter},
};

pub(in crate::ad::generator::flow::s8_conv_post) fn fill_effect_projectee_filters(a_data: &mut AData) {
    for a_item in a_data.items.values_mut() {
        for (effect_aid, a_effect_data) in a_item.effect_datas.iter_mut() {
            if let Some(n_effect) = N_EFFECT_MAP.get(effect_aid)
                && let Some(n_projectee_filter) = &n_effect.projectee_filter
            {
                let item_list_aid = match n_projectee_filter {
                    NEffectProjecteeFilter::ItemList(item_list_aid) => *item_list_aid,
                    NEffectProjecteeFilter::ItemListAttr(attr_aid) => match a_item.attrs.get(attr_aid) {
                        Some(&attr_value) => {
                            let eve_item_list_aid = AEveItemListId::new_of64(attr_value.into_inner());
                            if eve_item_list_aid == AEveItemListId::new(0) {
                                continue;
                            }
                            AItemListId::Eve(eve_item_list_aid)
                        }
                        None => continue,
                    },
                };
                a_effect_data.projectee_filter = Some(item_list_aid);
            }
        }
    }
}
