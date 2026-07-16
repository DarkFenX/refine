use crate::{
    Item,
    stats::{GetItemStatsCmd, ItemStats},
};

impl Item<'_, '_> {
    pub async fn get_stats(&mut self, cmd: GetItemStatsCmd) -> ItemStats {
        let item_id = self.id;
        self.sol
            .exec_standard_safe(move |core_sol| {
                // Holding mutex on sol - nothing can remove the item before we get it here
                let mut core_item = core_sol.get_item_mut(&item_id).unwrap();
                cmd.execute(&mut core_item)
            })
            .await
    }
}
