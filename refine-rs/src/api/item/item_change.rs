use crate::{
    ChangedItemIdsResp, CmdResp, CmdResps, Item, ItemChangeEnumCmd, ItemInfo, ItemInfoCmdBr, err::ItemChangeEnumError,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Item<'_, '_> {
    #[tracing::instrument(name = "itm-chg", level = "error", skip_all)]
    pub async fn change(&mut self, ctl_cmd: ItemChangeEnumCmd) -> Result<ChangedItemIdsResp, ItemChangeEnumError> {
        // Variables for move
        let item_id = self.id;
        self.sol
            .exec_standard(move |core_sol| {
                // Holding mutex on sol - nothing can remove the item before we get it here
                let mut core_item = core_sol.get_item(&item_id).unwrap();
                ctl_cmd.execute(&mut core_item)
            })
            .await
    }
    #[tracing::instrument(name = "itm-chg-inf", level = "error", skip_all)]
    pub async fn change_and_get_info(
        &mut self,
        ctl_cmd: ItemChangeEnumCmd,
        info_cmd: ItemInfoCmdBr,
    ) -> Result<(ChangedItemIdsResp, ItemInfo), ItemChangeEnumError> {
        // Variables for move
        let item_id = self.id;
        self.sol
            .exec_standard(move |core_sol| {
                // Holding mutex on sol - nothing can remove the item before we get it here
                let mut core_item = core_sol.get_item(&item_id).unwrap();
                let ctl_cmd_resp = ctl_cmd.execute(&mut core_item)?;
                let ctl_cmd_resps = CmdResps::with_resp(ctl_cmd_resp.into());
                let info_cmd = info_cmd.br_resolve(&ctl_cmd_resps);
                let item_info = info_cmd.execute(&mut core_item);
                let Some(CmdResp::ChangedItemIds(ctl_cmd_resp)) = ctl_cmd_resps.into_iter().next() else {
                    unreachable!("expected the same command which was sent in");
                };
                Ok((ctl_cmd_resp, item_info))
            })
            .await
    }
}
