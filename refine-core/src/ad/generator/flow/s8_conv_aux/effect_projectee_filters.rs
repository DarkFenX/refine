use crate::{
    ad::{ADataGenerator, AItemListId},
    nd::{N_EFFECT_MAP, NEffectProjecteeFilter},
};

impl ADataGenerator {
    pub(super) fn fill_effect_projectee_filters(&mut self) {
        for a_item in self.a_data.items.data.values_mut() {
            for a_item_effect in a_item.effects.iter_mut() {
                if let Some(n_effect) = N_EFFECT_MAP.get(&a_item_effect.id)
                    && let Some(n_projectee_filter) = &n_effect.projectee_filter
                {
                    let item_list_aid = match n_projectee_filter {
                        NEffectProjecteeFilter::ItemList(item_list_aid) => *item_list_aid,
                        NEffectProjecteeFilter::ItemListAttr(attr_aid) => {
                            let Some(a_item_attr) = a_item.attrs.get(attr_aid) else {
                                continue;
                            };
                            match AItemListId::try_eve_from_f64_rounded(a_item_attr.value.into_f64()) {
                                Some(item_list_aid) => item_list_aid,
                                None => continue,
                            }
                        }
                    };
                    a_item_effect.data.projectee_filter = Some(item_list_aid);
                }
            }
        }
    }
}
