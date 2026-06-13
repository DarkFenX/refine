// There is an attribute names "disallowDriveJumping" in EVE, but despite its name, it is used not
// for drive jumping only: it is also involved in gate jumping checks, and possibly in other places
// as well. Since there is a need to have an attribute which disables just drive jumping (e.g. for
// bubble effects), this attribute exists.

use crate::{
    ad::{AAttr, AAttrId, AValue},
    nd::NAttr,
};

const ATTR_AID: AAttrId = AAttrId::DISALLOW_DRIVE_JUMPING_ONLY;

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
        hig: false,
        def_val: AValue::from_f64(0.0),
        ..
    }
}
