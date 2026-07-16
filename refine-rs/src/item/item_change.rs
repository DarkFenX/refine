use crate::{
    cmd::{ChangeItemEnumCmd, ChangeItemEnumError, ChangedItemIdsResp},
    info::{ItemInfo, ItemInfoMode},
    item::Item,
};

impl Item<'_, '_> {
    #[tracing::instrument(name = "itm-chg", level = "trace", skip_all)]
    pub async fn change(&mut self, cmd: ChangeItemEnumCmd) -> Result<ChangedItemIdsResp, ChangeItemEnumError> {
        let item_id = self.id;
        self.sol
            .exec_standard_fallible(move |core_sol| {
                // Holding mutex on sol - nothing can remove the item before we get it here
                let mut core_item = core_sol.get_item_mut(&item_id).unwrap();
                cmd.execute(&mut core_item)
            })
            .await
    }
    #[tracing::instrument(name = "itm-chg", level = "trace", skip_all)]
    pub async fn change_and_get_info(
        &mut self,
        cmd: ChangeItemEnumCmd,
        item_mode: ItemInfoMode,
    ) -> Result<(ChangedItemIdsResp, ItemInfo), ChangeItemEnumError> {
        let item_id = self.id;
        self.sol
            .exec_standard_fallible(move |core_sol| {
                // Holding mutex on sol - nothing can remove the item before we get it here
                let mut core_item = core_sol.get_item_mut(&item_id).unwrap();
                let resp = cmd.execute(&mut core_item)?;
                let item_info = ItemInfo::from_core(&mut core_item, item_mode);
                Ok((resp, item_info))
            })
            .await
    }
}
