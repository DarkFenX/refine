use crate::{SolInfo, SolInfoCmd, SolarSystem};

impl SolarSystem<'_> {
    pub async fn get_info(&mut self, info_cmd: SolInfoCmd) -> SolInfo {
        // Variables for move
        let sol_id = self.get_id();
        let src_alias = self.get_src_alias();
        self.exec_standard_safe(move |core_sol| info_cmd.execute_into_info(sol_id, src_alias, core_sol))
            .await
    }
}
