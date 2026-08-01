pub use basic::ItemKindMatchError;

#[cfg(feature = "serde")]
pub use crate::ud::err::{ParseFitIdError, ParseFleetIdError, ParseItemIdError};
pub use crate::{
    api::{
        AddMutationError, AddProjError, AddSkillError, AttrMutateRawError, FleetAddFitError, FleetRemoveFitError,
        GetAbilityError, GetAutochargeError, GetBoosterError, GetCharacterError, GetChargeError, GetDroneError,
        GetFighterError, GetFitError, GetFleetError, GetFwEffectError, GetImplantError, GetItemAttrError, GetItemError,
        GetModuleError, GetProjEffectError, GetProjError, GetRawMAttrError, GetRigError, GetServiceError, GetShipError,
        GetSideEffectError, GetSkillError, GetStanceError, GetSubsystemError, GetSwEffectError, IterItemAttrsError,
        IterItemEffectsError, IterItemModifiersError, RemoveFitRahIncomingDpsError, RemoveItemError, SetFitFleetError,
        SetSkillTypeIdError, UnsetFitFleetError,
    },
    num::{CountNzError, FitSecStatusError, PValueError, SkillLevelError, UnitIntervalError},
};

pub mod basic;
