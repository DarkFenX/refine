use crate::{Item, ItemInfo, ItemInfoModes};

impl Item<'_, '_> {
    pub async fn get_info(&mut self, modes: ItemInfoModes) -> ItemInfo {
        // Variables for move
        let item_id = self.id;
        self.sol
            .exec_standard_safe(move |core_sol| {
                // Holding mutex on sol - nothing can remove the item before we get it here
                let mut core_item = core_sol.get_item_mut(&item_id).unwrap();
                ItemInfo::from_core(&mut core_item, modes)
            })
            .await
    }
}
