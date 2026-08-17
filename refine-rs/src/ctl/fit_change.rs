use crate::{
    AutochargeChangeCmd, BoosterAddCmd, BoosterChangeCmd, CharacterChangeCmd, CharacterSetCmd, CharacterUnsetCmd,
    ChargeChangeCmd, CmdResp, CmdResps, DroneAddCmd, DroneAddCmdBr, DroneChangeCmd, DroneChangeCmdBr, FighterAddCmd,
    FighterAddCmdBr, FighterChangeCmd, FighterChangeCmdBr, FitChangeCmd, FwEffectAddCmd, FwEffectChangeCmd,
    ImplantAddCmd, ImplantChangeCmd, ItemId, ItemIdBr, ItemRemoveCmd, ModuleAddCmd, ModuleAddCmdBr, ModuleChangeCmd,
    ModuleChangeCmdBr, RigAddCmd, RigChangeCmd, ServiceAddCmd, ServiceChangeCmd, ShipChangeCmd, ShipSetCmd,
    ShipUnsetCmd, SkillAddCmd, SkillChangeCmd, StanceChangeCmd, StanceSetCmd, StanceUnsetCmd, SubsystemAddCmd,
    SubsystemChangeCmd,
    ctl::core::{
        AutochargeChangeCmdCtxItem, AutochargeChangeCmdCtxItemBr, BoosterChangeCmdCtxItem, BoosterChangeCmdCtxItemBr,
        ChargeChangeCmdCtxItem, ChargeChangeCmdCtxItemBr, DroneChangeCmdCtxItem, DroneChangeCmdCtxItemBr,
        FighterChangeCmdCtxItem, FighterChangeCmdCtxItemBr, FwEffectChangeCmdCtxItem, FwEffectChangeCmdCtxItemBr,
        ImplantChangeCmdCtxItem, ImplantChangeCmdCtxItemBr, ItemRemoveCmdCtxItem, ItemRemoveCmdCtxItemBr,
        ModuleChangeCmdCtxItem, ModuleChangeCmdCtxItemBr, RigChangeCmdCtxItem, RigChangeCmdCtxItemBr,
        ServiceChangeCmdCtxItem, ServiceChangeCmdCtxItemBr, SkillChangeCmdCtxItem, SkillChangeCmdCtxItemBr,
        SubsystemChangeCmdCtxItem, SubsystemChangeCmdCtxItemBr,
    },
    err::{
        BrResolveError, DroneAddError, FighterAddError, FitChangeError, FitCharacterChangeError, FitShipChangeError,
        FitStanceChangeError, ItemGetAutochargeChangeError, ItemGetBoosterChangeError, ItemGetChargeChangeError,
        ItemGetDroneChangeError, ItemGetFighterChangeError, ItemGetFwEffectChangeError, ItemGetImplantChangeError,
        ItemGetItemRemoveError, ItemGetModuleChangeError, ItemGetRigChangeError, ItemGetServiceChangeError,
        ItemGetSkillChangeError, ItemGetSubsystemChangeError, ModuleAddError, SkillAddError,
    },
};

