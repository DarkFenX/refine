use crate::{Fit, ItemTypeId, val::TryFitItemsCmd};

impl Fit<'_, '_> {
    #[tracing::instrument(name = "fit-try", level = "trace", skip_all)]
    pub async fn try_fit_items(&mut self, cmd: TryFitItemsCmd) -> Vec<ItemTypeId> {
        let fit_id = self.id;
        // Try-fit-items method is modifying sol state even if it does not fail - due to how charge
        // checks are done. Always roll sol state back because of that
        self.sol
            .exec_standard_rollback(move |core_sol| {
                // Holding mutex on sol - nothing can remove the fit before we get it here
                let mut core_fit = core_sol.get_fit_mut(&fit_id).unwrap();
                cmd.execute(&mut core_fit)
            })
            .await
    }
}
