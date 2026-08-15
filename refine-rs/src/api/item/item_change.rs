use crate::{
    ChangeItemEnumCmd, ChangedItemIdsResp, Item, ItemInfo, ItemInfoArgs, err::ChangeItemEnumError,
    info::ItemInfoModesInt,
};

impl Item<'_, '_> {
    #[tracing::instrument(name = "itm-chg", level = "trace", skip_all)]
    pub async fn change(&mut self, cmd: ChangeItemEnumCmd) -> Result<ChangedItemIdsResp, ChangeItemEnumError> {
        // Variables for move
        let item_id = self.id;
        self.sol
            .exec_standard_fallible(move |core_sol| {
                // Holding mutex on sol - nothing can remove the item before we get it here
                let mut core_item = core_sol.get_item_mut(&item_id).unwrap();
                cmd.execute(&mut core_item)
            })
            .await
    }
    #[tracing::instrument(name = "itm-chg-inf", level = "trace", skip_all)]
    pub async fn change_and_get_info(
        &mut self,
        exec_cmd: ChangeItemEnumCmd,
        info_args: ItemInfoArgs,
    ) -> Result<(ChangedItemIdsResp, ItemInfo), ChangeItemEnumError> {
        // Variables for move
        let item_id = self.id;
        self.sol
            .exec_standard_fallible(move |core_sol| {
                // Holding mutex on sol - nothing can remove the item before we get it here
                let mut core_item = core_sol.get_item_mut(&item_id).unwrap();
                let resp = exec_cmd.execute(&mut core_item)?;
                let item_info_modes = ItemInfoModesInt::from_pub_modes_regular(info_args.item);
                let item_info = ItemInfo::from_core(&mut core_item, &item_info_modes);
                Ok((resp, item_info))
            })
            .await
    }
}
