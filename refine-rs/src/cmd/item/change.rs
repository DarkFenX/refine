use crate::cmd::{
    inner::{
        ICmdAutochargeChangeICtx, ICmdBoosterChangeICtx, ICmdCharacterChangeICtx, ItemChangeAutochargeError,
        ItemChangeBoosterError, ItemChangeCharacterError,
    },
    shared::ChangedItemIdsResp,
};

pub enum ChangeItemEnumCmd {
    Autocharge(ItemChangeAutochargeCmd),
    Booster(ItemChangeBoosterCmd),
    Character(ItemChangeCharacterCmd),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ChangeItemEnumCmd {
    pub(crate) fn execute(&self, core_item: &mut rc::ItemMut) -> Result<ChangedItemIdsResp, ChangeItemEnumError> {
        match self {
            Self::Autocharge(cmd) => Ok(cmd.inner.execute(core_item)?.into()),
            Self::Booster(cmd) => Ok(cmd.inner.execute(core_item)?.into()),
            Self::Character(cmd) => Ok(cmd.inner.execute_via_item(core_item)?.into()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChangeItemEnumError {
    #[error("failed to change autocharge: {0}")]
    AutochargeFailed(#[from] ItemChangeAutochargeError),
    #[error("failed to change booster: {0}")]
    BoosterFailed(#[from] ItemChangeBoosterError),
    #[error("failed to change character: {0}")]
    CharacterFailed(#[from] ItemChangeCharacterError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sub-commands - item - autocharge
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default)]
pub struct ItemChangeAutochargeCmd {
    inner: ICmdAutochargeChangeICtx = ICmdAutochargeChangeICtx { .. },
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sub-commands - item - booster
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default)]
pub struct ItemChangeBoosterCmd {
    inner: ICmdBoosterChangeICtx = ICmdBoosterChangeICtx { .. },
}
impl ItemChangeBoosterCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_type_id(mut self, type_id: rc::ItemTypeId) -> Self {
        self.inner.type_id = Some(type_id);
        self
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.inner.state = Some(state);
        self
    }
    pub fn with_side_effects(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, bool)>) -> Self {
        self.inner.side_effects.clear();
        self.inner.side_effects.extend(effect_modes);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.effect_modes.clear();
        self.inner.effect_modes.extend(effect_modes);
        self
    }
}
impl From<ItemChangeBoosterCmd> for ChangeItemEnumCmd {
    fn from(sub_cmd: ItemChangeBoosterCmd) -> Self {
        Self::Booster(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sub-commands - item - character
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default)]
pub struct ItemChangeCharacterCmd {
    inner: ICmdCharacterChangeICtx = ICmdCharacterChangeICtx { .. },
}
impl ItemChangeCharacterCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_type_id(mut self, type_id: rc::ItemTypeId) -> Self {
        self.inner.type_id = Some(type_id);
        self
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
impl From<ItemChangeCharacterCmd> for ChangeItemEnumCmd {
    fn from(sub_cmd: ItemChangeCharacterCmd) -> Self {
        Self::Character(sub_cmd)
    }
}
