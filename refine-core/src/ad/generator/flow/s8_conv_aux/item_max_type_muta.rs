// Data for item count of a type limit - need to do it here to take into account that mutated item
// can have the limit even if raw mutated type has no such limit

use crate::{
    ad::{AAttrId, ADataGenerator, AItem},
    util::RSet,
};

impl ADataGenerator {
    pub(super) fn fill_max_type_mutations(&mut self) {
        let a_items = &self.a_data.items.data;
        let mut tracked_item_aids: RSet<_> = a_items
            .values()
            .filter_map(|a_item| match is_limited(a_item) {
                true => Some(a_item.id),
                false => None,
            })
            .collect();
        // Mutated items inherit attributes of their base item, so the limit of base item makes
        // items of mutated type count
        for a_muta in self.a_data.mutas.data.values() {
            for a_item_conv in a_muta.items.iter() {
                if a_items.get(&a_item_conv.base_item_id).is_some_and(is_limited) {
                    tracked_item_aids.insert(a_item_conv.mutated_item_id);
                }
            }
        }
        for a_item in self.a_data.items.data.values_mut() {
            a_item.val_fitted_type_tracked = tracked_item_aids.contains(&a_item.id);
        }
    }
}

fn is_limited(a_item: &AItem) -> bool {
    a_item.attrs.contains_id(&AAttrId::MAX_TYPE_FITTED)
}
