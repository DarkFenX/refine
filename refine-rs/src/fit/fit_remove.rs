use crate::{cmd::RemoveFitCmd, fit::Fit};

impl Fit<'_, '_> {
    #[tracing::instrument(name = "fit-rmv", level = "trace", skip_all)]
    pub async fn remove(self, cmd: RemoveFitCmd) {
        let fit_id = self.id;
        self.sol
            .exec_standard_safe(move |core_sol| {
                // Holding mutex on sol - nothing can remove the fit before we get it here
                let core_fit = core_sol.get_fit_mut(&fit_id).unwrap();
                cmd.execute(core_fit)
            })
            .await;
    }
}
