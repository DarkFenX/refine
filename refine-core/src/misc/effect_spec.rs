use crate::{rd::REffectId, ud::UItemId};

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
#[cfg(feature = "dev")]
mod debug {
    use super::*;
    use crate::{dbg::DebugResult, ud::UData};

    impl EffectSpec {
        pub(crate) fn consistency_check(&self, u_data: &UData, check_item_load: bool) -> DebugResult {
            let Self { item_uid, effect_rid } = self;
            item_uid.consistency_check(u_data, check_item_load)?;
            effect_rid.consistency_check(u_data)?;
            Ok(())
        }
    }
}
