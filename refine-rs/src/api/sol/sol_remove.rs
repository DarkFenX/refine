use crate::{Sol, SolId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol<'_> {
    #[tracing::instrument(name = "sol-rmv", level = "error", skip_all)]
    pub async fn remove(mut self) -> Result<(), SolRemoveError> {
        match self.refine.id_sol_map.remove(self.inner_mut()) {
            true => {
                // The lock guard holds the solar system, which is freed with it unless a request
                // waiting for the lock still holds it
                self.refine.tpool.free_standard(self.take_inner());
                Ok(())
            }
            false => Err(SolRemoveError::SolNotFound(self.get_id())),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SolRemoveError {
    #[error("solar system {0} not found")]
    SolNotFound(SolId),
}
