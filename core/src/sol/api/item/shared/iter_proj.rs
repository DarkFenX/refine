use crate::{sol::SolarSystem, uad::UadItemKey};

pub(in crate::sol::api) fn iter_projectee_keys(
    sol: &SolarSystem,
    item_key: UadItemKey,
) -> impl ExactSizeIterator<Item = UadItemKey> + use<'_> {
    sol.uad.items.get(item_key).iter_projectees().unwrap()
}
