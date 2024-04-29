use crate::{
    ad,
    adg::{GData, GSupport},
};

mod attr;
mod buff;
mod effect;
mod item;
mod muta;

// Convert EVE data types into adapted data types.
pub(in crate::adg) fn convert(g_data: &GData, g_supp: &GSupport, a_data: &mut ad::AData) {
    a_data.items = item::conv_items(g_data, g_supp);
    a_data.attrs = attr::conv_attrs(g_data);
    a_data.mutas = muta::conv_mutas(g_data);
    a_data.effects = effect::conv_effects(g_data, g_supp);
    a_data.buffs = buff::conv_buffs(g_data);
}
