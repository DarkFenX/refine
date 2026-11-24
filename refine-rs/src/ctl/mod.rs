pub use core::{
    AddMutation, AddedFitIdResp, AddedFleetIdResp, AddedItemIdsResp, AttrMutation, AutochargeChangeCmd,
    AutochargeItemIds, BoosterAddCmd, BoosterChangeCmd, ChangeMutation, ChangedItemIdsResp, CharacterChangeCmd,
    CharacterSetCmd, CharacterUnsetCmd, ChargeChangeCmd, DroneAddCmd, DroneAddCmdBr, DroneChangeCmd, DroneChangeCmdBr,
    FighterAddCmd, FighterAddCmdBr, FighterChangeCmd, FighterChangeCmdBr, FitAddCmd, FitAddCmdBr, FitChangeCmd,
    FitChangeCmdBr, FitRemoveCmd, FleetAddCmd, FleetAddCmdBr, FleetChangeCmd, FleetChangeCmdBr, FleetRemoveCmd,
    FwEffectAddCmd, FwEffectChangeCmd, ImplantAddCmd, ImplantChangeCmd, ItemAutodetectAddCmd, ItemRemoveCmd,
    ModuleAddCmd, ModuleAddCmdBr, ModuleChangeCmd, ModuleChangeCmdBr, ProjEffectAddCmd, ProjEffectAddCmdBr,
    ProjEffectChangeCmd, ProjEffectChangeCmdBr, RigAddCmd, RigChangeCmd, ServiceAddCmd, ServiceChangeCmd,
    ShipChangeCmd, ShipSetCmd, ShipUnsetCmd, SkillAddCmd, SkillChangeCmd, SolAddCmd, SolChangeCmd, StanceChangeCmd,
    StanceSetCmd, StanceUnsetCmd, SubsystemAddCmd, SubsystemChangeCmd, SwEffectAddCmd, SwEffectChangeCmd,
};
pub(crate) use core::{
    DroneAddCmdGen, FighterAddCmdGen, FitAddCmdGen, FleetAddCmdGen, ModuleAddCmdGen, ProjEffectAddCmdGen,
};

pub use fit_change::{FitChangeEnumCmd, FitChangeEnumCmdBr};
pub use item_add::ItemAddEnumCmd;
pub use item_change::ItemChangeEnumCmd;
pub use sol_change::{SolChangeEnumCmd, SolChangeEnumCmdBr};

mod core;
mod fit_change;
mod item_add;
mod item_change;
mod sol_change;

pub mod err {
    pub use crate::{
        api::{SolAddError, SolRemoveError, SolSwitchSrcError},
        ctl::{
            core::{
                AutochargeChangeError, BoosterChangeError, CharacterChangeError, ChargeChangeError, DroneAddError,
                DroneChangeError, FighterAddError, FighterChangeError, FitAddError, FitChangeError,
                FitCharacterChangeError, FitGetBoosterAddError, FitGetCharacterChangeError, FitGetCharacterSetError,
                FitGetCharacterUnsetError, FitGetDroneAddError, FitGetFighterAddError, FitGetFitChangeError,
                FitGetFitRemoveError, FitGetFwEffectAddError, FitGetImplantAddError, FitGetItemAutodetectAddError,
                FitGetModuleAddError, FitGetRigAddError, FitGetServiceAddError, FitGetShipChangeError,
                FitGetShipSetError, FitGetShipUnsetError, FitGetSkillAddError, FitGetStanceChangeError,
                FitGetStanceSetError, FitGetStanceUnsetError, FitGetSubsystemAddError, FitItemGetAutochargeChangeError,
                FitItemGetBoosterChangeError, FitItemGetChargeChangeError, FitItemGetDroneChangeError,
                FitItemGetFighterChangeError, FitItemGetFwEffectChangeError, FitItemGetImplantChangeError,
                FitItemGetItemRemoveError, FitItemGetModuleChangeError, FitItemGetRigChangeError,
                FitItemGetServiceChangeError, FitItemGetSkillChangeError, FitItemGetSubsystemChangeError,
                FitShipChangeError, FitStanceChangeError, FleetAddError, FleetChangeError, FleetGetFleetChangeError,
                FleetGetFleetRemoveError, FwEffectChangeError, GenItemGetAutochargeChangeError,
                GenItemGetBoosterChangeError, GenItemGetChargeChangeError, GenItemGetDroneChangeError,
                GenItemGetFighterChangeError, GenItemGetFwEffectChangeError, GenItemGetImplantChangeError,
                GenItemGetItemRemoveError, GenItemGetModuleChangeError, GenItemGetRigChangeError,
                GenItemGetServiceChangeError, GenItemGetSkillChangeError, GenItemGetSubsystemChangeError,
                ImplantChangeError, ItemAutodetectAddError, ItemCharacterChangeError, ItemGetAutochargeChangeError,
                ItemGetBoosterChangeError, ItemGetCharacterChangeError, ItemGetChargeChangeError,
                ItemGetDroneChangeError, ItemGetFighterChangeError, ItemGetFwEffectChangeError,
                ItemGetImplantChangeError, ItemGetItemRemoveError, ItemGetModuleChangeError,
                ItemGetProjEffectChangeError, ItemGetRigChangeError, ItemGetServiceChangeError, ItemGetShipChangeError,
                ItemGetSkillChangeError, ItemGetStanceChangeError, ItemGetSubsystemChangeError,
                ItemGetSwEffectChangeError, ItemRemoveError, ItemShipChangeError, ItemStanceChangeError,
                ModuleAddError, ModuleChangeError, ProjEffectAddError, ProjEffectChangeError, RigChangeError,
                ServiceChangeError, ShipChangeError, SkillAddError, SkillChangeError, StanceChangeError,
                SubsystemChangeError, SwEffectChangeError,
            },
            fit_change::FitChangeEnumError,
            item_add::ItemAddEnumError,
            item_change::ItemChangeEnumError,
            sol_change::SolChangeEnumError,
        },
    };
}
