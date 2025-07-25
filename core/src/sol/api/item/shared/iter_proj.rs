use crate::{sol::SolarSystem, ud::UItemKey};

pub(in crate::sol::api) fn iter_projectee_keys(
    sol: &SolarSystem,
    item_key: UItemKey,
) -> impl ExactSizeIterator<Item = UItemKey> + use<'_> {
    sol.u_data.items.get(item_key).iter_projectees().unwrap()
}
