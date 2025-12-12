use crate::ad::AData;

mod effect_autocharges;
mod effect_projectee_filters;
mod item_buff_item_lists;
mod item_cap_use_attrs;
mod item_disallowed_in_wspace;
mod item_ice_harvester;
mod item_max_grp_muta;
mod item_max_state;

// Fill in extra data, which has customization as prerequisite
pub(in crate::ad::generator) fn convert_post(a_data: &mut AData) {
    item_max_state::fill_max_state(a_data);
    item_buff_item_lists::fill_buff_item_lists(a_data);
    item_max_grp_muta::fill_max_group_mutations(a_data);
    item_cap_use_attrs::fill_cap_use_attr_ids(a_data);
    item_ice_harvester::fill_ice_harvesters(a_data);
    item_disallowed_in_wspace::fill_disallowed_in_wspace(a_data);
    effect_autocharges::fill_effect_autocharges(a_data);
    effect_projectee_filters::fill_effect_projectee_filters(a_data);
}
