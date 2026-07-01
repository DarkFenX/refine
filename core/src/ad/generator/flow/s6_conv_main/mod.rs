use crate::ad::ADataGenerator;

mod abil;
mod attr;
mod buff;
mod effect;
mod item;
mod item_list;
mod muta;
mod space_comp;

// Convert EVE data types into adapted data types.
impl ADataGenerator {
    pub(in crate::ad::generator) fn convert_main(&mut self) {
        self.conv_items();
        self.conv_item_lists();
        self.conv_attrs();
        self.conv_mutas();
        self.conv_effects();
        self.conv_buffs();
        self.conv_abils();
        self.apply_space_comps();
    }
}
