use crate::{
    Fit,
    val::{FitValInfo, ValInfoMode, ValidateFitCmd},
};

impl Fit<'_, '_> {
    #[tracing::instrument(name = "fit-val", level = "trace", skip_all)]
    pub async fn validate(&mut self, cmd: ValidateFitCmd, val_mode: ValInfoMode) -> FitValInfo {
        let fit_id = self.id;
        self.sol
            .exec_standard_safe(move |core_sol| {
                // Holding mutex on sol - nothing can remove the fit before we get it here
                let mut core_fit = core_sol.get_fit_mut(&fit_id).unwrap();
                cmd.execute(&mut core_fit, val_mode)
            })
            .await
    }
}
