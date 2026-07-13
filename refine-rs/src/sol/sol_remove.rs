use crate::{
    refine::Refine,
    sol::{SolarSystem, SolarSystemId},
};

impl Refine {
    #[tracing::instrument(name = "sol-rmv", level = "trace", skip_all)]
    async fn remove_sol(&self, id: SolarSystemId) -> Result<(), RemoveSolError> {
        match self.id_sol_map.write().await.remove(&id) {
            Some(_) => Ok(()),
            None => Err(RemoveSolError::SolNotFound(id)),
        }
    }
}

impl SolarSystem<'_> {
    pub async fn remove(self) -> Result<(), RemoveSolError> {
        self.refine.remove_sol(self.id).await
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RemoveSolError {
    #[error("solar system {0} not found")]
    SolNotFound(SolarSystemId),
}
