use crate::{Refine, SolarSystem, SolarSystemId};

impl Refine {
    #[tracing::instrument(name = "sol-rmv", level = "trace", skip_all)]
    async fn remove_sol(&self, id: SolarSystemId) -> Result<(), SolRemoveError> {
        match self.id_sol_map.write().await.remove(&id) {
            Some(..) => Ok(()),
            None => Err(SolRemoveError::SolNotFound(id)),
        }
    }
}

impl SolarSystem<'_> {
    pub async fn remove(self) -> Result<(), SolRemoveError> {
        self.refine.remove_sol(self.get_id()).await
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SolRemoveError {
    #[error("solar system {0} not found")]
    SolNotFound(SolarSystemId),
}
