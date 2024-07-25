use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::item::SolItemBase,
    src::Src,
    util::Named,
};

pub(in crate::sol) struct SolAutoCharge {
    pub(in crate::sol) base: SolItemBase,
    pub(in crate::sol) fit_id: SolFitId,
    pub(in crate::sol) cont_id: SolItemId,
}
impl SolAutoCharge {
    pub(in crate::sol) fn new(
        src: &Src,
        id: SolItemId,
        fit_id: SolFitId,
        a_item_id: EItemId,
        cont_id: SolItemId,
    ) -> Self {
        Self {
            base: SolItemBase::new(src, id, a_item_id),
            fit_id,
            cont_id,
        }
    }
}
impl Named for SolAutoCharge {
    fn get_name() -> &'static str {
        "SolAutoCharge"
    }
}
impl std::fmt::Display for SolAutoCharge {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}(id={}, a_item_id={})",
            Self::get_name(),
            self.base.id,
            self.base.a_item_id
        )
    }
}
