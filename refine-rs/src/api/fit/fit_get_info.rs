use crate::{
    Fit, FitInfo, FitInfoArgs,
    info::{FitInfoModesInt, ItemInfoModesInt},
};

impl Fit<'_, '_> {
    pub async fn get_info(&mut self, info_args: FitInfoArgs) -> FitInfo {
        // Variables for move
        let fit_id = self.id;
        self.sol
            .exec_standard_safe(move |core_sol| {
                // Holding mutex on sol - nothing can remove the core fit without consuming the
                // high-level Fit
                let mut core_fit = core_sol.get_fit_mut(&fit_id).unwrap();
                let fit_info_modes = FitInfoModesInt::from_pub_mode(info_args.fit);
                let item_info_modes = ItemInfoModesInt::from_pub_modes_regular(info_args.item);
                FitInfo::from_core(&mut core_fit, &fit_info_modes, &item_info_modes)
            })
            .await
    }
}