pub enum FitChangeEnumCmd {
    // Fit
    FitChange(FitChangeCmd),
    // Item
    ItemRemove(ItemRemoveCmdCtxItem),
    // Item - autocharge
    AutochargeChange(AutochargeChangeCmdCtxItem),
    // Item - booster
    BoosterAdd(BoosterAddCmd),
    BoosterChange(BoosterChangeCmdCtxItem),
    // Item - character
    CharacterSet(CharacterSetCmd),
    CharacterChange(CharacterChangeCmd),
    CharacterUnset(CharacterUnsetCmd),
    // Item - charge
    ChargeChange(ChargeChangeCmdCtxItem),
    // Item - drone
    DroneAdd(DroneAddCmd),
    DroneChange(DroneChangeCmdCtxItem),
    // Item - fighter
    FighterAdd(FighterAddCmd),
    FighterChange(FighterChangeCmdCtxItem),
    // Item - fit-wide effect
    FwEffectAdd(FwEffectAddCmd),
    FwEffectChange(FwEffectChangeCmdCtxItem),
    // Item - implant
    ImplantAdd(ImplantAddCmd),
    ImplantChange(ImplantChangeCmdCtxItem),
    // Item - module
    ModuleAdd(ModuleAddCmd),
    ModuleChange(ModuleChangeCmdCtxItem),
    // Item - rig
    RigAdd(RigAddCmd),
    RigChange(RigChangeCmdCtxItem),
    // Item - service
    ServiceAdd(ServiceAddCmd),
    ServiceChange(ServiceChangeCmdCtxItem),
    // Item - ship
    ShipSet(ShipSetCmd),
    ShipChange(ShipChangeCmd),
    ShipUnset(ShipUnsetCmd),
    // Item - skill
    SkillAdd(SkillAddCmd),
    SkillChange(SkillChangeCmdCtxItem),
    // Item - stance
    StanceSet(StanceSetCmd),
    StanceChange(StanceChangeCmd),
    StanceUnset(StanceUnsetCmd),
    // Item - subsystem
    SubsystemAdd(SubsystemAddCmd),
    SubsystemChange(SubsystemChangeCmdCtxItem),
}

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(tag = "type", rename_all = "snake_case")
)]
pub enum FitChangeEnumCmdBr {
    // Fit
    FitChange(FitChangeCmd),
    // Item
    ItemRemove(ItemRemoveCmdCtxItemBr),
    // Item - autocharge
    AutochargeChange(AutochargeChangeCmdCtxItemBr),
    // Item - booster
    BoosterAdd(BoosterAddCmd),
    BoosterChange(BoosterChangeCmdCtxItemBr),
    // Item - character
    CharacterSet(CharacterSetCmd),
    CharacterChange(CharacterChangeCmd),
    CharacterUnset(CharacterUnsetCmd),
    // Item - charge
    ChargeChange(ChargeChangeCmdCtxItemBr),
    // Item - drone
    DroneAdd(DroneAddCmdBr),
    DroneChange(DroneChangeCmdCtxItemBr),
    // Item - fighter
    FighterAdd(FighterAddCmdBr),
    FighterChange(FighterChangeCmdCtxItemBr),
    // Item - fit-wide effect
    FwEffectAdd(FwEffectAddCmd),
    FwEffectChange(FwEffectChangeCmdCtxItemBr),
    // Item - implant
    ImplantAdd(ImplantAddCmd),
    ImplantChange(ImplantChangeCmdCtxItemBr),
    // Item - module
    ModuleAdd(ModuleAddCmdBr),
    ModuleChange(ModuleChangeCmdCtxItemBr),
    // Item - rig
    RigAdd(RigAddCmd),
    RigChange(RigChangeCmdCtxItemBr),
    // Item - service
    ServiceAdd(ServiceAddCmd),
    ServiceChange(ServiceChangeCmdCtxItemBr),
    // Item - ship
    ShipSet(ShipSetCmd),
    ShipChange(ShipChangeCmd),
    ShipUnset(ShipUnsetCmd),
    // Item - skill
    SkillAdd(SkillAddCmd),
    SkillChange(SkillChangeCmdCtxItemBr),
    // Item - stance
    StanceSet(StanceSetCmd),
    StanceChange(StanceChangeCmd),
    StanceUnset(StanceUnsetCmd),
    // Item - subsystem
    SubsystemAdd(SubsystemAddCmd),
    SubsystemChange(SubsystemChangeCmdCtxItemBr),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
// Fit
impl FitChangeCmd {
    pub fn into_fit(self) -> FitChangeEnumCmd {
        FitChangeEnumCmd::FitChange(self)
    }
    pub fn into_fit_br(self) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::FitChange(self)
    }
}
// Item
impl ItemRemoveCmd {
    pub fn into_fit(self, item_id: ItemId) -> FitChangeEnumCmd {
        FitChangeEnumCmd::ItemRemove(self.into_ctx_item(item_id))
    }
    pub fn into_fit_br(self, item_id: impl Into<ItemIdBr>) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::ItemRemove(self.into_ctx_item_br(item_id))
    }
}
// Item - autocharge
impl AutochargeChangeCmd {
    pub fn into_fit(self, item_id: ItemId) -> FitChangeEnumCmd {
        FitChangeEnumCmd::AutochargeChange(self.into_ctx_item(item_id))
    }
    pub fn into_fit_br(self, item_id: impl Into<ItemIdBr>) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::AutochargeChange(self.into_ctx_item_br(item_id))
    }
}
// Item - booster
impl BoosterAddCmd {
    pub fn into_fit(self) -> FitChangeEnumCmd {
        FitChangeEnumCmd::BoosterAdd(self)
    }
    pub fn into_fit_br(self) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::BoosterAdd(self)
    }
}
impl BoosterChangeCmd {
    pub fn into_fit(self, item_id: ItemId) -> FitChangeEnumCmd {
        FitChangeEnumCmd::BoosterChange(self.into_ctx_item(item_id))
    }
    pub fn into_fit_br(self, item_id: impl Into<ItemIdBr>) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::BoosterChange(self.into_ctx_item_br(item_id))
    }
}
// Item - character
impl CharacterSetCmd {
    pub fn into_fit(self) -> FitChangeEnumCmd {
        FitChangeEnumCmd::CharacterSet(self)
    }
    pub fn into_fit_br(self) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::CharacterSet(self)
    }
}
impl CharacterChangeCmd {
    pub fn into_fit(self) -> FitChangeEnumCmd {
        FitChangeEnumCmd::CharacterChange(self)
    }
    pub fn into_fit_br(self) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::CharacterChange(self)
    }
}
impl CharacterUnsetCmd {
    pub fn into_fit(self) -> FitChangeEnumCmd {
        FitChangeEnumCmd::CharacterUnset(self)
    }
    pub fn into_fit_br(self) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::CharacterUnset(self)
    }
}
// Item - charge
impl ChargeChangeCmd {
    pub fn into_fit(self, item_id: ItemId) -> FitChangeEnumCmd {
        FitChangeEnumCmd::ChargeChange(self.into_ctx_item(item_id))
    }
    pub fn into_fit_br(self, item_id: impl Into<ItemIdBr>) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::ChargeChange(self.into_ctx_item_br(item_id))
    }
}
// Item - drone
impl DroneAddCmd {
    pub fn into_fit(self) -> FitChangeEnumCmd {
        FitChangeEnumCmd::DroneAdd(self)
    }
    pub fn into_fit_br(self) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::DroneAdd(self.into_br())
    }
}
impl DroneAddCmdBr {
    pub fn into_fit_br(self) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::DroneAdd(self)
    }
}
impl DroneChangeCmd {
    pub fn into_fit(self, item_id: ItemId) -> FitChangeEnumCmd {
        FitChangeEnumCmd::DroneChange(self.into_ctx_item(item_id))
    }
    pub fn into_fit_br(self, item_id: impl Into<ItemIdBr>) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::DroneChange(self.into_ctx_item_br(item_id))
    }
}
impl DroneChangeCmdBr {
    pub fn into_fit_br(self, item_id: impl Into<ItemIdBr>) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::DroneChange(self.into_ctx_item_br(item_id))
    }
}
// Item - fighter
impl FighterAddCmd {
    pub fn into_fit(self) -> FitChangeEnumCmd {
        FitChangeEnumCmd::FighterAdd(self)
    }
    pub fn into_fit_br(self) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::FighterAdd(self.into_br())
    }
}
impl FighterAddCmdBr {
    pub fn into_fit_br(self) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::FighterAdd(self)
    }
}
impl FighterChangeCmd {
    pub fn into_fit(self, item_id: ItemId) -> FitChangeEnumCmd {
        FitChangeEnumCmd::FighterChange(self.into_ctx_item(item_id))
    }
    pub fn into_fit_br(self, item_id: impl Into<ItemIdBr>) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::FighterChange(self.into_ctx_item_br(item_id))
    }
}
impl FighterChangeCmdBr {
    pub fn into_fit_br(self, item_id: impl Into<ItemIdBr>) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::FighterChange(self.into_ctx_item_br(item_id))
    }
}
// Item - fit-wide effect
impl FwEffectAddCmd {
    pub fn into_fit(self) -> FitChangeEnumCmd {
        FitChangeEnumCmd::FwEffectAdd(self)
    }
    pub fn into_fit_br(self) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::FwEffectAdd(self)
    }
}
impl FwEffectChangeCmd {
    pub fn into_fit(self, item_id: ItemId) -> FitChangeEnumCmd {
        FitChangeEnumCmd::FwEffectChange(self.into_ctx_item(item_id))
    }
    pub fn into_fit_br(self, item_id: impl Into<ItemIdBr>) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::FwEffectChange(self.into_ctx_item_br(item_id))
    }
}
// Item - implant
impl ImplantAddCmd {
    pub fn into_fit(self) -> FitChangeEnumCmd {
        FitChangeEnumCmd::ImplantAdd(self)
    }
    pub fn into_fit_br(self) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::ImplantAdd(self)
    }
}
impl ImplantChangeCmd {
    pub fn into_fit(self, item_id: ItemId) -> FitChangeEnumCmd {
        FitChangeEnumCmd::ImplantChange(self.into_ctx_item(item_id))
    }
    pub fn into_fit_br(self, item_id: impl Into<ItemIdBr>) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::ImplantChange(self.into_ctx_item_br(item_id))
    }
}
// Item - module
impl ModuleAddCmd {
    pub fn into_fit(self) -> FitChangeEnumCmd {
        FitChangeEnumCmd::ModuleAdd(self)
    }
    pub fn into_fit_br(self) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::ModuleAdd(self.into_br())
    }
}
impl ModuleAddCmdBr {
    pub fn into_fit_br(self) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::ModuleAdd(self)
    }
}
impl ModuleChangeCmd {
    pub fn into_fit(self, item_id: ItemId) -> FitChangeEnumCmd {
        FitChangeEnumCmd::ModuleChange(self.into_ctx_item(item_id))
    }
    pub fn into_fit_br(self, item_id: impl Into<ItemIdBr>) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::ModuleChange(self.into_ctx_item_br(item_id))
    }
}
impl ModuleChangeCmdBr {
    pub fn into_fit_br(self, item_id: impl Into<ItemIdBr>) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::ModuleChange(self.into_ctx_item_br(item_id))
    }
}
// Item - rig
impl RigAddCmd {
    pub fn into_fit(self) -> FitChangeEnumCmd {
        FitChangeEnumCmd::RigAdd(self)
    }
    pub fn into_fit_br(self) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::RigAdd(self)
    }
}
impl RigChangeCmd {
    pub fn into_fit(self, item_id: ItemId) -> FitChangeEnumCmd {
        FitChangeEnumCmd::RigChange(self.into_ctx_item(item_id))
    }
    pub fn into_fit_br(self, item_id: impl Into<ItemIdBr>) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::RigChange(self.into_ctx_item_br(item_id))
    }
}
// Item - service
impl ServiceAddCmd {
    pub fn into_fit(self) -> FitChangeEnumCmd {
        FitChangeEnumCmd::ServiceAdd(self)
    }
    pub fn into_fit_br(self) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::ServiceAdd(self)
    }
}
impl ServiceChangeCmd {
    pub fn into_fit(self, item_id: ItemId) -> FitChangeEnumCmd {
        FitChangeEnumCmd::ServiceChange(self.into_ctx_item(item_id))
    }
    pub fn into_fit_br(self, item_id: impl Into<ItemIdBr>) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::ServiceChange(self.into_ctx_item_br(item_id))
    }
}
// Item - ship
impl ShipSetCmd {
    pub fn into_fit(self) -> FitChangeEnumCmd {
        FitChangeEnumCmd::ShipSet(self)
    }
    pub fn into_fit_br(self) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::ShipSet(self)
    }
}
impl ShipChangeCmd {
    pub fn into_fit(self) -> FitChangeEnumCmd {
        FitChangeEnumCmd::ShipChange(self)
    }
    pub fn into_fit_br(self) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::ShipChange(self)
    }
}
impl ShipUnsetCmd {
    pub fn into_fit(self) -> FitChangeEnumCmd {
        FitChangeEnumCmd::ShipUnset(self)
    }
    pub fn into_fit_br(self) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::ShipUnset(self)
    }
}
// Item - skill
impl SkillAddCmd {
    pub fn into_fit(self) -> FitChangeEnumCmd {
        FitChangeEnumCmd::SkillAdd(self)
    }
    pub fn into_fit_br(self) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::SkillAdd(self)
    }
}
impl SkillChangeCmd {
    pub fn into_fit(self, item_id: ItemId) -> FitChangeEnumCmd {
        FitChangeEnumCmd::SkillChange(self.into_ctx_item(item_id))
    }
    pub fn into_fit_br(self, item_id: impl Into<ItemIdBr>) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::SkillChange(self.into_ctx_item_br(item_id))
    }
}
// Item - stance
impl StanceSetCmd {
    pub fn into_fit(self) -> FitChangeEnumCmd {
        FitChangeEnumCmd::StanceSet(self)
    }
    pub fn into_fit_br(self) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::StanceSet(self)
    }
}
impl StanceChangeCmd {
    pub fn into_fit(self) -> FitChangeEnumCmd {
        FitChangeEnumCmd::StanceChange(self)
    }
    pub fn into_fit_br(self) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::StanceChange(self)
    }
}
impl StanceUnsetCmd {
    pub fn into_fit(self) -> FitChangeEnumCmd {
        FitChangeEnumCmd::StanceUnset(self)
    }
    pub fn into_fit_br(self) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::StanceUnset(self)
    }
}
// Item - subsystem
impl SubsystemAddCmd {
    pub fn into_fit(self) -> FitChangeEnumCmd {
        FitChangeEnumCmd::SubsystemAdd(self)
    }
    pub fn into_fit_br(self) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::SubsystemAdd(self)
    }
}
impl SubsystemChangeCmd {
    pub fn into_fit(self, item_id: ItemId) -> FitChangeEnumCmd {
        FitChangeEnumCmd::SubsystemChange(self.into_ctx_item(item_id))
    }
    pub fn into_fit_br(self, item_id: impl Into<ItemIdBr>) -> FitChangeEnumCmdBr {
        FitChangeEnumCmdBr::SubsystemChange(self.into_ctx_item_br(item_id))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitChangeEnumCmdBr {
    pub(crate) fn render(self, resps: &CmdResps) -> Result<FitChangeEnumCmd, BrResolveError> {
        Ok(match self {
            // Fit
            Self::FitChange(cmd) => FitChangeEnumCmd::FitChange(cmd),
            // Item
            Self::ItemRemove(cmd) => FitChangeEnumCmd::ItemRemove(cmd.render(resps)?),
            // Item - autocharge
            Self::AutochargeChange(cmd) => FitChangeEnumCmd::AutochargeChange(cmd.render(resps)?),
            // Item - booster
            Self::BoosterAdd(cmd) => FitChangeEnumCmd::BoosterAdd(cmd),
            Self::BoosterChange(cmd) => FitChangeEnumCmd::BoosterChange(cmd.render(resps)?),
            // Item - character
            Self::CharacterSet(cmd) => FitChangeEnumCmd::CharacterSet(cmd),
            Self::CharacterChange(cmd) => FitChangeEnumCmd::CharacterChange(cmd),
            Self::CharacterUnset(cmd) => FitChangeEnumCmd::CharacterUnset(cmd),
            // Item - charge
            Self::ChargeChange(cmd) => FitChangeEnumCmd::ChargeChange(cmd.render(resps)?),
            // Item - drone
            Self::DroneAdd(cmd) => FitChangeEnumCmd::DroneAdd(cmd.render(resps)?),
            Self::DroneChange(cmd) => FitChangeEnumCmd::DroneChange(cmd.render(resps)?),
            // Item - fighter
            Self::FighterAdd(cmd) => FitChangeEnumCmd::FighterAdd(cmd.render(resps)?),
            Self::FighterChange(cmd) => FitChangeEnumCmd::FighterChange(cmd.render(resps)?),
            // Item - fit-wide effect
            Self::FwEffectAdd(cmd) => FitChangeEnumCmd::FwEffectAdd(cmd),
            Self::FwEffectChange(cmd) => FitChangeEnumCmd::FwEffectChange(cmd.render(resps)?),
            // Item - implant
            Self::ImplantAdd(cmd) => FitChangeEnumCmd::ImplantAdd(cmd),
            Self::ImplantChange(cmd) => FitChangeEnumCmd::ImplantChange(cmd.render(resps)?),
            // Item - drone
            Self::ModuleAdd(cmd) => FitChangeEnumCmd::ModuleAdd(cmd.render(resps)?),
            Self::ModuleChange(cmd) => FitChangeEnumCmd::ModuleChange(cmd.render(resps)?),
            // Item - rig
            Self::RigAdd(cmd) => FitChangeEnumCmd::RigAdd(cmd),
            Self::RigChange(cmd) => FitChangeEnumCmd::RigChange(cmd.render(resps)?),
            // Item - service
            Self::ServiceAdd(cmd) => FitChangeEnumCmd::ServiceAdd(cmd),
            Self::ServiceChange(cmd) => FitChangeEnumCmd::ServiceChange(cmd.render(resps)?),
            // Item - ship
            Self::ShipSet(cmd) => FitChangeEnumCmd::ShipSet(cmd),
            Self::ShipChange(cmd) => FitChangeEnumCmd::ShipChange(cmd),
            Self::ShipUnset(cmd) => FitChangeEnumCmd::ShipUnset(cmd),
            // Item - skill
            Self::SkillAdd(cmd) => FitChangeEnumCmd::SkillAdd(cmd),
            Self::SkillChange(cmd) => FitChangeEnumCmd::SkillChange(cmd.render(resps)?),
            // Item - stance
            Self::StanceSet(cmd) => FitChangeEnumCmd::StanceSet(cmd),
            Self::StanceChange(cmd) => FitChangeEnumCmd::StanceChange(cmd),
            Self::StanceUnset(cmd) => FitChangeEnumCmd::StanceUnset(cmd),
            // Item - subsystem
            Self::SubsystemAdd(cmd) => FitChangeEnumCmd::SubsystemAdd(cmd),
            Self::SubsystemChange(cmd) => FitChangeEnumCmd::SubsystemChange(cmd.render(resps)?),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitChangeEnumCmd {
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut) -> Result<CmdResp, FitChangeEnumError> {
        match self {
            // Fit
            Self::FitChange(cmd) => Ok(cmd.execute(core_fit)?.into()),
            // Item
            Self::ItemRemove(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - autocharge
            Self::AutochargeChange(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - booster
            Self::BoosterAdd(cmd) => Ok(cmd.execute(core_fit).into()),
            Self::BoosterChange(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - character
            Self::CharacterSet(cmd) => Ok(cmd.execute(core_fit).into()),
            Self::CharacterChange(cmd) => Ok(cmd.execute_via_fit(core_fit)?.into()),
            #[expect(clippy::unit_arg)]
            Self::CharacterUnset(cmd) => Ok(cmd.execute(core_fit).into()),
            // Item - charge
            Self::ChargeChange(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - drone
            Self::DroneAdd(cmd) => Ok(cmd.execute(core_fit)?.into()),
            Self::DroneChange(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - fighter
            Self::FighterAdd(cmd) => Ok(cmd.execute(core_fit)?.into()),
            Self::FighterChange(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - fit-wide effect
            Self::FwEffectAdd(cmd) => Ok(cmd.execute(core_fit).into()),
            Self::FwEffectChange(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - implant
            Self::ImplantAdd(cmd) => Ok(cmd.execute(core_fit).into()),
            Self::ImplantChange(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - module
            Self::ModuleAdd(cmd) => Ok(cmd.execute(core_fit)?.into()),
            Self::ModuleChange(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - rig
            Self::RigAdd(cmd) => Ok(cmd.execute(core_fit).into()),
            Self::RigChange(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - service
            Self::ServiceAdd(cmd) => Ok(cmd.execute(core_fit).into()),
            Self::ServiceChange(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - ship
            Self::ShipSet(cmd) => Ok(cmd.execute(core_fit).into()),
            Self::ShipChange(cmd) => Ok(cmd.execute_via_fit(core_fit)?.into()),
            #[expect(clippy::unit_arg)]
            Self::ShipUnset(cmd) => Ok(cmd.execute(core_fit).into()),
            // Item - skill
            Self::SkillAdd(cmd) => Ok(cmd.execute(core_fit)?.into()),
            Self::SkillChange(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - stance
            Self::StanceSet(cmd) => Ok(cmd.execute(core_fit).into()),
            Self::StanceChange(cmd) => Ok(cmd.execute_via_fit(core_fit)?.into()),
            #[expect(clippy::unit_arg)]
            Self::StanceUnset(cmd) => Ok(cmd.execute(core_fit).into()),
            // Item - subsystem
            Self::SubsystemAdd(cmd) => Ok(cmd.execute(core_fit).into()),
            Self::SubsystemChange(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FitChangeEnumError {
    // Fit
    #[error("failed to change fleet")]
    FitChange(#[from] FitChangeError),
    // Item
    #[error("failed to remove item")]
    ItemRemove(#[from] ItemGetItemRemoveError),
    // Item - autocharge
    #[error("failed to change autocharge")]
    AutochargeChange(#[from] ItemGetAutochargeChangeError),
    // Item - booster
    #[error("failed to change booster")]
    BoosterChange(#[from] ItemGetBoosterChangeError),
    // Item - character
    #[error("failed to change character")]
    CharacterChange(#[from] FitCharacterChangeError),
    // Item - charge
    #[error("failed to change charge")]
    ChargeChange(#[from] ItemGetChargeChangeError),
    // Item - drone
    #[error("failed to add drone")]
    DroneAdd(#[from] DroneAddError),
    #[error("failed to change drone")]
    DroneChange(#[from] ItemGetDroneChangeError),
    // Item - fighter
    #[error("failed to add fighter")]
    FighterAdd(#[from] FighterAddError),
    #[error("failed to change fighter")]
    FighterChange(#[from] ItemGetFighterChangeError),
    // Item - fit-wide effect
    #[error("failed to change fit-wide effect")]
    FwEffectChange(#[from] ItemGetFwEffectChangeError),
    // Item - implant
    #[error("failed to change implant")]
    ImplantChange(#[from] ItemGetImplantChangeError),
    // Item - module
    #[error("failed to add module")]
    ModuleAdd(#[from] ModuleAddError),
    #[error("failed to change module")]
    ModuleChange(#[from] ItemGetModuleChangeError),
    // Item - rig
    #[error("failed to change rig")]
    RigChange(#[from] ItemGetRigChangeError),
    // Item - service
    #[error("failed to change service")]
    ServiceChange(#[from] ItemGetServiceChangeError),
    // Item - ship
    #[error("failed to change ship")]
    ShipChange(#[from] FitShipChangeError),
    // Item - skill
    #[error("failed to add skill")]
    SkillAdd(#[from] SkillAddError),
    #[error("failed to change skill")]
    SkillChange(#[from] ItemGetSkillChangeError),
    // Item - stance
    #[error("failed to change stance")]
    StanceChange(#[from] FitStanceChangeError),
    // Item - subsystem
    #[error("failed to change subsystem")]
    SubsystemChange(#[from] ItemGetSubsystemChangeError),
}
