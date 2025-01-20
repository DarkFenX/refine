use crate::{
    defs::{AttrVal, EAttrId},
    ec,
    util::StMap,
};

pub(super) fn get_item_volume(attrs: &StMap<EAttrId, AttrVal>) -> Option<AttrVal> {
    attrs.get(&ec::attrs::VOLUME).map(|v| *v)
}
