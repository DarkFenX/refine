use crate::{AddItemEnumCmd, Item, ItemInfo, ItemInfoArgs, SolarSystem, err::AddItemEnumError};

impl<'r, 's> SolarSystem<'r> {
    #[tracing::instrument(name = "itm-add", level = "trace", skip_all)]
    pub async fn add_item(&'s mut self, cmd: AddItemEnumCmd) -> Result<Item<'r, 's>, AddItemEnumError> {
        let item_id = self
            .exec_standard_fallible(move |core_sol| cmd.execute(core_sol).map(|cmd_resp| cmd_resp.item_id))
            .await?;
        let item = Item::new(self, item_id);
        Ok(item)
    }
    #[tracing::instrument(name = "itm-add-inf", level = "trace", skip_all)]
    pub async fn add_item_and_get_info(
        &'s mut self,
        exec_cmd: AddItemEnumCmd,
        info_args: ItemInfoArgs,
    ) -> Result<(Item<'r, 's>, ItemInfo), AddItemEnumError> {
        let (item_id, item_info) = self
            .exec_standard_fallible(move |core_sol| {
                let item_id = exec_cmd.execute(core_sol)?.item_id;
                let mut core_item = core_sol.get_item_mut(&item_id).unwrap();
                let item_info = ItemInfo::from_core(&mut core_item, info_args);
                Ok::<_, AddItemEnumError>((item_id, item_info))
            })
            .await?;
        let item = Item::new(self, item_id);
        Ok((item, item_info))
    }
}
