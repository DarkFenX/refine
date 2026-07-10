use crate::ad::{ADataGenerator, AItemId};

impl ADataGenerator {
    pub(super) fn fill_ice_harvesters(&mut self) {
        for a_item in self.a_data.items.data.values_mut() {
            a_item.is_ice_harvester = a_item.srqs.contains_id(&AItemId::ICE_HARVESTING);
        }
    }
}
