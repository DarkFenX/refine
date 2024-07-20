use crate::{
    defs::{Amount, EItemId, SolFitId, SolItemId},
    sol::item::{SolAutocharges, SolItemBase, SolItemState},
    src::Src,
    util::Named,
};

pub(in crate::sol) struct SolFighter {
    pub(in crate::sol) base: SolItemBase,
    pub(in crate::sol) fit_id: SolFitId,
    pub(in crate::sol) state: SolItemState,
    pub(in crate::sol) amt_override: Option<Amount>,
    pub(in crate::sol) autocharges: SolAutocharges,
}
impl SolFighter {
    pub(in crate::sol) fn new(
        src: &Src,
        id: SolItemId,
        fit_id: SolFitId,
        a_item_id: EItemId,
        state: SolItemState,
    ) -> Self {
        Self {
            base: SolItemBase::new(src, id, a_item_id),
            fit_id,
            state,
            amt_override: None,
            autocharges: SolAutocharges::new(),
        }
    }
}
impl Named for SolFighter {
    fn get_name() -> &'static str {
        "SolFighter"
    }
}
impl std::fmt::Display for SolFighter {
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
