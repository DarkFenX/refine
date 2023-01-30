use crate::ch;

use super::{data::Support, CgData};

mod attr;
mod buff;
mod effect;
mod item;
mod muta;

// Convert data handler-provided entities into cacheable types.
pub(super) fn convert(cg_data: &CgData, supp: &Support, warns: &mut Vec<String>, cdcont: &mut ch::ChData) {
    cdcont.items = item::conv_items(cg_data, supp, warns);
    cdcont.attrs = attr::conv_attrs(cg_data);
    cdcont.mutas = muta::conv_mutas(cg_data);
    cdcont.effects = effect::conv_effects(cg_data, warns);
    cdcont.buffs = buff::conv_buffs(cg_data, warns);
}
