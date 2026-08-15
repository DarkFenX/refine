use crate::{Fit, FitInfo, FitInfoCmd};

impl Fit<'_, '_> {
    pub async fn get_info(&mut self, info_cmd: FitInfoCmd) -> FitInfo {
        // Variables for move
        let fit_id = self.id;
        self.sol
            .exec_standard_safe(move |core_sol| {
                // Holding mutex on sol - nothing can remove the core fit without consuming the
                // high-level Fit
                let mut core_fit = core_sol.get_fit_mut(&fit_id).unwrap();
                info_cmd.execute(&mut core_fit)
            })
            .await
    }
}
