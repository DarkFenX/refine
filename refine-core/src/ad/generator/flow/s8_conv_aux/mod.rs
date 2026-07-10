use crate::ad::ADataGenerator;

mod effect_autocharges;
mod effect_projectee_filters;
mod item_buff_item_lists;
mod item_cloak;
mod item_disallowed_in_wspace;
mod item_ice_harvester;
mod item_max_grp_muta;
mod item_max_state;

// Fill in extra data, which has customization as prerequisite
impl ADataGenerator {
    pub(in crate::ad::generator) fn convert_aux(&mut self) {
        self.fill_max_state();
        self.fill_buff_item_lists();
        self.fill_max_group_mutations();
        self.fill_cloaks();
        self.fill_ice_harvesters();
        self.fill_disallowed_in_wspace();
        self.fill_effect_autocharges();
        self.fill_effect_projectee_filters();
    }
}
