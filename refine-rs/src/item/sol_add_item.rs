use crate::{
    cmd::{AddItemEnumCmd, AddItemEnumError},
    item::Item,
    sol::SolarSystem,
};

impl<'r, 's> SolarSystem<'r> {
    #[tracing::instrument(name = "itm-add", level = "trace", skip_all)]
    pub async fn add_item(&'s mut self, cmd: AddItemEnumCmd) -> Result<Item<'r, 's>, AddItemEnumError> {
        let item_id = self
            .exec_standard_fallible(move |core_sol| cmd.execute(core_sol).map(|cmd_resp| cmd_resp.item_id))
            .await?;
        Ok(Item::new(self, item_id))
    }
}
