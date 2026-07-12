use crate::{
    refine::Refine,
    sol::{SolarSystem, SolarSystemId},
};

impl Refine {
    pub async fn get_sol(&mut self, id: SolarSystemId) -> Result<SolarSystem<'_>, GetSolError> {
        let guarded_inner_sol = match self.id_sol_map.read().await.get(&id) {
            Some(sol) => sol.clone(),
            None => return Err(GetSolError::SolNotFound(id)),
        };
        Ok(SolarSystem::new(self, id, guarded_inner_sol))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetSolError {
    #[error("solar system {0} not found")]
    SolNotFound(SolarSystemId),
}
