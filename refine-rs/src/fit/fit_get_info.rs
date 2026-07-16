use crate::{
    fit::Fit,
    info::{FitInfo, FitInfoMode, ItemInfoMode},
};

impl Fit<'_, '_> {
    pub async fn get_info(&mut self, fit_mode: FitInfoMode, item_mode: ItemInfoMode) -> FitInfo {
        let fit_id = self.id;
        self.sol
            .exec_standard_safe(move |core_sol| {
                // Holding mutex on sol - nothing can remove the core fit without consuming the
                // high-level Fit
                let mut core_fit = core_sol.get_fit_mut(&fit_id).unwrap();
                FitInfo::from_core(&mut core_fit, fit_mode, item_mode)
            })
            .await
    }
}
