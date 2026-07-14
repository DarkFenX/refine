use crate::{
    cmd::{ChangeItemEnumCmd, ChangeItemEnumError, ChangedItemIdsResp},
    item::Item,
};

impl Item<'_, '_> {
    #[tracing::instrument(name = "itm-chg", level = "trace", skip_all)]
    pub async fn change(self, cmd: ChangeItemEnumCmd) -> Result<ChangedItemIdsResp, ChangeItemEnumError> {
        let item_id = self.id;
        self.sol
            .exec_standard_fallible(move |core_sol| {
                // Holding mutex on sol - nothing can remove the item before we get it here
                let mut core_item = core_sol.get_item_mut(&item_id).unwrap();
                cmd.execute(&mut core_item)
            })
            .await
    }
}
