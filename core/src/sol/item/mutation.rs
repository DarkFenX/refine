use crate::{
    ad,
    defs::{EAttrId, EMutaId, MutaRange},
    util::StMap,
};

#[derive(Clone)]
pub(in crate::sol) struct SolItemMutation {
    muta_id: EMutaId,
    attrs: StMap<EAttrId, MutaRange>,
    a_muta: Option<ad::ArcMuta>,
}
impl SolItemMutation {
    pub(in crate::sol::item) fn new(muta_id: EMutaId) -> Self {
        Self {
            muta_id,
            attrs: StMap::new(),
            a_muta: None,
        }
    }
}
