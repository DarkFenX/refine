use crate::{Fit, ItemTypeId, trial::FitTryItemsCmd};

impl Fit<'_, '_> {
    #[tracing::instrument(name = "fit-try", level = "trace", skip_all)]
    pub async fn try_items(&mut self, try_cmd: FitTryItemsCmd) -> Vec<ItemTypeId> {
        // Variables for move
        let fit_id = self.id;
        self.sol
            .exec_standard_infallible(move |core_sol| {
                // Holding mutex on sol - nothing can remove the fit before we get it here
                let mut core_fit = core_sol.get_fit_mut(&fit_id).unwrap();
                try_cmd.execute(&mut core_fit)
            })
            .await
    }
}
