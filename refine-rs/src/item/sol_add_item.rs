use crate::{
    cmd::{AddItemEnumCmd, AddItemEnumError},
    info::{ItemInfo, ItemInfoMode},
    item::Item,
    sol::SolarSystem,
};

impl<'r, 's> SolarSystem<'r> {
    #[tracing::instrument(name = "itm-add", level = "trace", skip_all)]
    pub async fn add_item(&'s mut self, cmd: AddItemEnumCmd) -> Result<Item<'r, 's>, AddItemEnumError> {
        let item_id = self
            .exec_standard_fallible(move |core_sol| cmd.execute(core_sol).map(|cmd_resp| cmd_resp.item_id))
            .await?;
        let item = Item::new(self, item_id);
        Ok(item)
    }
    #[tracing::instrument(name = "itm-add", level = "trace", skip_all)]
    pub async fn add_item_and_get_info(
        &'s mut self,
        cmd: AddItemEnumCmd,
        item_mode: ItemInfoMode,
    ) -> Result<(Item<'r, 's>, ItemInfo), AddItemEnumError> {
        let (item_id, item_info) = self
            .exec_standard_fallible(move |core_sol| {
                let item_id = cmd.execute(core_sol)?.item_id;
                let mut core_item = core_sol.get_item_mut(&item_id).unwrap();
                let item_info = ItemInfo::from_core(&mut core_item, item_mode);
                Ok::<_, AddItemEnumError>((item_id, item_info))
            })
            .await?;
        let item = Item::new(self, item_id);
        Ok((item, item_info))
    }
}
