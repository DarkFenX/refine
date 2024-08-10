use crate::sol::{err::basic::FitAllocError, fit_info::SolFitInfo, SolarSystem};

impl SolarSystem {
    pub fn add_fit(&mut self) -> Result<SolFitInfo, AddFitError> {
        let fit_id = self.fits.add_fit()?;
        self.svcs.add_fit(&fit_id);
        let fit = self.fits.get_fit(&fit_id).unwrap();
        Ok(SolFitInfo::from(fit))
    }
}

#[derive(Debug)]
pub enum AddFitError {
    FitIdAllocFailed(FitAllocError),
}
impl From<FitAllocError> for AddFitError {
    fn from(error: FitAllocError) -> Self {
        Self::FitIdAllocFailed(error)
    }
}
impl std::error::Error for AddFitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitIdAllocFailed(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddFitError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitIdAllocFailed(e) => e.fmt(f),
        }
    }
}
