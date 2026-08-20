use crate::{
    Item,
    stats::{ItemStatsCmd, ItemStatsResp},
};

impl Item<'_, '_> {
    pub async fn get_stats(&mut self, stats_cmd: ItemStatsCmd) -> ItemStatsResp {
        // Variables for move
        let item_id = self.id;
        self.sol
            .exec_standard_safe(move |core_sol| {
                // Holding mutex on sol - nothing can remove the item before we get it here
                let mut core_item = core_sol.get_item_mut(&item_id).unwrap();
                stats_cmd.execute(&mut core_item)
            })
            .await
    }
}
