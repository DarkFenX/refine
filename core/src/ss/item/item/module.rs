use crate::{
    ad,
    consts::{EffectMode, ModRack, State},
    defs::{EffectId, ReeId, ReeIdx, ReeInt},
    src::Src,
    util::{Named, OptMap},
};

pub(in crate::ss) struct SsModule {
    pub(in crate::ss) id: ReeId,
    pub(in crate::ss) fit_id: ReeId,
    pub(in crate::ss) a_item_id: ReeInt,
    pub(in crate::ss) state: State,
    pub(in crate::ss) rack: ModRack,
    pub(in crate::ss) pos: ReeIdx,
    pub(in crate::ss) charge_a_item_id: Option<ReeId>,
    pub(in crate::ss) effect_modes: OptMap<EffectId, EffectMode>,
    pub(in crate::ss) a_item: Option<ad::ArcItem>,
}
impl SsModule {
    pub(in crate::ss) fn new(
        src: &Src,
        id: ReeId,
        fit_id: ReeId,
        a_item_id: ReeInt,
        state: State,
        rack: ModRack,
        pos: ReeIdx,
        charge_a_item_id: Option<ReeId>,
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
