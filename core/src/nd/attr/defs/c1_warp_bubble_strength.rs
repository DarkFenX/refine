use crate::{
    ad::{AAttr, AAttrId, AValue},
    nd::NAttr,
};

const ATTR_AID: AAttrId = AAttrId::WARP_BUBBLE_STRENGTH;

pub(in crate::nd::attr) fn mk_n_attr() -> NAttr {
    NAttr {
        aid: ATTR_AID,
        adg_make_attr_fn: Some(make_attr),
        ..
    }
}

fn make_attr() -> AAttr {
    AAttr {
        id: ATTR_AID,
        penalizable: false,
        hig: true,
        def_val: AValue::from_f64(0.0),
        ..
    }
}
