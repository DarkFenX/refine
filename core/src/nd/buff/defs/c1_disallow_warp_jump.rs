use crate::{
    ad::{AAttrId, ABuff, ABuffAffecteeFilter, ABuffAggrMode, ABuffId, ABuffModifier, AOp},
    nd::NBuff,
};

const BUFF_AID: ABuffId = ABuffId::DISALLOW_WARP_JUMP;

pub(in crate::nd::buff) fn mk_n_buff() -> NBuff {
    NBuff {
        aid: BUFF_AID,
        adg_make_buff_fn: Some(make_buff),
        ..
    }
}

fn make_buff() -> ABuff {
    ABuff {
        id: BUFF_AID,
        aggr_mode: ABuffAggrMode::Max,
        op: AOp::Add,
        mods: [
            ABuffModifier {
                affectee_filter: ABuffAffecteeFilter::Direct,
                affectee_attr_id: AAttrId::DISALLOW_WARPING,
            },
            ABuffModifier {
                affectee_filter: ABuffAffecteeFilter::Direct,
                affectee_attr_id: AAttrId::DISALLOW_DRIVE_JUMPING,
            },
        ]
        .into_iter()
        .collect(),
    }
}
