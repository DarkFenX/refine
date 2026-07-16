use crate::{Item, ItemInfo, ItemInfoMode};

impl Item<'_, '_> {
    pub async fn get_info(&mut self, item_mode: ItemInfoMode) -> ItemInfo {
        let item_id = self.id;
        self.sol
            .exec_standard_safe(move |core_sol| {
                // Holding mutex on sol - nothing can remove the item before we get it here
                let mut core_item = core_sol.get_item_mut(&item_id).unwrap();
                ItemInfo::from_core(&mut core_item, item_mode)
            })
            .await
    }
}
