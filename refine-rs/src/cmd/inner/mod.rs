pub use fit::{AddFitError, FitChangeFitError, GetFitChangeFitError, GetFitRemoveFitError};
pub(in crate::cmd) use fit::{
    ICmdFitAddFCtxBIds, ICmdFitAddFCtxRIds, ICmdFitChangeFCtxBIds, ICmdFitChangeFCtxRIds, ICmdFitChangeICtxBIds,
    ICmdFitChangeICtxRIds, ICmdFitRemoveFCtxBIds, ICmdFitRemoveFCtxRIds, ICmdFitRemoveICtx,
};
pub use fleet::{AddFleetError, FleetChangeFleetError, GetFleetChangeFleetError, GetFleetRemoveFleetError};
pub(in crate::cmd) use fleet::{
    ICmdFleetAddFCtxBIds, ICmdFleetAddFCtxRIds, ICmdFleetChangeFCtxBIds, ICmdFleetChangeFCtxRIds,
    ICmdFleetChangeICtxRIds, ICmdFleetRemoveFCtxBIds, ICmdFleetRemoveFCtxRIds, ICmdFleetRemoveICtx,
};
pub use item::{GetItemRemoveItemError, ItemRemoveItemError};
pub(in crate::cmd) use item::{ICmdItemRemoveFCtxBIds, ICmdItemRemoveFCtxRIds, ICmdItemRemoveICtx};
pub use item_autocharge::{GetItemChangeAutochargeError, ItemChangeAutochargeError};
pub(in crate::cmd) use item_autocharge::{
    ICmdAutochargeChangeFCtxBIds, ICmdAutochargeChangeFCtxRIds, ICmdAutochargeChangeICtx,
};
pub use item_booster::{GetFitAddBoosterError, GetItemChangeBoosterError, ItemChangeBoosterError};
pub(in crate::cmd) use item_booster::{
    ICmdBoosterAddFCtxBIds, ICmdBoosterAddFCtxRIds, ICmdBoosterAddICtx, ICmdBoosterChangeFCtxBIds,
    ICmdBoosterChangeFCtxRIds, ICmdBoosterChangeICtx,
};
pub use item_character::{
    FitChangeCharacterError, GetFitChangeCharacterError, GetFitSetCharacterError, GetFitUnsetCharacterError,
    GetItemChangeCharacterError, ItemChangeCharacterError,
};
pub(in crate::cmd) use item_character::{
    ICmdCharacterChangeFFitCtxBIds, ICmdCharacterChangeFFitCtxRIds, ICmdCharacterChangeFItemCtxBIds,
    ICmdCharacterChangeFItemCtxRIds, ICmdCharacterChangeICtx, ICmdCharacterSetFCtxBIds, ICmdCharacterSetFCtxRIds,
    ICmdCharacterSetICtx, ICmdCharacterUnsetFCtxBIds, ICmdCharacterUnsetFCtxRIds, ICmdCharacterUnsetICtx,
};
pub use item_charge::{GetItemChangeChargeError, ItemChangeChargeError};
pub(in crate::cmd) use item_charge::{ICmdChargeChangeFCtxBIds, ICmdChargeChangeFCtxRIds, ICmdChargeChangeICtx};
pub use item_drone::{FitAddDroneError, GetFitAddDroneError, GetItemChangeDroneError, ItemChangeDroneError};
pub(in crate::cmd) use item_drone::{
    ICmdDroneAddFCtxBIds, ICmdDroneAddFCtxRIds, ICmdDroneAddICtxBIds, ICmdDroneAddICtxRIds, ICmdDroneAddShared,
    ICmdDroneChangeFCtxBIds, ICmdDroneChangeFCtxRIds, ICmdDroneChangeICtxRIds,
};
pub use item_fighter::{FitAddFighterError, GetFitAddFighterError, GetItemChangeFighterError, ItemChangeFighterError};
pub(in crate::cmd) use item_fighter::{
    ICmdFighterAddFCtxBIds, ICmdFighterAddFCtxRIds, ICmdFighterAddICtxBIds, ICmdFighterAddICtxRIds,
    ICmdFighterAddShared, ICmdFighterChangeFCtxBIds, ICmdFighterChangeFCtxRIds, ICmdFighterChangeICtxRIds,
};
pub use item_fw_effect::{GetFitAddFwEffectError, GetItemChangeFwEffectError, ItemChangeFwEffectError};
pub(in crate::cmd) use item_fw_effect::{
    ICmdFwEffectAddFCtxBIds, ICmdFwEffectAddFCtxRIds, ICmdFwEffectAddICtx, ICmdFwEffectChangeFCtxBIds,
    ICmdFwEffectChangeFCtxRIds, ICmdFwEffectChangeICtx,
};
pub use item_implant::{GetFitAddImplantError, GetItemChangeImplantError, ItemChangeImplantError};
pub(in crate::cmd) use item_implant::{
    ICmdImplantAddFCtxBIds, ICmdImplantAddFCtxRIds, ICmdImplantAddICtx, ICmdImplantChangeFCtxBIds,
    ICmdImplantChangeFCtxRIds, ICmdImplantChangeICtx,
};
pub use item_module::{FitAddModuleError, GetFitAddModuleError, GetItemChangeModuleError, ItemChangeModuleError};
pub(in crate::cmd) use item_module::{
    ICmdModuleAddFCtxBIds, ICmdModuleAddFCtxRIds, ICmdModuleAddICtxBIds, ICmdModuleAddICtxRIds, ICmdModuleAddShared,
    ICmdModuleChangeFCtxBIds, ICmdModuleChangeFCtxRIds, ICmdModuleChangeICtxRIds,
};
pub use item_proj_effect::{AddProjEffectError, GetItemChangeProjEffectError, ItemChangeProjEffectError};
pub(in crate::cmd) use item_proj_effect::{
    ICmdProjEffectAddFCtxBIds, ICmdProjEffectAddFCtxRIds, ICmdProjEffectAddShared, ICmdProjEffectChangeFCtxBIds,
    ICmdProjEffectChangeFCtxRIds, ICmdProjEffectChangeICtxRIds,
};
pub use item_rig::{GetFitAddRigError, GetItemChangeRigError, ItemChangeRigError};
pub(in crate::cmd) use item_rig::{
    ICmdRigAddFCtxBIds, ICmdRigAddFCtxRIds, ICmdRigAddICtx, ICmdRigChangeFCtxBIds, ICmdRigChangeFCtxRIds,
    ICmdRigChangeICtx,
};
pub use item_service::{GetFitAddServiceError, GetItemChangeServiceError, ItemChangeServiceError};
pub(in crate::cmd) use item_service::{
    ICmdServiceAddFCtxBIds, ICmdServiceAddFCtxRIds, ICmdServiceAddICtx, ICmdServiceChangeFCtxBIds,
    ICmdServiceChangeFCtxRIds, ICmdServiceChangeICtx,
};
pub use item_ship::{
    FitChangeShipError, GetFitChangeShipError, GetFitSetShipError, GetFitUnsetShipError, GetItemChangeShipError,
    ItemChangeShipError,
};
pub(in crate::cmd) use item_ship::{
    ICmdShipChangeFFitCtxBIds, ICmdShipChangeFFitCtxRIds, ICmdShipChangeFItemCtxBIds, ICmdShipChangeFItemCtxRIds,
    ICmdShipChangeICtx, ICmdShipSetFCtxBIds, ICmdShipSetFCtxRIds, ICmdShipSetICtx, ICmdShipUnsetFCtxBIds,
    ICmdShipUnsetFCtxRIds, ICmdShipUnsetICtx,
};
pub use item_skill::{FitAddSkillError, GetFitAddSkillError, GetItemChangeSkillError, ItemChangeSkillError};
pub(in crate::cmd) use item_skill::{
    ICmdSkillAddFCtxBIds, ICmdSkillAddFCtxRIds, ICmdSkillAddICtx, ICmdSkillChangeFCtxBIds, ICmdSkillChangeFCtxRIds,
    ICmdSkillChangeICtx,
};
pub use item_stance::{
    FitChangeStanceError, GetFitChangeStanceError, GetFitSetStanceError, GetFitUnsetStanceError,
    GetItemChangeStanceError, ItemChangeStanceError,
};
pub(in crate::cmd) use item_stance::{
    ICmdStanceChangeFFitCtxBIds, ICmdStanceChangeFFitCtxRIds, ICmdStanceChangeFItemCtxBIds,
    ICmdStanceChangeFItemCtxRIds, ICmdStanceChangeICtx, ICmdStanceSetFCtxBIds, ICmdStanceSetFCtxRIds,
    ICmdStanceSetICtx, ICmdStanceUnsetFCtxBIds, ICmdStanceUnsetFCtxRIds, ICmdStanceUnsetICtx,
};
pub use item_subsystem::{GetFitAddSubsystemError, GetItemChangeSubsystemError, ItemChangeSubsystemError};
pub(in crate::cmd) use item_subsystem::{
    ICmdSubsystemAddFCtxBIds, ICmdSubsystemAddFCtxRIds, ICmdSubsystemAddICtx, ICmdSubsystemChangeFCtxBIds,
    ICmdSubsystemChangeFCtxRIds, ICmdSubsystemChangeICtx,
};
pub(in crate::cmd) use sol::{ICmdSolAddFCtx, ICmdSolChangeFCtx};

mod fit;
mod fleet;
mod item;
mod item_autocharge;
mod item_booster;
mod item_character;
mod item_charge;
mod item_drone;
mod item_fighter;
mod item_fw_effect;
mod item_implant;
mod item_module;
mod item_proj_effect;
mod item_rig;
mod item_service;
mod item_ship;
mod item_skill;
mod item_stance;
mod item_subsystem;
mod sol;
