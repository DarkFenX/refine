#[derive(serde::Serialize, serde::Deserialize)]
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
impl From<&rc::SolEffectMode> for HEffectMode {
    fn from(core_effect_mode: &rc::SolEffectMode) -> Self {
        match core_effect_mode {
            rc::SolEffectMode::FullCompliance => Self::FullCompliance,
            rc::SolEffectMode::StateCompliance => Self::StateCompliance,
            rc::SolEffectMode::ForceRun => Self::ForceRun,
            rc::SolEffectMode::ForceStop => Self::ForceStop,
        }
    }
}
impl Into<rc::SolEffectMode> for &HEffectMode {
    fn into(self) -> rc::SolEffectMode {
        match self {
            HEffectMode::FullCompliance => rc::SolEffectMode::FullCompliance,
            HEffectMode::StateCompliance => rc::SolEffectMode::StateCompliance,
            HEffectMode::ForceRun => rc::SolEffectMode::ForceRun,
            HEffectMode::ForceStop => rc::SolEffectMode::ForceStop,
        }
    }
}
