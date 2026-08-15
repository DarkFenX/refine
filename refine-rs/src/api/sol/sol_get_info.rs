use crate::{SolInfo, SolInfoArgs, SolarSystem};

impl SolarSystem<'_> {
    pub async fn get_info(&mut self, info_args: SolInfoArgs) -> SolInfo {
        // Variables for move
        let sol_id = self.get_id();
        let src_alias = self.get_src_alias();
        self.exec_standard_safe(move |core_sol| SolInfo::from_ids_and_core(sol_id, src_alias, core_sol, info_args))
            .await
    }
}
