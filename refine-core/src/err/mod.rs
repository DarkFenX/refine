pub use basic::ItemKindMatchError;

#[cfg(feature = "serde")]
pub use crate::ud::err::{FitIdParseError, FleetIdParseError, ItemIdParseError};
pub use crate::{
    api::{
        AbilityGetError, AttrMutateRawError, AutochargeGetError, BoosterGetError, CharacterGetError, ChargeGetError,
        DroneGetError, FighterGetError, FitFleetSetError, FitFleetUnsetError, FitGetError, FitItemAutodetectAddError,
        FitRahIncomingDpsRemoveError, FleetFitAddError, FleetFitRemoveError, FleetGetError, FwEffectGetError,
        ImplantGetError, ItemAttrGetError, ItemAttrsIterError, ItemEffectsIterError, ItemGetError,
        ItemModifiersIterError, ItemRemoveError, ModuleGetError, MutationAddError, ProjAddError, ProjEffectGetError,
        ProjGetError, RawMAttrGetError, RigGetError, ServiceGetError, ShipGetError, SideEffectGetError, SkillAddError,
        SkillGetError, SkillTypeIdSetError, StanceGetError, SubsystemGetError, SwEffectGetError,
    },
    num::{CountNzError, FitSecStatusError, PValueError, SkillLevelError, UnitIntervalError},
};

pub mod basic;
