use crate::{Refine, SolarSystem, SolarSystemId};

impl Refine {
    #[tracing::instrument(name = "sol-get", level = "trace", skip_all)]
    pub async fn get_sol(&self, id: SolarSystemId) -> Result<SolarSystem<'_>, SolGetError> {
        let guarded_inner_sol = match self.id_sol_map.read().await.get(&id) {
            Some(sol) => sol.clone(),
            None => return Err(SolGetError::SolNotFound(id)),
        };
        let sol = SolarSystem::new(self, guarded_inner_sol).await;
        Ok(sol)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SolGetError {
    #[error("solar system {0} not found")]
    SolNotFound(SolarSystemId),
}
