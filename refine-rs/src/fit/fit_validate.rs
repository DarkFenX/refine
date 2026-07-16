use crate::{
    Fit,
    val::{FitValInfo, ValInfoMode, ValOptions},
};

impl Fit<'_, '_> {
    #[tracing::instrument(name = "fit-val", level = "trace", skip_all)]
    pub async fn validate(self, options: ValOptions, val_mode: ValInfoMode) -> FitValInfo {
        let fit_id = self.id;
        self.sol
            .exec_standard_safe(move |core_sol| {
                // Holding mutex on sol - nothing can remove the fit before we get it here
                let mut core_fit = core_sol.get_fit_mut(&fit_id).unwrap();
                match val_mode {
                    ValInfoMode::Simple => FitValInfo {
                        passed: core_fit.validate_fast(&options),
                        details: None,
                    },
                    ValInfoMode::Detailed => {
                        let details = core_fit.validate_verbose(&options);
                        FitValInfo {
                            passed: details.all_passed(),
                            details: Some(details),
                        }
                    }
                }
            })
            .await
    }
}
