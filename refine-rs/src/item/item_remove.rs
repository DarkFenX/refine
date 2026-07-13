use crate::{
    cmd::{RemoveItemCmd, RemoveItemError},
    item::Item,
};

impl Item<'_, '_> {
    #[tracing::instrument(name = "itm-rmv", level = "trace", skip_all)]
    pub async fn remove(self, cmd: RemoveItemCmd) -> Result<(), RemoveItemError> {
        let item_id = self.id;
        self.sol
            .exec_standard_safe(move |core_sol| {
                // Holding mutex on sol - nothing can remove the core item without consuming the
                // high-level Item
                let core_item = core_sol.get_item_mut(&item_id).unwrap();
                cmd.execute(core_item)
            })
            .await?;
        Ok(())
    }
}
