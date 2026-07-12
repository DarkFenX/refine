use tokio_rayon::AsyncThreadPool;

use crate::{
    cmd::SolAddCmd,
    refine::Refine,
    sol::{SolarSystem, SolarSystemId, SolarSystemInnerGuarded},
    src::{GetSrcError, SrcAlias},
};

impl Refine {
    #[tracing::instrument(name = "sol-add", level = "trace", skip_all)]
    pub async fn create_sol(
        &mut self,
        src_alias: Option<&SrcAlias>,
        cmd: SolAddCmd,
    ) -> Result<SolarSystem<'_>, CreateSolError> {
        let core_src = self.internal_get_src(src_alias).await?.get_core().clone();
        let sync_span = tracing::trace_span!("sync");
        let inner_sol = self
            .tpool
            .standard
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
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
        Ok(SolarSystem::new(self, id, inner_sol))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CreateSolError {
    #[error("failed to get source: {0}")]
    GetSrcFailed(#[from] GetSrcError),
}
