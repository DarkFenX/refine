use crate::cmd::{
    GetFitAddBoosterError, GetFitAddDroneError, GetFitAddFighterError, GetFitAddFwEffectError, GetFitAddRigError,
    GetFitSetCharacterError, ItemAddBoosterCmd, ItemAddDroneCmd, ItemAddFighterCmd, ItemAddFwEffectCmd, ItemAddRigCmd,
    ItemSetCharacterCmd, shared::AddedItemIdsResp,
};

pub enum AddItemEnumCmd {
    Booster(ItemAddBoosterCmd),
    Character(ItemSetCharacterCmd),
    Drone(ItemAddDroneCmd),
    Fighter(ItemAddFighterCmd),
    FwEffect(ItemAddFwEffectCmd),
    Rig(ItemAddRigCmd),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AddItemEnumCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<AddedItemIdsResp, AddItemEnumError> {
        match self {
            Self::Booster(cmd) => Ok(cmd.inner.execute(core_sol)?.into()),
            Self::Character(cmd) => Ok(cmd.inner.execute(core_sol)?.into()),
            Self::Drone(cmd) => Ok(cmd.inner.execute(core_sol)?.into()),
            Self::Fighter(cmd) => Ok(cmd.inner.execute(core_sol)?.into()),
            Self::FwEffect(cmd) => Ok(cmd.inner.execute(core_sol)?.into()),
            Self::Rig(cmd) => Ok(cmd.inner.execute(core_sol)?.into()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddItemEnumError {
    #[error("failed to add booster: {0}")]
    BoosterFailed(#[from] GetFitAddBoosterError),
    #[error("failed to set character: {0}")]
    CharacterFailed(#[from] GetFitSetCharacterError),
    #[error("failed to add drone: {0}")]
    DroneFailed(#[from] GetFitAddDroneError),
    #[error("failed to add fighter: {0}")]
    FighterFailed(#[from] GetFitAddFighterError),
    #[error("failed to add fit-wide effect: {0}")]
    FwEffectFailed(#[from] GetFitAddFwEffectError),
    #[error("failed to add rig: {0}")]
    RigFailed(#[from] GetFitAddRigError),
}
