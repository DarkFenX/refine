use crate::{
    ad,
    defs::{EItemId, Idx, SolFitId, SolItemId},
    sol::{
        item::{SolEffectModes, SolItemState, SolProjs},
        SolModRack,
    },
    src::Src,
    util::Named,
};

pub(in crate::sol) struct SolModule {
    pub(in crate::sol) id: SolItemId,
    pub(in crate::sol) fit_id: SolFitId,
    pub(in crate::sol) a_item_id: EItemId,
    pub(in crate::sol) state: SolItemState,
    pub(in crate::sol) rack: SolModRack,
    pub(in crate::sol) pos: Idx,
    pub(in crate::sol) charge_item_id: Option<SolItemId>,
    pub(in crate::sol) projs: SolProjs,
    pub(in crate::sol) effect_modes: SolEffectModes,
    pub(in crate::sol) a_item: Option<ad::ArcItem>,
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
            id,
            fit_id,
            a_item_id,
            state,
            rack,
            pos,
            charge_item_id: charge_a_item_id,
            projs: SolProjs::new(),
            effect_modes: SolEffectModes::new(),
            a_item: src.get_a_item(&a_item_id).cloned(),
        }
    }
}
impl Named for SolModule {
    fn get_name() -> &'static str {
        "SolModule"
    }
}
impl std::fmt::Display for SolModule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={}, a_item_id={})", Self::get_name(), self.id, self.a_item_id)
    }
}
