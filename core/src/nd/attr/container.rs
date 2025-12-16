use std::sync::LazyLock;

use crate::{
    ad::AAttrId,
    nd::attr::{NAttr, data},
    util::RMap,
};

pub(crate) static N_ATTR_MAP: LazyLock<RMap<AAttrId, NAttr>> = LazyLock::new(get_attr_map);

fn get_attr_map() -> RMap<AAttrId, NAttr> {
    [data::c1_disallow_warp_jump_drive::mk_n_attr()]
        .into_iter()
        .map(|v| (v.aid, v))
        .collect()
}
