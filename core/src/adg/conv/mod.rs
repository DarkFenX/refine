use crate::adh;

use super::{data::Support, Data};

mod attr;
mod buff;
mod effect;
mod item;
mod muta;

// Convert data handler-provided entities into cacheable types.
pub(super) fn convert(erg_data: &Data, supp: &Support, cd_cont: &mut adh::AData) {
    cd_cont.items = item::conv_items(erg_data, supp);
    cd_cont.attrs = attr::conv_attrs(erg_data);
    cd_cont.mutas = muta::conv_mutas(erg_data);
    cd_cont.effects = effect::conv_effects(erg_data);
    cd_cont.buffs = buff::conv_buffs(erg_data);
}
