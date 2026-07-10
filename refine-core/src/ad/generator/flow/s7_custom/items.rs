use crate::{ad::ADataGenerator, nd::N_ITEM_MAP};

impl ADataGenerator {
    pub(super) fn customize_items(&mut self) {
        for n_item in N_ITEM_MAP.values() {
            if let Some(item_updater) = n_item.adg_update_item_fn {
                let Some(a_item) = self.a_data.items.data.get_mut(&n_item.aid) else {
                    let warning = format!("item {}: not found for customization", n_item.aid);
                    self.a_data.warnings.customization.push(warning);
                    return;
                };
                item_updater(a_item);
            }
        }
    }
}
