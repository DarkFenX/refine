use crate::{ad, ad::generator::GSupport, ed};

mod abil;
mod attr;
mod buff;
mod effect;
mod item;
mod item_list;
mod muta;
mod space_comp;

// Convert EVE data types into adapted data types.
pub(in crate::ad::generator) fn convert_pre(e_data: &ed::EData, g_supp: &GSupport, a_data: &mut ad::AData) {
    a_data.items = item::conv_items(e_data, g_supp);
    a_data.item_lists = item_list::conv_item_lists(e_data);
    a_data.attrs = attr::conv_attrs(e_data);
    a_data.mutas = muta::conv_mutas(e_data);
    a_data.effects = effect::conv_effects(e_data, g_supp);
    a_data.buffs = buff::conv_buffs(e_data);
    a_data.abils = abil::conv_abils(e_data);
    space_comp::apply_space_comps(e_data, a_data);
}
