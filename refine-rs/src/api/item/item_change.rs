use crate::{ChangeItemEnumCmd, ChangedItemIdsResp, Item, ItemInfo, err::ChangeItemEnumError, info::ItemInfoCmd};

impl Item<'_, '_> {
    #[tracing::instrument(name = "itm-chg", level = "trace", skip_all)]
    pub async fn change(&mut self, ctl_cmd: ChangeItemEnumCmd) -> Result<ChangedItemIdsResp, ChangeItemEnumError> {
        // Variables for move
        let item_id = self.id;
        self.sol
            .exec_standard_fallible(move |core_sol| {
                // Holding mutex on sol - nothing can remove the item before we get it here
                let mut core_item = core_sol.get_item_mut(&item_id).unwrap();
                ctl_cmd.execute(&mut core_item)
            })
            .await
    }
    #[tracing::instrument(name = "itm-chg-inf", level = "trace", skip_all)]
    pub async fn change_and_get_info(
        &mut self,
        ctl_cmd: ChangeItemEnumCmd,
        info_cmd: ItemInfoCmd,
    ) -> Result<(ChangedItemIdsResp, ItemInfo), ChangeItemEnumError> {
        // Variables for move
        let item_id = self.id;
        self.sol
            .exec_standard_fallible(move |core_sol| {
                // Holding mutex on sol - nothing can remove the item before we get it here
                let mut core_item = core_sol.get_item_mut(&item_id).unwrap();
                let resp = ctl_cmd.execute(&mut core_item)?;
                let item_info = info_cmd.execute(&mut core_item);
                Ok((resp, item_info))
            })
            .await
    }
}
