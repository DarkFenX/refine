#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HEffect {
    pub(crate) running: bool,
    pub(crate) mode: HEffectMode,
}
impl HEffect {
    fn new(running: bool, mode: HEffectMode) -> Self {
        Self { running, mode }
    }
}
impl From<&rc::EffectInfo> for HEffect {
    fn from(core_effect_info: &rc::EffectInfo) -> Self {
        Self::new(core_effect_info.running, (&core_effect_info.mode).into())
    }
}

#[derive(serde::Serialize)]
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
