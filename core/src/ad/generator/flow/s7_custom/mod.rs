//! Data customizations which are applied on adapted data generation.

use crate::ad::AData;

mod attrs;
mod buffs;
mod effects;
mod item_lists;
mod misc_asb_cap_stick_effect;
mod misc_subsystem_slots;
mod misc_wdfg_bubble;

pub(in crate::ad::generator) fn customize(a_data: &mut AData) {
    item_lists::customize_item_lists(a_data);
    attrs::customize_attrs(a_data);
    effects::customize_effects(a_data);
    buffs::customize_buffs(a_data);
    misc_asb_cap_stick_effect::remove_asb_cap_stick_effect(a_data);
    misc_subsystem_slots::fix_subsysem_slot_count(a_data);
    misc_wdfg_bubble::add_wdfg_bubble_strength(a_data);
}
