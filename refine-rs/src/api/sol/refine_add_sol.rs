use std::collections::hash_map::Entry;

use crate::{
    AddSolCmd, Refine, SolInfo, SolInfoCmd, SolarSystem, SolarSystemId,
    src::{SrcAlias, err::GetSrcError},
    svc::SolarSystemInnerGuarded,
};

impl Refine {
    #[tracing::instrument(name = "sol-add", level = "trace", skip_all)]
    pub async fn add_sol(
        &self,
        src_alias: Option<SrcAlias>,
        ctl_cmd: AddSolCmd,
    ) -> Result<SolarSystem<'_>, AddSolError> {
        let inner_src = self.internal_get_src(src_alias).await?;
        // Variables for move
        let core_src = inner_src.get_core().clone();
        let core_sol = self.tpool.exec_standard(move || ctl_cmd.execute(&core_src)).await;
        let src_alias = inner_src.get_alias();
        let inner_sol = self.create_and_store_inner_sol(src_alias, core_sol).await;
        let sol = SolarSystem::new(self, inner_sol).await;
        Ok(sol)
    }
    #[tracing::instrument(name = "sol-add-inf", level = "trace", skip_all)]
    pub async fn add_sol_and_get_info(
        &self,
        src_alias: Option<SrcAlias>,
        ctl_cmd: AddSolCmd,
        info_cmd: SolInfoCmd,
    ) -> Result<(SolarSystem<'_>, SolInfo), AddSolError> {
        let inner_src = self.internal_get_src(src_alias).await?;
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
        let inner_sol = self.create_and_store_inner_sol(src_alias, core_sol).await;
        let sol = SolarSystem::new(self, inner_sol).await;
        let info = SolInfo::from_ids_and_ext(sol.get_id(), sol.get_src_alias(), info_ext);
        Ok((sol, info))
    }
    async fn create_and_store_inner_sol(
        &self,
        src_alias: SrcAlias,
        core_sol: rc::SolarSystem,
    ) -> SolarSystemInnerGuarded {
        let mut id = SolarSystemId::new();
        let mut id_sol_map = self.id_sol_map.write().await;
        loop {
            match id_sol_map.entry(id) {
                Entry::Vacant(entry) => {
                    let inner_sol = SolarSystemInnerGuarded::new(id, src_alias, core_sol);
                    entry.insert(inner_sol.clone());
                    return inner_sol;
                }
                Entry::Occupied(..) => {
                    id = SolarSystemId::new();
                    continue;
                }
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddSolError {
    #[error("failed to get source")]
    SrcGet(#[from] GetSrcError),
}
