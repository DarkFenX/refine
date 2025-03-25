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
impl From<&HEffectMode> for rc::EffectMode {
    fn from(h_effect_mode: &HEffectMode) -> Self {
        match h_effect_mode {
            HEffectMode::FullCompliance => Self::FullCompliance,
            HEffectMode::StateCompliance => Self::StateCompliance,
            HEffectMode::ForceRun => Self::ForceRun,
            HEffectMode::ForceStop => Self::ForceStop,
        }
    }
}
