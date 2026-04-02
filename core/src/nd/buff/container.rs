use std::sync::LazyLock;

use crate::{
    ad::ABuffId,
    nd::buff::{NBuff, defs},
    util::RMap,
};

pub(crate) static N_BUFF_MAP: LazyLock<RMap<ABuffId, NBuff>> = LazyLock::new(get_buff_map);

fn get_buff_map() -> RMap<ABuffId, NBuff> {
    [defs::c1_disallow_warp_jump::mk_n_buff()]
        .into_iter()
        .map(|n_buff| (n_buff.aid, n_buff))
        .collect()
}
