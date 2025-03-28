use super::HSideEffectOp;

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HSideEffectStr {
    pub(crate) op: HSideEffectOp,
    pub(crate) val: rc::AttrVal,
}
impl HSideEffectStr {
    pub(in crate::info::item::booster::side_effect) fn from_core_str(
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
        core_se_str: &rc::SideEffectStr,
    ) -> Option<Self> {
        let val = match core_sol.get_item_attr(item_id, &core_se_str.attr_id) {
            Ok(val) => val.extra,
            _ => return None,
        };
        match core_se_str.op {
            rc::OpInfo::Add => Some(HSideEffectStr {
                op: HSideEffectOp::Add,
                val,
            }),
            rc::OpInfo::Sub => Some(HSideEffectStr {
                op: HSideEffectOp::Add,
                val: -val,
            }),
            rc::OpInfo::PreMul | rc::OpInfo::PostMul | rc::OpInfo::ExtraMul => Some(HSideEffectStr {
                op: HSideEffectOp::Perc,
                val: (val - rc::AttrVal::from(1.0)) * rc::AttrVal::from(100.0),
            }),
            rc::OpInfo::PreDiv | rc::OpInfo::PostDiv => match val.into_inner() {
                0.0 => None,
                _ => Some(HSideEffectStr {
                    op: HSideEffectOp::Perc,
                    val: (rc::AttrVal::from(1.0) / val - rc::AttrVal::from(1.0)) * rc::AttrVal::from(100.0),
                }),
            },
            rc::OpInfo::PostPerc => Some(HSideEffectStr {
                op: HSideEffectOp::Perc,
                val,
            }),
            _ => None,
        }
    }
}
