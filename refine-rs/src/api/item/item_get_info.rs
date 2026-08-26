use crate::{Item, ItemInfo, ItemInfoCmd};

impl Item<'_, '_> {
    #[tracing::instrument(name = "itm-inf", level = "trace", skip_all)]
    pub async fn get_info(&mut self, info_cmd: ItemInfoCmd) -> ItemInfo {
        // Variables for move
        let item_id = self.id;
        self.sol
            .exec_standard_infallible(move |core_sol| {
                // Holding mutex on sol - nothing can remove the item before we get it here
                let mut core_item = core_sol.get_item_mut(&item_id).unwrap();
                info_cmd.execute(&mut core_item)
            })
            .await
    }
}
