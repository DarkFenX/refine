use crate::{Refine, Sol, SolId, svc::SolInnerGuarded};

impl Refine {
    #[tracing::instrument(name = "sol-get", level = "error", skip_all)]
    pub async fn get_sol(&self, id: SolId) -> Result<Sol<'_>, SolGetError> {
        let guarded_inner_sol = match self.id_sol_map.get(&id) {
            Some(sol) => sol,
            None => return Err(SolGetError::SolNotFound(id)),
        };
        match self.internal_lock_sol(guarded_inner_sol, true).await {
            Some(sol) => Ok(sol),
            None => Err(SolGetError::SolNotFound(id)),
        }
    }
    pub(crate) async fn internal_lock_sol(&self, guarded_inner_sol: SolInnerGuarded, touch: bool) -> Option<Sol<'_>> {
        let mut sol = Sol::new(self, guarded_inner_sol, touch).await;
        // Solar system could be removed while the request was waiting for its lock
        if sol.inner.is_closed() {
            return None;
        }
        if sol.inner.is_poisoned() {
            tracing::error!("discarding solar system poisoned by a panic");
            self.id_sol_map.remove(&mut sol.inner);
            return None;
        }
        Some(sol)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SolGetError {
    #[error("solar system {0} not found")]
    SolNotFound(SolId),
}
