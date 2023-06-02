use super::enums::{ModAfeeFilter, ModAggrMode, ModOp};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct Buff {
    id: rc::ReeInt,
    aggr_mode: ModAggrMode,
    op: ModOp,
    mods: Vec<BuffAttrMod>,
}
impl From<&rc::adt::ABuff> for Buff {
    fn from(value: &rc::adt::ABuff) -> Self {
        Buff {
            id: value.id,
            aggr_mode: (&value.aggr_mode).into(),
            op: (&value.op).into(),
            mods: value.mods.iter().map(|v| v.into()).collect(),
        }
    }
}
impl Into<rc::adt::ABuff> for &Buff {
    fn into(self) -> rc::adt::ABuff {
        rc::adt::ABuff {
            id: self.id,
            aggr_mode: (&self.aggr_mode).into(),
            op: (&self.op).into(),
            mods: self.mods.iter().map(|v| v.into()).collect(),
        }
    }
}

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
struct BuffAttrMod {
    afee_filter: ModAfeeFilter,
    afee_attr_id: rc::ReeInt,
}
impl From<&rc::adt::ABuffAttrMod> for BuffAttrMod {
    fn from(value: &rc::adt::ABuffAttrMod) -> Self {
        BuffAttrMod {
            afee_filter: (&value.afee_filter).into(),
            afee_attr_id: value.afee_attr_id,
        }
    }
}
impl Into<rc::adt::ABuffAttrMod> for &BuffAttrMod {
    fn into(self) -> rc::adt::ABuffAttrMod {
        rc::adt::ABuffAttrMod {
            afee_filter: (&self.afee_filter).into(),
            afee_attr_id: self.afee_attr_id,
        }
    }
}
