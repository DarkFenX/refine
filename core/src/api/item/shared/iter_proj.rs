use crate::{sol::SolarSystem, ud::UItemId};

pub(in crate::api) fn iter_projectee_keys(
    sol: &SolarSystem,
    item_key: UItemId,
) -> impl ExactSizeIterator<Item = UItemId> + use<'_> {
    sol.u_data.items.get(item_key).iter_projectees().unwrap()
}
