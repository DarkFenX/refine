//! Data customizations which are applied on adapted data generation.

use crate::ad::ADataGenerator;

mod attrs;
mod buffs;
mod effects;
mod item_lists;
mod items;
mod misc_asb_cap_stick_effect;
mod misc_subsystem_slots;
mod misc_wdfg_bubble;

impl ADataGenerator {
    pub(in crate::ad::generator) fn customize(&mut self) {
        self.customize_items();
        self.customize_item_lists();
        self.customize_attrs();
        self.customize_effects();
        self.customize_buffs();
        self.remove_asb_cap_stick_effect();
        self.fix_subsystem_slot_count();
        self.add_wdfg_bubble_strength();
    }
}
