use crate::{
    def::FitKey,
    svc::{
        Svc, SvcCtx,
        vast::{ValOptionsInt, ValOptionsSolInt, ValResultFit, ValResultSol},
    },
    uad::Uad,
};

impl Svc {
    pub(crate) fn validate_sol_fast(&mut self, uad: &Uad, options: &ValOptionsSolInt) -> bool {
        self.vast
            .validate_sol_fast(SvcCtx::new(uad, &self.eprojs), &mut self.calc, options)
    }
    pub(crate) fn validate_sol_verbose(&mut self, uad: &Uad, options: &ValOptionsSolInt) -> ValResultSol {
        self.vast
            .validate_sol_verbose(SvcCtx::new(uad, &self.eprojs), &mut self.calc, options)
    }
    pub(crate) fn validate_fit_fast(&mut self, uad: &Uad, fit_key: FitKey, options: &ValOptionsInt) -> bool {
        self.vast
            .validate_fit_fast(SvcCtx::new(uad, &self.eprojs), &mut self.calc, fit_key, options)
    }
    pub(crate) fn validate_fit_verbose(&mut self, uad: &Uad, fit_key: FitKey, options: &ValOptionsInt) -> ValResultFit {
        self.vast
            .validate_fit_verbose(SvcCtx::new(uad, &self.eprojs), &mut self.calc, fit_key, options)
    }
}
