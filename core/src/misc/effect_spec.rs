use crate::{
    dbg::DebugResult,
    rd::REffectId,
    ud::{UData, UItemId},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct EffectSpec {
    pub(crate) item_uid: UItemId,
    pub(crate) effect_rid: REffectId,
}
impl EffectSpec {
    pub(crate) fn new(item_uid: UItemId, effect_rid: REffectId) -> Self {
        Self { item_uid, effect_rid }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
impl EffectSpec {
    pub(crate) fn consistency_check(&self, u_data: &UData, check_item_load: bool) -> DebugResult {
        self.item_uid.consistency_check(u_data, check_item_load)?;
        self.effect_rid.consistency_check(u_data)?;
        Ok(())
    }
}
