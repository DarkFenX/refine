use crate::{FitId, FitIdBackref, InfoModes, info::InfoModesInt};

#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone)]
pub enum FitInfoMode {
    Id,
    Full,
}
const impl Default for FitInfoMode {
    fn default() -> Self {
        Self::Full
    }
}

pub type FitInfoModes = InfoModes<FitInfoMode, FitId>;
pub type FitInfoModesBackref = InfoModes<FitInfoMode, FitIdBackref>;
pub(crate) type FitInfoModesInt = InfoModesInt<FitInfoMode, FitId>;
