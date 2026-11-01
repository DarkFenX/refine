use crate::{Refine, SolarSystem, SolarSystemId};

impl Refine {
    #[tracing::instrument(name = "sol-get", level = "error", skip_all)]
    pub async fn get_sol(&self, id: SolarSystemId) -> Result<SolarSystem<'_>, SolGetError> {
        let guarded_inner_sol = match self.id_sol_map.get(&id) {
            Some(sol) => sol,
            None => return Err(SolGetError::SolNotFound(id)),
        };
        let mut sol = SolarSystem::new(self, guarded_inner_sol).await;
        // Solar system could be removed while the request was waiting for its lock
        if sol.inner.is_closed() {
            return Err(SolGetError::SolNotFound(id));
        }
        if sol.inner.is_poisoned() {
            tracing::error!("discarding solar system poisoned by a panic");
            self.id_sol_map.remove(&mut sol.inner);
            return Err(SolGetError::SolNotFound(id));
        }
        Ok(sol)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SolGetError {
    #[error("solar system {0} not found")]
    SolNotFound(SolarSystemId),
}
