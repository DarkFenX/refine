use crate::ad::{AItemId, AMutaAttrs, AMutaItemConvs};

pub struct AMuta {
    pub id: AItemId,
    pub item_map: AMutaItemConvs = AMutaItemConvs::new(),
    pub attr_mods: AMutaAttrs = AMutaAttrs::new(),
}
