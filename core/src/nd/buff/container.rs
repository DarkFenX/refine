use std::sync::LazyLock;

use crate::{
    ad::ABuffId,
    nd::buff::{NBuff, data},
    util::RMap,
};

pub(crate) static N_BUFF_MAP: LazyLock<RMap<ABuffId, NBuff>> = LazyLock::new(get_buff_map);

fn get_buff_map() -> RMap<ABuffId, NBuff> {
    [data::c1_disallow_warp_jump_drive::mk_n_buff()]
        .into_iter()
        .map(|v| (v.aid, v))
        .collect()
}
