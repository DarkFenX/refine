use crate::{Fit, FitInfo, FitInfoArgs};

impl Fit<'_, '_> {
    pub async fn get_info(&mut self, info_args: FitInfoArgs) -> FitInfo {
        // Variables for move
        let fit_id = self.id;
        self.sol
            .exec_standard_safe(move |core_sol| {
                // Holding mutex on sol - nothing can remove the core fit without consuming the
                // high-level Fit
                let mut core_fit = core_sol.get_fit_mut(&fit_id).unwrap();
                FitInfo::from_core(&mut core_fit, info_args)
            })
            .await
    }
}
