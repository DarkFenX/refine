use crate::{Item, ItemInfo, ItemInfoArgs, info::ItemInfoModesInt};

impl Item<'_, '_> {
    pub async fn get_info(&mut self, info_args: ItemInfoArgs) -> ItemInfo {
        // Variables for move
        let item_id = self.id;
        self.sol
            .exec_standard_safe(move |core_sol| {
                // Holding mutex on sol - nothing can remove the item before we get it here
                let mut core_item = core_sol.get_item_mut(&item_id).unwrap();
                let item_info_modes = ItemInfoModesInt::from_pub_modes_regular(info_args.item);
                ItemInfo::from_core(&mut core_item, &item_info_modes)
            })
            .await
    }
}
