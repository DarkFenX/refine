use crate::{
    ad,
    defs::{EItemId, SolFitId, SolItemId},
    sol::item::SolItemBase,
    src::Src,
    util::{Named, Result},
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
    pub(in crate::sol) fn get_id(&self) -> SolItemId {
        self.base.get_id()
    }
    pub(in crate::sol::item) fn get_fit_id(&self) -> SolFitId {
        self.fit_id
    }
    pub(in crate::sol) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::sol) fn get_a_item(&self) -> Result<&ad::ArcItem> {
        self.base.get_a_item()
    }
    pub(in crate::sol::item) fn reload_a_item(&mut self, _: &Src) {
        // Just panic to expose attempts to reload it, since autocharges should never be reloaded.
        // Instead, they are removed and re-added when source changes.
        panic!();
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
