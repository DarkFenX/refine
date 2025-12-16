use crate::{
    ac,
    ad::{ABuff, ABuffAffecteeFilter, ABuffAggrMode, ABuffId, ABuffModifier, AOp},
    nd::NBuff,
};

const A_BUFF_ID: ABuffId = ac::buffs::DISALLOW_WARP_JUMP;

pub(in crate::nd::buff) fn mk_n_buff() -> NBuff {
    NBuff {
        eid: None,
        aid: A_BUFF_ID,
        adg_make_buff_fn: Some(make_buff),
        ..
    }
}

fn make_buff() -> ABuff {
    ABuff {
        id: A_BUFF_ID,
        aggr_mode: ABuffAggrMode::Max,
        op: AOp::Add,
        mods: vec![ABuffModifier {
            affectee_filter: ABuffAffecteeFilter::Direct,
            affectee_attr_id: ac::attrs::DISALLOW_WARPING_JUMPING,
        }],
    }
}
