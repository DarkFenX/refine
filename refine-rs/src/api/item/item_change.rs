use crate::{ChangedItemIdsResp, CmdResps, Item, ItemChangeEnumCmd, ItemInfo, ItemInfoCmdBr, err::ItemChangeEnumError};

impl Item<'_, '_> {
    #[tracing::instrument(name = "itm-chg", level = "trace", skip_all)]
    pub async fn change(&mut self, ctl_cmd: ItemChangeEnumCmd) -> Result<ChangedItemIdsResp, ItemChangeEnumError> {
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
        ctl_cmd: ItemChangeEnumCmd,
        info_cmd: ItemInfoCmdBr,
    ) -> Result<(ChangedItemIdsResp, ItemInfo), ItemChangeEnumError> {
        // Variables for move
        let item_id = self.id;
        self.sol
            .exec_standard_fallible(move |core_sol| {
                // Holding mutex on sol - nothing can remove the item before we get it here
                let mut core_item = core_sol.get_item_mut(&item_id).unwrap();
                let ctl_cmd_resp = ctl_cmd.execute(&mut core_item)?;
                let ctl_cmd_resps = CmdResps::with_resp(ctl_cmd_resp.into());
                let info_cmd = info_cmd.br_resolve(&ctl_cmd_resps);
                let item_info = info_cmd.execute(&mut core_item);
                Ok((ctl_cmd_resp, item_info))
            })
            .await
    }
}
