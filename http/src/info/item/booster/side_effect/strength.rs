use super::HSideEffectOp;

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HSideEffectStr {
    pub(crate) op: HSideEffectOp,
    pub(crate) val: rc::AttrVal,
}
impl HSideEffectStr {
    fn new(op: HSideEffectOp, val: rc::AttrVal) -> Self {
        Self { op, val }
    }
    pub(in crate::info::item::booster::side_effect) fn from_core_str(
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::SolItemId,
        core_se_str: &rc::SideEffectStr,
    ) -> Option<Self> {
        let val = match core_sol.get_item_attr(item_id, &core_se_str.attr_id) {
            Ok(val) => val.extra,
            _ => return None,
        };
        match core_se_str.op {
            rc::ad::AOp::PreAssign => None,
            rc::ad::AOp::PreMul => Some(HSideEffectStr::new(
                HSideEffectOp::Perc,
                (val - rc::OF(1.0)) * rc::OF(100.0),
            )),
            rc::ad::AOp::PreDiv => match val {
                rc::OF(0.0) => None,
                _ => Some(HSideEffectStr::new(
                    HSideEffectOp::Perc,
                    (rc::OF(1.0) / val - rc::OF(1.0)) * rc::OF(100.0),
                )),
            },
            rc::ad::AOp::Add => Some(HSideEffectStr::new(HSideEffectOp::Add, val)),
            rc::ad::AOp::Sub => Some(HSideEffectStr::new(HSideEffectOp::Add, -val)),
            rc::ad::AOp::PostMul => Some(HSideEffectStr::new(
                HSideEffectOp::Perc,
                (val - rc::OF(1.0)) * rc::OF(100.0),
            )),
            rc::ad::AOp::PostMulImmune => Some(HSideEffectStr::new(
                HSideEffectOp::Perc,
                (val - rc::OF(1.0)) * rc::OF(100.0),
            )),
            rc::ad::AOp::PostDiv => match val {
                rc::OF(0.0) => None,
                _ => Some(HSideEffectStr::new(
                    HSideEffectOp::Perc,
                    (rc::OF(1.0) / val - rc::OF(1.0)) * rc::OF(100.0),
                )),
            },
            rc::ad::AOp::PostPerc => Some(HSideEffectStr::new(HSideEffectOp::Perc, val)),
            rc::ad::AOp::PostAssign => None,
        }
    }
}
