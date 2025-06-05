use crate::{ad, sol::uad::item::UadItem};

pub(in crate::sol::svc) fn get_resist_a_attr_id(item: &UadItem, a_effect: &ad::AEffect) -> Option<ad::AAttrId> {
    match a_effect.resist_attr_id {
        Some(resist_a_attr_id) => Some(resist_a_attr_id),
        None => match item.get_a_extras() {
            Some(a_extras) => a_extras.remote_resist_attr_id,
            None => None,
        },
    }
}
