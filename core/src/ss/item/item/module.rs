use crate::{
    ad,
    consts::{EffectMode, ModRack, State},
    defs::{EEffectId, EItemId, Idx, SsFitId, SsItemId},
    src::Src,
    util::{Named, OptMap},
};

pub(in crate::ss) struct SsModule {
    pub(in crate::ss) id: SsItemId,
    pub(in crate::ss) fit_id: SsFitId,
    pub(in crate::ss) a_item_id: EItemId,
    pub(in crate::ss) state: State,
    pub(in crate::ss) rack: ModRack,
    pub(in crate::ss) pos: Idx,
    pub(in crate::ss) charge_a_item_id: Option<SsItemId>,
    pub(in crate::ss) effect_modes: OptMap<EEffectId, EffectMode>,
    pub(in crate::ss) a_item: Option<ad::ArcItem>,
}
impl SsModule {
    pub(in crate::ss) fn new(
        src: &Src,
        id: SsItemId,
        fit_id: SsFitId,
        a_item_id: EItemId,
        state: State,
        rack: ModRack,
        pos: Idx,
        charge_a_item_id: Option<SsItemId>,
    ) -> Self {
        Self {
            id,
            fit_id,
            a_item_id,
            state,
            rack,
            pos,
            charge_a_item_id: charge_a_item_id,
            effect_modes: OptMap::new(),
            a_item: src.get_a_item(&a_item_id),
        }
    }
}
impl Named for SsModule {
    fn get_name() -> &'static str {
        "SsModule"
    }
}
impl std::fmt::Display for SsModule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}(id={}, a_item_id={})", Self::get_name(), self.id, self.a_item_id)
    }
}
