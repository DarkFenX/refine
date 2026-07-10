use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub(crate) enum HEffectMode {
    #[serde(rename = "full")]
    FullCompliance,
    #[serde(rename = "state")]
    StateCompliance,
    #[serde(rename = "run")]
    ForceRun,
    #[serde(rename = "stop")]
    ForceStop,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HEffectMode {
    pub(crate) fn from_core(core_effect_mode: rc::EffectMode) -> Self {
        match core_effect_mode {
            rc::EffectMode::FullCompliance => Self::FullCompliance,
            rc::EffectMode::StateCompliance => Self::StateCompliance,
            rc::EffectMode::ForceRun => Self::ForceRun,
            rc::EffectMode::ForceStop => Self::ForceStop,
        }
    }
    pub(crate) fn into_core(self) -> rc::EffectMode {
        match self {
            Self::FullCompliance => rc::EffectMode::FullCompliance,
            Self::StateCompliance => rc::EffectMode::StateCompliance,
            Self::ForceRun => rc::EffectMode::ForceRun,
            Self::ForceStop => rc::EffectMode::ForceStop,
        }
    }
}
