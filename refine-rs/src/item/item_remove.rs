use crate::{
    cmd::{BasicRemoveItemError, RemoveItemCmd},
    item::Item,
};

impl Item<'_, '_> {
    #[tracing::instrument(name = "itm-rmv", level = "trace", skip_all)]
    pub async fn remove(self, cmd: RemoveItemCmd) -> Result<(), RemoveItemError> {
        let item_id = self.id;
        self.sol
            .exec_standard_safe(move |core_sol| cmd.execute(core_sol, &item_id))
            .await?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
#[error("{0}")]
pub struct RemoveItemError(#[source] pub rc::err::RemoveItemError);
impl From<BasicRemoveItemError> for RemoveItemError {
    fn from(error: BasicRemoveItemError) -> Self {
        match error {
            // Holding mutex on sol - nothing can remove the core item without consuming the
            // high-level Item
            BasicRemoveItemError::ItemGetFailed(_) => unreachable!(),
            BasicRemoveItemError::ItemRemoveFailed(core_error) => Self(core_error),
        }
    }
}
