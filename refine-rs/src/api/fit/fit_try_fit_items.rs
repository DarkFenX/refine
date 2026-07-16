use crate::{Fit, ItemTypeId, val::TryFitItemsCmd};

impl Fit<'_, '_> {
    #[tracing::instrument(name = "fit-try", level = "trace", skip_all)]
    pub async fn try_fit_items(&mut self, cmd: TryFitItemsCmd) -> Vec<ItemTypeId> {
        let fit_id = self.id;
        self.sol
            .exec_standard_safe(move |core_sol| {
                // Holding mutex on sol - nothing can remove the fit before we get it here
                let mut core_fit = core_sol.get_fit_mut(&fit_id).unwrap();
                cmd.execute(&mut core_fit)
            })
            .await
    }
}
