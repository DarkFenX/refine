use crate::{
    CmdResps, Item, ItemAddEnumCmd, ItemInfo, ItemInfoCmdBr, SolarSystem, err::ItemAddEnumError,
    shared::ResidueResolver,
};

impl<'r, 's> SolarSystem<'r> {
    #[tracing::instrument(name = "itm-add", level = "trace", skip_all)]
    pub async fn add_item(&'s mut self, ctl_cmd: ItemAddEnumCmd) -> Result<Item<'r, 's>, ItemAddEnumError> {
        let sol_backup = ResidueResolver::new().add_cmd(ctl_cmd.exec_residue());
        let item_id = self
            .exec_standard(sol_backup, |core_sol| {
                ctl_cmd.execute(core_sol).map(|ctl_cmd_resp| ctl_cmd_resp.item_id)
            })
            .await?;
        let item = Item::new(self, item_id);
        Ok(item)
    }
    #[tracing::instrument(name = "itm-add-inf", level = "trace", skip_all)]
    pub async fn add_item_and_get_info(
        &'s mut self,
        ctl_cmd: ItemAddEnumCmd,
        info_cmd: ItemInfoCmdBr,
    ) -> Result<(Item<'r, 's>, ItemInfo), ItemAddEnumError> {
        let sol_backup = ResidueResolver::new().add_cmds([ctl_cmd.exec_residue(), info_cmd.exec_residue()].into_iter());
        let (item_id, item_info) = self
            .exec_standard(sol_backup, |core_sol| {
                let ctl_cmd_resp = ctl_cmd.execute(core_sol)?;
                let item_id = ctl_cmd_resp.item_id;
                let ctl_cmd_resps = CmdResps::with_resp(ctl_cmd_resp.into());
                let info_cmd = info_cmd.br_resolve(&ctl_cmd_resps);
                // The item which we just added should still be there, we hold sol mutex
                let mut core_item = core_sol.get_item_mut(&item_id).unwrap();
                let item_info = info_cmd.execute(&mut core_item);
                Ok::<_, ItemAddEnumError>((item_id, item_info))
            })
            .await?;
        let item = Item::new(self, item_id);
        Ok((item, item_info))
    }
}
