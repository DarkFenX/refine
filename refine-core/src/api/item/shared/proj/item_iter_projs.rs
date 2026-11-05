use crate::{sol::SolCore, ud::UItemId};

pub(super) fn iter_projectee_uids(
    sol: &SolCore,
    item_uid: UItemId,
) -> impl ExactSizeIterator<Item = UItemId> + use<'_> {
    sol.u_data.items.get(item_uid).iter_projectees().unwrap()
}
