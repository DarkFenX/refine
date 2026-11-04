use crate::{sol::CoreInner, ud::UItemId};

pub(super) fn iter_projectee_uids(
    core: &CoreInner,
    item_uid: UItemId,
) -> impl ExactSizeIterator<Item = UItemId> + use<'_> {
    core.u_data.items.get(item_uid).iter_projectees().unwrap()
}
