use crate::{
    AutochargeChangeCmd, BoosterChangeCmd, ChangedItemIdsResp, ChargeChangeCmd, ImplantChangeCmd,
    ItemChangeCharacterCmd, ItemChangeDroneCmd, ItemChangeFighterCmd, ItemChangeFwEffectCmd, ItemChangeModuleCmd,
    ItemChangeProjEffectCmd, ItemChangeShipCmd, ItemChangeSkillCmd, ItemChangeStanceCmd, ItemChangeSwEffectCmd,
    RigChangeCmd, ServiceChangeCmd, SubsystemChangeCmd,
    err::{
        AutochargeChangeError, BoosterChangeError, ChargeChangeError, ImplantChangeError, ItemChangeCharacterError,
        ItemChangeDroneError, ItemChangeFighterError, ItemChangeFwEffectError, ItemChangeModuleError,
        ItemChangeProjEffectError, ItemChangeShipError, ItemChangeSkillError, ItemChangeStanceError,
        ItemChangeSwEffectError, RigChangeError, ServiceChangeError, SubsystemChangeError,
    },
};

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(tag = "type", rename_all = "snake_case")
)]
pub enum ItemCtlCmd {
    Autocharge(AutochargeChangeCmd),
    Booster(BoosterChangeCmd),
    Character(ItemChangeCharacterCmd),
    Charge(ChargeChangeCmd),
    Drone(ItemChangeDroneCmd),
    Fighter(ItemChangeFighterCmd),
    FwEffect(ItemChangeFwEffectCmd),
    Implant(ImplantChangeCmd),
    Module(ItemChangeModuleCmd),
    ProjEffect(ItemChangeProjEffectCmd),
    Rig(RigChangeCmd),
    Service(ServiceChangeCmd),
    Ship(ItemChangeShipCmd),
    Skill(ItemChangeSkillCmd),
    Stance(ItemChangeStanceCmd),
    Subsystem(SubsystemChangeCmd),
    SwEffect(ItemChangeSwEffectCmd),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl BoosterChangeCmd {
    pub fn into_item_ctl(self) -> ItemCtlCmd {
        ItemCtlCmd::Booster(self)
    }
}
impl ChargeChangeCmd {
    pub fn into_item_ctl(self) -> ItemCtlCmd {
        ItemCtlCmd::Charge(self)
    }
}
impl ImplantChangeCmd {
    pub fn into_item_ctl(self) -> ItemCtlCmd {
        ItemCtlCmd::Implant(self)
    }
}
impl RigChangeCmd {
    pub fn into_item_ctl(self) -> ItemCtlCmd {
        ItemCtlCmd::Rig(self)
    }
}
impl ServiceChangeCmd {
    pub fn into_item_ctl(self) -> ItemCtlCmd {
        ItemCtlCmd::Service(self)
    }
}
impl SubsystemChangeCmd {
    pub fn into_item_ctl(self) -> ItemCtlCmd {
        ItemCtlCmd::Subsystem(self)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemCtlCmd {
    pub(crate) fn execute(self, core_item: &mut rc::ItemMut) -> Result<ChangedItemIdsResp, ItemCtlError> {
        match self {
            Self::Autocharge(cmd) => Ok(cmd.execute(core_item)?),
            Self::Booster(cmd) => Ok(cmd.execute(core_item)?),
            Self::Character(cmd) => Ok(cmd.inner.execute_via_item(core_item)?),
            Self::Charge(cmd) => Ok(cmd.execute(core_item)?),
            Self::Drone(cmd) => Ok(cmd.inner.execute(core_item)?),
            Self::Fighter(cmd) => Ok(cmd.inner.execute(core_item)?),
            Self::FwEffect(cmd) => Ok(cmd.inner.execute(core_item)?),
            Self::Implant(cmd) => Ok(cmd.execute(core_item)?),
            Self::Module(cmd) => Ok(cmd.inner.execute(core_item)?),
            Self::ProjEffect(cmd) => Ok(cmd.inner.execute(core_item)?),
            Self::Rig(cmd) => Ok(cmd.execute(core_item)?),
            Self::Service(cmd) => Ok(cmd.execute(core_item)?),
            Self::Ship(cmd) => Ok(cmd.inner.execute_via_item(core_item)?),
            Self::Skill(cmd) => Ok(cmd.inner.execute(core_item)?),
            Self::Stance(cmd) => Ok(cmd.inner.execute_via_item(core_item)?),
            Self::Subsystem(cmd) => Ok(cmd.execute(core_item)?),
            Self::SwEffect(cmd) => Ok(cmd.inner.execute(core_item)?),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ItemCtlError {
    #[error("failed to change autocharge")]
    Autocharge(#[from] AutochargeChangeError),
    #[error("failed to change booster")]
    Booster(#[from] BoosterChangeError),
    #[error("failed to change character")]
    Character(#[from] ItemChangeCharacterError),
    #[error("failed to change charge")]
    Charge(#[from] ChargeChangeError),
    #[error("failed to change drone")]
    Drone(#[from] ItemChangeDroneError),
    #[error("failed to change fighter")]
    Fighter(#[from] ItemChangeFighterError),
    #[error("failed to change fit-wide effect")]
    FwEffect(#[from] ItemChangeFwEffectError),
    #[error("failed to change implant")]
    Implant(#[from] ImplantChangeError),
    #[error("failed to change module")]
    Module(#[from] ItemChangeModuleError),
    #[error("failed to change projected effect")]
    ProjEffect(#[from] ItemChangeProjEffectError),
    #[error("failed to change rig")]
    Rig(#[from] RigChangeError),
    #[error("failed to change service")]
    Service(#[from] ServiceChangeError),
    #[error("failed to change ship")]
    Ship(#[from] ItemChangeShipError),
    #[error("failed to change skill")]
    Skill(#[from] ItemChangeSkillError),
    #[error("failed to change stance")]
    Stance(#[from] ItemChangeStanceError),
    #[error("failed to change subsystem")]
    Subsystem(#[from] SubsystemChangeError),
    #[error("failed to change system-wide effect")]
    SwEffect(#[from] ItemChangeSwEffectError),
}
