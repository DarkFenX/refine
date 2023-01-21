use crate::ch;

use super::{data::Support, Data};

mod attr;
mod buff;
mod effect;
mod item;
mod muta;

// Convert data handler-provided entities into cacheable types.
pub(super) fn convert(data: &Data, supp: &Support, warns: &mut Vec<String>, cdcont: &mut ch::Container) {
    cdcont.items = item::conv_items(data, supp, warns);
    cdcont.attrs = attr::conv_attrs(data);
    cdcont.mutas = muta::conv_mutas(data);
    cdcont.effects = effect::conv_effects(data, warns);
    cdcont.buffs = buff::conv_buffs(data, warns);
}
