use crate::{ad::ADataGenerator, nd::N_EFFECT_MAP};

impl ADataGenerator {
    pub(super) fn fill_cloaks(&mut self) {
        for a_item in self.a_data.items.data.values_mut() {
            for a_item_effect in a_item.effects.iter() {
                let Some(n_effect) = N_EFFECT_MAP.get(&a_item_effect.id) else {
                    continue;
                };
                if n_effect.cloaks_carrier {
                    a_item.is_cloak = true;
                    break;
                }
            }
        }
    }
}
