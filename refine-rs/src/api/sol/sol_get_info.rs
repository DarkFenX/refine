use crate::{
    SolInfo, SolInfoArgs, SolarSystem,
    info::{FitInfoModesInt, FleetInfoModesInt, ItemInfoModesInt},
};

impl SolarSystem<'_> {
    pub async fn get_info(&mut self, info_args: SolInfoArgs) -> SolInfo {
        // Variables for move
        let sol_id = self.get_id();
        let src_alias = self.get_src_alias();
        self.exec_standard_safe(move |core_sol| {
            let sol_info_mode = info_args.sol;
            let fleet_info_modes = FleetInfoModesInt::from_pub_modes_regular(info_args.fleet);
            let fit_info_modes = FitInfoModesInt::from_pub_modes_regular(info_args.fit);
            let item_info_modes = ItemInfoModesInt::from_pub_modes_regular(info_args.item);
            SolInfo::from_ids_and_core(
                sol_id,
                src_alias,
                core_sol,
                sol_info_mode,
                &fleet_info_modes,
                &fit_info_modes,
                &item_info_modes,
            )
        })
        .await
    }
}
