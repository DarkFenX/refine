use crate::info::valid::details::HResValFail;

#[derive(serde::Serialize)]
pub(crate) struct HValidInfoDetailed {
    passed: bool,
    #[serde(skip_serializing_if = "HValidInfoDetails::is_empty")]
    details: HValidInfoDetails,
}
impl From<&rc::SolValResult> for HValidInfoDetailed {
    fn from(core_val_result: &rc::SolValResult) -> Self {
        Self {
            passed: core_val_result.all_passed(),
            details: core_val_result.into(),
        }
    }
}

#[derive(serde::Serialize)]
struct HValidInfoDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu: Option<HResValFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pg: Option<HResValFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    calibration: Option<HResValFail>,
}
impl HValidInfoDetails {
    fn is_empty(&self) -> bool {
        self.cpu.is_none() && self.pg.is_none() && self.calibration.is_none()
    }
}
impl From<&rc::SolValResult> for HValidInfoDetails {
    fn from(core_val_result: &rc::SolValResult) -> Self {
        Self {
            cpu: core_val_result.cpu.as_ref().map(|v| v.into()),
            pg: core_val_result.pg.as_ref().map(|v| v.into()),
            calibration: core_val_result.calibration.as_ref().map(|v| v.into()),
        }
    }
}
