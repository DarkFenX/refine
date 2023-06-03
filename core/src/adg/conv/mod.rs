use crate::{
    ad,
    adg::{GData, GSupport},
};

mod attr;
mod buff;
mod effect;
mod item;
mod muta;

// Convert data handler-provided entities into cacheable types.
pub(in crate::adg) fn convert(gdata: &GData, gsupp: &GSupport, adata: &mut ad::AData) {
    adata.items = item::conv_items(gdata, gsupp);
    adata.attrs = attr::conv_attrs(gdata);
    adata.mutas = muta::conv_mutas(gdata);
    adata.effects = effect::conv_effects(gdata);
    adata.buffs = buff::conv_buffs(gdata);
}
