use crate::{
    cmd::{CreateItemEnumCmd, CreateItemEnumError},
    item::Item,
    sol::SolarSystem,
};

impl<'r, 's> SolarSystem<'r> {
    #[tracing::instrument(name = "itm-crt", level = "trace", skip_all)]
    pub async fn create_item(&'s mut self, cmd: CreateItemEnumCmd) -> Result<Item<'r, 's>, CreateItemEnumError> {
        let item_id = self
            .exec_standard_fallible(move |core_sol| cmd.execute(core_sol).map(|cmd_resp| cmd_resp.item_id))
            .await?;
        Ok(Item::new(self, item_id))
    }
}
