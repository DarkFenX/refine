use crate::{
    ad::{AData, AEveItemListId, AItemListId},
    nd::{N_EFFECT_MAP, NEffectProjecteeFilter},
};

pub(in crate::ad::generator::flow::s8_conv_post) fn fill_effect_projectee_filters(a_data: &mut AData) {
    for a_item in a_data.items.data.values_mut() {
        for a_item_effect in a_item.effect_datas.iter_mut() {
            if let Some(n_effect) = N_EFFECT_MAP.get(&a_item_effect.id)
                && let Some(n_projectee_filter) = &n_effect.projectee_filter
            {
                let item_list_aid = match n_projectee_filter {
                    NEffectProjecteeFilter::ItemList(item_list_aid) => *item_list_aid,
                    NEffectProjecteeFilter::ItemListAttr(attr_aid) => match a_item.attrs.get(attr_aid) {
                        Some(a_item_attr) => {
                            let eve_item_list_aid = AEveItemListId::from_f64_rounded(a_item_attr.value.into_f64());
                            if eve_item_list_aid == AEveItemListId::from_i32(0) {
                                continue;
                            }
                            AItemListId::Eve(eve_item_list_aid)
                        }
                        None => continue,
                    },
                };
                a_item_effect.data.projectee_filter = Some(item_list_aid);
            }
        }
    }
}
