use crate::{err::basic::FitFoundError, sol::SolarSystem};

impl SolarSystem {
    pub fn validate_fit_verbose(&self) -> Result<bool, ValidateFitError> {
        Ok(true)
    }
    pub fn validate_fit_fast(&self) -> Result<bool, ValidateFitError> {
        Ok(true)
    }
}

#[derive(Debug)]
pub enum ValidateFitError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for ValidateFitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for ValidateFitError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for ValidateFitError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
