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
#[derive(Clone)]
pub enum ItemChangeEnumCmd {
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
    pub fn into_item_ctl(self) -> ItemChangeEnumCmd {
        ItemChangeEnumCmd::Autocharge(self)
    }
}
impl BoosterChangeCmd {
    pub fn into_item_ctl(self) -> ItemChangeEnumCmd {
        ItemChangeEnumCmd::Booster(self)
    }
}
impl CharacterChangeCmd {
    pub fn into_item_ctl(self) -> ItemChangeEnumCmd {
        ItemChangeEnumCmd::Character(self)
    }
}
impl ChargeChangeCmd {
    pub fn into_item_ctl(self) -> ItemChangeEnumCmd {
        ItemChangeEnumCmd::Charge(self)
    }
}
impl DroneChangeCmd {
    pub fn into_item_ctl(self) -> ItemChangeEnumCmd {
        ItemChangeEnumCmd::Drone(self)
    }
}
impl FighterChangeCmd {
    pub fn into_item_ctl(self) -> ItemChangeEnumCmd {
        ItemChangeEnumCmd::Fighter(self)
    }
}
impl FwEffectChangeCmd {
    pub fn into_item_ctl(self) -> ItemChangeEnumCmd {
        ItemChangeEnumCmd::FwEffect(self)
    }
}
impl ImplantChangeCmd {
    pub fn into_item_ctl(self) -> ItemChangeEnumCmd {
        ItemChangeEnumCmd::Implant(self)
    }
}
impl ModuleChangeCmd {
    pub fn into_item_ctl(self) -> ItemChangeEnumCmd {
        ItemChangeEnumCmd::Module(self)
    }
}
impl ProjEffectChangeCmd {
    pub fn into_item_ctl(self) -> ItemChangeEnumCmd {
        ItemChangeEnumCmd::ProjEffect(self)
    }
}
impl RigChangeCmd {
    pub fn into_item_ctl(self) -> ItemChangeEnumCmd {
        ItemChangeEnumCmd::Rig(self)
    }
}
impl ServiceChangeCmd {
    pub fn into_item_ctl(self) -> ItemChangeEnumCmd {
        ItemChangeEnumCmd::Service(self)
    }
}
impl ShipChangeCmd {
    pub fn into_item_ctl(self) -> ItemChangeEnumCmd {
        ItemChangeEnumCmd::Ship(self)
    }
}
impl SkillChangeCmd {
    pub fn into_item_ctl(self) -> ItemChangeEnumCmd {
        ItemChangeEnumCmd::Skill(self)
    }
}
impl StanceChangeCmd {
    pub fn into_item_ctl(self) -> ItemChangeEnumCmd {
        ItemChangeEnumCmd::Stance(self)
    }
}
impl SubsystemChangeCmd {
    pub fn into_item_ctl(self) -> ItemChangeEnumCmd {
        ItemChangeEnumCmd::Subsystem(self)
    }
}
impl SwEffectChangeCmd {
    pub fn into_item_ctl(self) -> ItemChangeEnumCmd {
        ItemChangeEnumCmd::SwEffect(self)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemChangeEnumCmd {
    pub(crate) fn execute(self, core_item: &mut rc::ItemMut) -> Result<ChangedItemIdsResp, ItemChangeEnumError> {
        Ok(match self {
            Self::Autocharge(cmd) => cmd.execute(core_item)?,
            Self::Booster(cmd) => cmd.execute(core_item)?,
            Self::Character(cmd) => cmd.execute_via_item(core_item)?,
            Self::Charge(cmd) => cmd.execute(core_item)?,
            Self::Drone(cmd) => cmd.execute(core_item)?,
            Self::Fighter(cmd) => cmd.execute(core_item)?,
            Self::FwEffect(cmd) => cmd.execute(core_item)?,
            Self::Implant(cmd) => cmd.execute(core_item)?,
            Self::Module(cmd) => cmd.execute(core_item)?,
            Self::ProjEffect(cmd) => cmd.execute(core_item)?,
            Self::Rig(cmd) => cmd.execute(core_item)?,
            Self::Service(cmd) => cmd.execute(core_item)?,
            Self::Ship(cmd) => cmd.execute_via_item(core_item)?,
            Self::Skill(cmd) => cmd.execute(core_item)?,
            Self::Stance(cmd) => cmd.execute_via_item(core_item)?,
            Self::Subsystem(cmd) => cmd.execute(core_item)?,
            Self::SwEffect(cmd) => cmd.execute(core_item)?,
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ItemChangeEnumError {
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
