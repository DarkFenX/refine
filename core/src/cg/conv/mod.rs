use crate::ch;

use super::{data::Support, Data};

mod attr;
mod buff;
mod effect;
mod item;
mod muta;

// Convert data handler-provided entities into cacheable types.
pub(super) fn convert(cg_data: &Data, supp: &Support, cd_cont: &mut ch::Data) {
    cd_cont.items = item::conv_items(cg_data, supp);
    cd_cont.attrs = attr::conv_attrs(cg_data);
    cd_cont.mutas = muta::conv_mutas(cg_data);
    cd_cont.effects = effect::conv_effects(cg_data);
    cd_cont.buffs = buff::conv_buffs(cg_data);
}
