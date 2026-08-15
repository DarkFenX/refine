use crate::{
    Fit,
    val::{FitValInfo, ValFitInfoArgs, ValidateFitCmd},
};

impl Fit<'_, '_> {
    #[tracing::instrument(name = "fit-val", level = "trace", skip_all)]
    pub async fn validate(&mut self, val_cmd: ValidateFitCmd, info_args: ValFitInfoArgs) -> FitValInfo {
        // Variables for move
        let fit_id = self.id;
        self.sol
            .exec_standard_safe(move |core_sol| {
                // Holding mutex on sol - nothing can remove the fit before we get it here
                let mut core_fit = core_sol.get_fit_mut(&fit_id).unwrap();
                val_cmd.execute(&mut core_fit, info_args.validation)
            })
            .await
    }
}
