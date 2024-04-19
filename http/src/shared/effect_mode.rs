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
impl From<&rc::SsEffectMode> for HEffectMode {
    fn from(core_effect_mode: &rc::SsEffectMode) -> Self {
        match core_effect_mode {
            rc::SsEffectMode::FullCompliance => Self::FullCompliance,
            rc::SsEffectMode::StateCompliance => Self::StateCompliance,
            rc::SsEffectMode::ForceRun => Self::ForceRun,
            rc::SsEffectMode::ForceStop => Self::ForceStop,
        }
    }
}
impl Into<rc::SsEffectMode> for &HEffectMode {
    fn into(self) -> rc::SsEffectMode {
        match self {
            HEffectMode::FullCompliance => rc::SsEffectMode::FullCompliance,
            HEffectMode::StateCompliance => rc::SsEffectMode::StateCompliance,
            HEffectMode::ForceRun => rc::SsEffectMode::ForceRun,
            HEffectMode::ForceStop => rc::SsEffectMode::ForceStop,
        }
    }
}
