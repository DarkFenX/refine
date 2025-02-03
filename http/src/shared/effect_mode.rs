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
impl From<&HEffectMode> for rc::SolEffectMode {
    fn from(h_effect_mode: &HEffectMode) -> Self {
        match h_effect_mode {
            HEffectMode::FullCompliance => Self::FullCompliance,
            HEffectMode::StateCompliance => Self::StateCompliance,
            HEffectMode::ForceRun => Self::ForceRun,
            HEffectMode::ForceStop => Self::ForceStop,
        }
    }
}
