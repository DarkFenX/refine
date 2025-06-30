use crate::{def::ItemKey, sol::SolarSystem};

pub(in crate::sol::api) fn iter_projectee_item_keys(
    sol: &SolarSystem,
    item_key: ItemKey,
) -> impl ExactSizeIterator<Item = ItemKey> + use<'_> {
    sol.uad.items.get(item_key).iter_projectee_item_keys().unwrap().copied()
}
