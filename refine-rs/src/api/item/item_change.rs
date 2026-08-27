use crate::{
    ChangedItemIdsResp, CmdResps, Item, ItemChangeEnumCmd, ItemInfo, ItemInfoCmdBr, err::ItemChangeEnumError,
    shared::ResidueResolver,
};

impl Item<'_, '_> {
    #[tracing::instrument(name = "itm-chg", level = "trace", skip_all)]
    pub async fn change(&mut self, ctl_cmd: ItemChangeEnumCmd) -> Result<ChangedItemIdsResp, ItemChangeEnumError> {
        let sol_backup = ResidueResolver::new().add_cmd(ctl_cmd.exec_residue());
        // Variables for move
        let item_id = self.id;
        self.sol
            .exec_standard(sol_backup, move |core_sol| {
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
        let sol_backup = ResidueResolver::new().add_cmds([ctl_cmd.exec_residue(), info_cmd.exec_residue()].into_iter());
        // Variables for move
        let item_id = self.id;
        self.sol
            .exec_standard(sol_backup, move |core_sol| {
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
