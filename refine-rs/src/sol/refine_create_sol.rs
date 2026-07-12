use crate::{
    cmd::CreateSolCmd,
    refine::Refine,
    sol::{SolarSystem, SolarSystemId, SolarSystemInnerGuarded},
    src::{GetSrcError, SrcAlias},
};

impl Refine {
    #[tracing::instrument(name = "sol-add", level = "trace", skip_all)]
    pub async fn create_sol(
        &self,
        src_alias: Option<&SrcAlias>,
        cmd: CreateSolCmd,
    ) -> Result<SolarSystem<'_>, CreateSolError> {
        let core_src = self.internal_get_src(src_alias).await?.get_core().clone();
        let inner_sol = self
            .tpool
            .exec_standard(move || {
                let core_sol = cmd.execute(&core_src);
                SolarSystemInnerGuarded::new(core_sol)
            })
            .await;
        let mut id = SolarSystemId::new();
        let mut map_lock = self.id_sol_map.write().await;
        while map_lock.contains_key(&id) {
            id = SolarSystemId::new();
        }
        map_lock.insert(id, inner_sol.clone());
        drop(map_lock);
        Ok(SolarSystem::new(self, id, inner_sol).await)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CreateSolError {
    #[error("failed to get source: {0}")]
    GetSrcFailed(#[from] GetSrcError),
}
