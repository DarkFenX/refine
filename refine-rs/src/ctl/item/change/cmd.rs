use crate::{
    AutochargeChangeCmd, BoosterChangeCmd, ChangedItemIdsResp, CharacterChangeCmd, ChargeChangeCmd, DroneChangeCmd,
    FighterChangeCmd, FwEffectChangeCmd, ImplantChangeCmd, ModuleChangeCmd, ProjEffectChangeCmd, RigChangeCmd,
    ServiceChangeCmd, ShipChangeCmd, SkillChangeCmd, StanceChangeCmd, SubsystemChangeCmd, SwEffectChangeCmd,
    err::{
        AutochargeChangeError, BoosterChangeError, ChargeChangeError, DroneChangeError, FighterChangeError,
        FwEffectChangeError, ImplantChangeError, ItemCharacterChangeError, ItemShipChangeError, ItemStanceChangeError,
        ModuleChangeError, ProjEffectChangeError, RigChangeError, ServiceChangeError, SkillChangeError,
        SubsystemChangeError, SwEffectChangeError,
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
    Character(CharacterChangeCmd),
    Charge(ChargeChangeCmd),
    Drone(DroneChangeCmd),
    Fighter(FighterChangeCmd),
    FwEffect(FwEffectChangeCmd),
    Implant(ImplantChangeCmd),
    Module(ModuleChangeCmd),
    ProjEffect(ProjEffectChangeCmd),
    Rig(RigChangeCmd),
    Service(ServiceChangeCmd),
    Ship(ShipChangeCmd),
    Skill(SkillChangeCmd),
    Stance(StanceChangeCmd),
    Subsystem(SubsystemChangeCmd),
    SwEffect(SwEffectChangeCmd),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AutochargeChangeCmd {
    pub fn into_item_ctl(self) -> ItemCtlCmd {
        ItemCtlCmd::Autocharge(self)
    }
}
impl BoosterChangeCmd {
    pub fn into_item_ctl(self) -> ItemCtlCmd {
        ItemCtlCmd::Booster(self)
    }
}
impl CharacterChangeCmd {
    pub fn into_item_ctl(self) -> ItemCtlCmd {
        ItemCtlCmd::Character(self)
    }
}
impl ChargeChangeCmd {
    pub fn into_item_ctl(self) -> ItemCtlCmd {
        ItemCtlCmd::Charge(self)
    }
}
impl DroneChangeCmd {
    pub fn into_item_ctl(self) -> ItemCtlCmd {
        ItemCtlCmd::Drone(self)
    }
}
impl FighterChangeCmd {
    pub fn into_item_ctl(self) -> ItemCtlCmd {
        ItemCtlCmd::Fighter(self)
    }
}
impl FwEffectChangeCmd {
    pub fn into_item_ctl(self) -> ItemCtlCmd {
        ItemCtlCmd::FwEffect(self)
    }
}
impl ImplantChangeCmd {
    pub fn into_item_ctl(self) -> ItemCtlCmd {
        ItemCtlCmd::Implant(self)
    }
}
impl ModuleChangeCmd {
    pub fn into_item_ctl(self) -> ItemCtlCmd {
        ItemCtlCmd::Module(self)
    }
}
impl ProjEffectChangeCmd {
    pub fn into_item_ctl(self) -> ItemCtlCmd {
        ItemCtlCmd::ProjEffect(self)
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
impl ShipChangeCmd {
    pub fn into_item_ctl(self) -> ItemCtlCmd {
        ItemCtlCmd::Ship(self)
    }
}
impl SkillChangeCmd {
    pub fn into_item_ctl(self) -> ItemCtlCmd {
        ItemCtlCmd::Skill(self)
    }
}
impl StanceChangeCmd {
    pub fn into_item_ctl(self) -> ItemCtlCmd {
        ItemCtlCmd::Stance(self)
    }
}
impl SubsystemChangeCmd {
    pub fn into_item_ctl(self) -> ItemCtlCmd {
        ItemCtlCmd::Subsystem(self)
    }
}
impl SwEffectChangeCmd {
    pub fn into_item_ctl(self) -> ItemCtlCmd {
        ItemCtlCmd::SwEffect(self)
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
            Self::Character(cmd) => Ok(cmd.execute_via_item(core_item)?),
            Self::Charge(cmd) => Ok(cmd.execute(core_item)?),
            Self::Drone(cmd) => Ok(cmd.execute(core_item)?),
            Self::Fighter(cmd) => Ok(cmd.execute(core_item)?),
            Self::FwEffect(cmd) => Ok(cmd.execute(core_item)?),
            Self::Implant(cmd) => Ok(cmd.execute(core_item)?),
            Self::Module(cmd) => Ok(cmd.execute(core_item)?),
            Self::ProjEffect(cmd) => Ok(cmd.execute(core_item)?),
            Self::Rig(cmd) => Ok(cmd.execute(core_item)?),
            Self::Service(cmd) => Ok(cmd.execute(core_item)?),
            Self::Ship(cmd) => Ok(cmd.execute_via_item(core_item)?),
            Self::Skill(cmd) => Ok(cmd.execute(core_item)?),
            Self::Stance(cmd) => Ok(cmd.execute_via_item(core_item)?),
            Self::Subsystem(cmd) => Ok(cmd.execute(core_item)?),
            Self::SwEffect(cmd) => Ok(cmd.execute(core_item)?),
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
    Character(#[from] ItemCharacterChangeError),
    #[error("failed to change charge")]
    Charge(#[from] ChargeChangeError),
    #[error("failed to change drone")]
    Drone(#[from] DroneChangeError),
    #[error("failed to change fighter")]
    Fighter(#[from] FighterChangeError),
    #[error("failed to change fit-wide effect")]
    FwEffect(#[from] FwEffectChangeError),
    #[error("failed to change implant")]
    Implant(#[from] ImplantChangeError),
    #[error("failed to change module")]
    Module(#[from] ModuleChangeError),
    #[error("failed to change projected effect")]
    ProjEffect(#[from] ProjEffectChangeError),
    #[error("failed to change rig")]
    Rig(#[from] RigChangeError),
    #[error("failed to change service")]
    Service(#[from] ServiceChangeError),
    #[error("failed to change ship")]
    Ship(#[from] ItemShipChangeError),
    #[error("failed to change skill")]
    Skill(#[from] SkillChangeError),
    #[error("failed to change stance")]
    Stance(#[from] ItemStanceChangeError),
    #[error("failed to change subsystem")]
    Subsystem(#[from] SubsystemChangeError),
    #[error("failed to change system-wide effect")]
    SwEffect(#[from] SwEffectChangeError),
}
