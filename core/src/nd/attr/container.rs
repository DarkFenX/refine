use std::sync::LazyLock;

use crate::{
    ad::AAttrId,
    nd::attr::{NAttr, defs},
    util::RMap,
};

pub(crate) static N_ATTR_MAP: LazyLock<RMap<AAttrId, NAttr>> = LazyLock::new(get_attr_map);

fn get_attr_map() -> RMap<AAttrId, NAttr> {
    [
        defs::c1_warp_bubble_strength::mk_n_attr(),
        defs::c2_disallow_warping::mk_n_attr(),
        defs::c3_disallow_wormhole_jumping::mk_n_attr(),
        defs::c4_disallow_drive_jumping_only::mk_n_attr(),
    ]
    .into_iter()
    .map(|n_attr| (n_attr.aid, n_attr))
    .collect()
}
