use crate::{AddItemEnumCmd, Item, ItemInfo, ItemInfoCmd, SolarSystem, err::AddItemEnumError};

impl<'r, 's> SolarSystem<'r> {
    #[tracing::instrument(name = "itm-add", level = "trace", skip_all)]
    pub async fn add_item(&'s mut self, ctl_cmd: AddItemEnumCmd) -> Result<Item<'r, 's>, AddItemEnumError> {
        let item_id = self
            .exec_standard_fallible(move |core_sol| ctl_cmd.execute(core_sol).map(|ctl_cmd_resp| ctl_cmd_resp.item_id))
            .await?;
        let item = Item::new(self, item_id);
        Ok(item)
    }
    #[tracing::instrument(name = "itm-add-inf", level = "trace", skip_all)]
    pub async fn add_item_and_get_info(
        &'s mut self,
        ctl_cmd: AddItemEnumCmd,
        info_cmd: ItemInfoCmd,
    ) -> Result<(Item<'r, 's>, ItemInfo), AddItemEnumError> {
        let (item_id, item_info) = self
            .exec_standard_fallible(move |core_sol| {
                let item_id = ctl_cmd.execute(core_sol)?.item_id;
                let mut core_item = core_sol.get_item_mut(&item_id).unwrap();
                let item_info = info_cmd.execute(&mut core_item);
                Ok::<_, AddItemEnumError>((item_id, item_info))
            })
            .await?;
        let item = Item::new(self, item_id);
        Ok((item, item_info))
    }
}
