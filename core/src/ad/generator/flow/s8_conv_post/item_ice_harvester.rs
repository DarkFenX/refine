use crate::ad::{AData, AItemId};

pub(in crate::ad::generator::flow::s8_conv_post) fn fill_ice_harvesters(a_data: &mut AData) {
    for a_item in a_data.items.values_mut() {
        a_item.is_ice_harvester = a_item.srqs.contains_key(&AItemId::ICE_HARVESTING);
    }
}
