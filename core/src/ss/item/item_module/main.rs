use crate::{
    ad,
    defs::{EItemId, Idx, SsFitId, SsItemId},
    src::Src,
    ss::{
        item::{SsEffectModes, SsItemState, SsTgtItems},
        SsModRack,
    },
    util::Named,
};

pub(in crate::ss) struct SsModule {
    pub(in crate::ss) id: SsItemId,
    pub(in crate::ss) fit_id: SsFitId,
    pub(in crate::ss) a_item_id: EItemId,
    pub(in crate::ss) state: SsItemState,
    pub(in crate::ss) rack: SsModRack,
    pub(in crate::ss) pos: Idx,
    pub(in crate::ss) charge_item_id: Option<SsItemId>,
    pub(in crate::ss) tgts: SsTgtItems,
    pub(in crate::ss) effect_modes: SsEffectModes,
    pub(in crate::ss) a_item: Option<ad::ArcItem>,
}
impl SsModule {
    pub(in crate::ss) fn new(
        src: &Src,
        id: SsItemId,
        fit_id: SsFitId,
        a_item_id: EItemId,
        state: SsItemState,
        rack: SsModRack,
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
            charge_item_id: charge_a_item_id,
            tgts: SsTgtItems::new(),
            effect_modes: SsEffectModes::new(),
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
