mod attr;
mod buff;
mod effect;
mod item;
mod muta;

use super::{data::Support, Data};

// Convert data handler-provided entities into cacheable types.
pub(super) fn convert(data: &Data, supp: &Support, warns: &mut Vec<String>) {
    let items = item::conv_items(data, supp, warns);
    let attrs = attr::conv_attrs(data);
    let mutas = muta::conv_mutas(data);
    let buffs = buff::conv_buffs(data, warns);
}
