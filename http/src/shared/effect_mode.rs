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
impl From<&rc::EffectMode> for HEffectMode {
    fn from(core_effect_mode: &rc::EffectMode) -> Self {
        match core_effect_mode {
            rc::EffectMode::FullCompliance => Self::FullCompliance,
            rc::EffectMode::StateCompliance => Self::StateCompliance,
            rc::EffectMode::ForceRun => Self::ForceRun,
            rc::EffectMode::ForceStop => Self::ForceStop,
        }
    }
}
impl Into<rc::EffectMode> for &HEffectMode {
    fn into(self) -> rc::EffectMode {
        match self {
            HEffectMode::FullCompliance => rc::EffectMode::FullCompliance,
            HEffectMode::StateCompliance => rc::EffectMode::StateCompliance,
            HEffectMode::ForceRun => rc::EffectMode::ForceRun,
            HEffectMode::ForceStop => rc::EffectMode::ForceStop,
        }
    }
}
