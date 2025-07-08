use crate::ad;

mod autocharges;
mod max_grp_muta;
mod max_state;

// Convert EVE data types into adapted data types.
pub(in crate::adg) fn convert_post(a_data: &mut ad::AData) {
    max_state::fill_max_state(a_data);
    autocharges::fill_autocharges(a_data);
    max_grp_muta::fill_max_group_mutations(a_data);
}
