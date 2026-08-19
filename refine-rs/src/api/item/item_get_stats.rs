use crate::{
    Item,
    stats::{StatItemOptions, StatItemResult},
};

impl Item<'_, '_> {
    pub async fn get_stats(&mut self, stat_opts: StatItemOptions) -> StatItemResult {
        // Variables for move
        let item_id = self.id;
        self.sol
            .exec_standard_safe(move |core_sol| {
                // Holding mutex on sol - nothing can remove the item before we get it here
                let mut core_item = core_sol.get_item_mut(&item_id).unwrap();
                stat_opts.execute(&mut core_item)
            })
            .await
    }
}
