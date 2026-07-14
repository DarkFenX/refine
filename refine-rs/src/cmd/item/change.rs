use crate::cmd::{
    inner::{ChangeAutochargeError, CmdAutochargeChangeICtx},
    shared::ChangedItemIdsResp,
};

pub enum ChangeItemEnumCmd {
    Autocharge(ItemChangeAutochargeCmd),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ChangeItemEnumCmd {
    pub(crate) fn execute(&self, core_item: &mut rc::ItemMut) -> Result<ChangedItemIdsResp, ChangeItemEnumError> {
        match self {
            // Item - autocharge
            Self::Autocharge(cmd) => Ok(cmd.inner.execute(core_item)?.into()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChangeItemEnumError {
    #[error("failed to change autocharge: {0}")]
    AutochargeFailed(#[from] ChangeAutochargeError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sub-commands - item - autocharge
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default)]
pub struct ItemChangeAutochargeCmd {
    inner: CmdAutochargeChangeICtx,
}
impl ItemChangeAutochargeCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.inner.state = Some(state);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.effect_modes.clear();
        self.inner.effect_modes.extend(effect_modes);
        self
    }
}
impl From<ItemChangeAutochargeCmd> for ChangeItemEnumCmd {
    fn from(sub_cmd: ItemChangeAutochargeCmd) -> Self {
        Self::Autocharge(sub_cmd)
    }
}
