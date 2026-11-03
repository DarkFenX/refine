use crate::{Refine, SolAddCmd, SolInfo, SolInfoCmd, SolarSystem, SrcAlias, src::err::SrcGetError};

impl Refine {
    #[tracing::instrument(name = "sol-add", level = "error", skip_all)]
    pub async fn add_sol(
        &self,
        src_alias: Option<SrcAlias>,
        ctl_cmd: SolAddCmd,
    ) -> Result<SolarSystem<'_>, SolAddError> {
        let inner_src = self.internal_get_src(src_alias)?;
        // Variables for move
        let core_src = inner_src.get_core().clone();
        let core_sol = self.tpool.exec_standard(move || ctl_cmd.execute(&core_src)).await;
        let src_alias = inner_src.get_alias();
        let inner_sol = self.id_sol_map.add(src_alias, core_sol);
        let sol = SolarSystem::new(self, inner_sol).await;
        Ok(sol)
    }
    #[tracing::instrument(name = "sol-add-inf", level = "error", skip_all)]
    pub async fn add_sol_and_get_info(
        &self,
        src_alias: Option<SrcAlias>,
        ctl_cmd: SolAddCmd,
        info_cmd: SolInfoCmd,
    ) -> Result<(SolarSystem<'_>, SolInfo), SolAddError> {
        let inner_src = self.internal_get_src(src_alias)?;
        // Variables for move
        let core_src = inner_src.get_core().clone();
        let (core_sol, info_ext) = self
            .tpool
            .exec_standard(move || {
                let mut core_sol = ctl_cmd.execute(&core_src);
                let info_ext = info_cmd.execute_into_info_ext(&mut core_sol);
                (core_sol, info_ext)
            })
            .await;
        let src_alias = inner_src.get_alias();
        let inner_sol = self.id_sol_map.add(src_alias, core_sol);
        let sol = SolarSystem::new(self, inner_sol).await;
        let info = SolInfo::from_ids_and_ext(sol.get_id(), sol.get_src_alias(), info_ext);
        Ok((sol, info))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SolAddError {
    #[error("failed to get source")]
    SrcGet(#[from] SrcGetError),
}
