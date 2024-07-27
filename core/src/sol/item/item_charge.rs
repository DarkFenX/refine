use crate::{
    ad,
    defs::{EItemId, SolFitId, SolItemId},
    sol::item::SolItemBase,
    src::Src,
    util::{Named, Result},
};

pub(in crate::sol) struct SolCharge {
    pub(in crate::sol) base: SolItemBase,
    pub(in crate::sol) fit_id: SolFitId,
    pub(in crate::sol) cont_id: SolItemId,
}
impl SolCharge {
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
    pub(in crate::sol::item) fn get_id(&self) -> SolItemId {
        self.base.get_id()
    }
    pub(in crate::sol::item) fn get_fit_id(&self) -> SolFitId {
        self.fit_id
    }
    pub(in crate::sol::item) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::sol) fn get_a_item(&self) -> Result<&ad::ArcItem> {
        self.base.get_a_item()
    }
    pub(in crate::sol::item) fn reload_a_item(&mut self, src: &Src) {
        self.base.reload_a_item(src);
    }
}
impl Named for SolCharge {
    fn get_name() -> &'static str {
        "SolCharge"
    }
}
impl std::fmt::Display for SolCharge {
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
