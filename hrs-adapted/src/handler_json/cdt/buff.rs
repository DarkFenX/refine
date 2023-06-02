use super::enums::{ModAfeeFilter, ModAggrMode, ModOp};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct Buff {
    id: rc::ReeInt,
    aggr_mode: ModAggrMode,
    op: ModOp,
    mods: Vec<BuffAttrMod>,
}
impl From<&rc::adt::Buff> for Buff {
    fn from(value: &rc::adt::Buff) -> Self {
        Buff {
            id: value.id,
            aggr_mode: (&value.aggr_mode).into(),
            op: (&value.op).into(),
            mods: value.mods.iter().map(|v| v.into()).collect(),
        }
    }
}
impl Into<rc::adt::Buff> for &Buff {
    fn into(self) -> rc::adt::Buff {
        rc::adt::Buff {
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
impl From<&rc::adt::BuffAttrMod> for BuffAttrMod {
    fn from(value: &rc::adt::BuffAttrMod) -> Self {
        BuffAttrMod {
            afee_filter: (&value.afee_filter).into(),
            afee_attr_id: value.afee_attr_id,
        }
    }
}
impl Into<rc::adt::BuffAttrMod> for &BuffAttrMod {
    fn into(self) -> rc::adt::BuffAttrMod {
        rc::adt::BuffAttrMod {
            afee_filter: (&self.afee_filter).into(),
            afee_attr_id: self.afee_attr_id,
        }
    }
}
