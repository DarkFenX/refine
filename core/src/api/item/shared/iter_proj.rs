use crate::{sol::SolarSystem, ud::UItemId};

pub(in crate::api) fn iter_projectee_uids(
    sol: &SolarSystem,
    item_uid: UItemId,
) -> impl ExactSizeIterator<Item = UItemId> + use<'_> {
    sol.u_data.items.get(item_uid).iter_projectees().unwrap()
}
