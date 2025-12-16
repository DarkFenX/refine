use crate::{
    ac,
    ad::{AAttr, AAttrId},
    def::OF,
    nd::NAttr,
};

const A_ATTR_ID: AAttrId = ac::attrs::DISALLOW_WARP_JUMP_DRIVE;

pub(in crate::nd::attr) fn mk_n_attr() -> NAttr {
    NAttr {
        eid: None,
        aid: A_ATTR_ID,
        adg_make_attr_fn: Some(make_attr),
        ..
    }
}

fn make_attr() -> AAttr {
    AAttr {
        id: A_ATTR_ID,
        penalizable: false,
        hig: false,
        def_val: OF(0.0),
        ..
    }
}
