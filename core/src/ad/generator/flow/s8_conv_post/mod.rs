use crate::ad::AData;

mod buff_item_lists;
mod disallowed_in_wspace;
mod effect_autocharges;
mod effect_projectee_filters;
mod ice_harvester;
mod max_grp_muta;
mod max_state;

// Fill in extra data, which has customization as prerequisite
pub(in crate::ad::generator) fn convert_post(a_data: &mut AData) {
    max_state::fill_max_state(a_data);
    effect_autocharges::fill_effect_autocharges(a_data);
    effect_projectee_filters::fill_effect_projectee_filters(a_data);
    max_grp_muta::fill_max_group_mutations(a_data);
    buff_item_lists::fill_buff_item_lists(a_data);
    ice_harvester::fill_ice_harvesters(a_data);
    disallowed_in_wspace::fill_disallowed_in_wspace(a_data);
}
