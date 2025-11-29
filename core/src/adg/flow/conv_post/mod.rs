use crate::{ad::AData, adg::GSupport};

mod buff_item_lists;
mod effect_autocharges;
mod effect_projectee_filters;
mod max_grp_muta;
mod max_state;

// Fill in extra data, which has customization as prerequisite
pub(in crate::adg) fn convert_post(a_data: &mut AData, g_supp: &GSupport) {
    max_state::fill_max_state(a_data);
    effect_autocharges::fill_effect_autocharges(a_data);
    effect_projectee_filters::fill_effect_projectee_filters(a_data, g_supp);
    max_grp_muta::fill_max_group_mutations(a_data);
    buff_item_lists::fill_buff_item_lists(a_data, g_supp);
}
