use crate::{
    defs::{AttrVal, EAttrId},
    ec,
    util::StMap,
};

pub(super) fn get_item_volume(item_attrs: &StMap<EAttrId, AttrVal>) -> Option<AttrVal> {
    item_attrs.get(&ec::attrs::VOLUME).copied()
}
