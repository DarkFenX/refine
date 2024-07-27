use crate::{
    ad,
    defs::{EItemId, Idx, SolFitId, SolItemId},
    sol::{
        item::{SolItemBase, SolItemState, SolProjs},
        SolModRack,
    },
    src::Src,
    util::{Named, Result},
};

pub(in crate::sol) struct SolModule {
    pub(in crate::sol) base: SolItemBase,
    pub(in crate::sol) fit_id: SolFitId,
    pub(in crate::sol) state: SolItemState,
    pub(in crate::sol) rack: SolModRack,
    pub(in crate::sol) pos: Idx,
    pub(in crate::sol) charge_item_id: Option<SolItemId>,
    pub(in crate::sol) projs: SolProjs,
}
impl SolModule {
    pub(in crate::sol) fn new(
        src: &Src,
        id: SolItemId,
        fit_id: SolFitId,
        a_item_id: EItemId,
        state: SolItemState,
        rack: SolModRack,
        pos: Idx,
        charge_a_item_id: Option<SolItemId>,
    ) -> Self {
        Self {
            base: SolItemBase::new(src, id, a_item_id),
            fit_id,
            state,
            rack,
            pos,
            charge_item_id: charge_a_item_id,
            projs: SolProjs::new(),
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
impl Named for SolModule {
    fn get_name() -> &'static str {
        "SolModule"
    }
}
impl std::fmt::Display for SolModule {
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
