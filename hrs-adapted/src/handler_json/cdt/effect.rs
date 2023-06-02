use super::enums::{ModAfeeFilter, ModAggrMode, ModBuildStatus, ModOp, State, TgtMode};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json::cdt) struct Effect {
    id: rc::ReeInt,
    state: State,
    tgt_mode: TgtMode,
    is_assist: bool,
    is_offense: bool,
    hisec: Option<bool>,
    lowsec: Option<bool>,
    discharge_attr_id: Option<rc::ReeInt>,
    duration_attr_id: Option<rc::ReeInt>,
    range_attr_id: Option<rc::ReeInt>,
    falloff_attr_id: Option<rc::ReeInt>,
    track_attr_id: Option<rc::ReeInt>,
    chance_attr_id: Option<rc::ReeInt>,
    resist_attr_id: Option<rc::ReeInt>,
    mod_build_status: ModBuildStatus,
    mods: Vec<AttrMod>,
    stop_ids: Vec<rc::ReeInt>,
}
impl From<rc::adt::Effect> for Effect {
    fn from(value: rc::adt::Effect) -> Self {
        Effect {
            id: value.id,
            state: value.state.into(),
            tgt_mode: value.tgt_mode.into(),
            is_assist: value.is_assist,
            is_offense: value.is_offense,
            hisec: value.hisec,
            lowsec: value.lowsec,
            discharge_attr_id: value.discharge_attr_id,
            duration_attr_id: value.duration_attr_id,
            range_attr_id: value.range_attr_id,
            falloff_attr_id: value.falloff_attr_id,
            track_attr_id: value.track_attr_id,
            chance_attr_id: value.chance_attr_id,
            resist_attr_id: value.resist_attr_id,
            mod_build_status: value.mod_build_status.into(),
            mods: value.mods.iter().map(|v| (*v).into()).collect(),
            stop_ids: value.stop_ids,
        }
    }
}
impl Into<rc::adt::Effect> for Effect {
    fn into(self) -> rc::adt::Effect {
        rc::adt::Effect {
            id: self.id,
            state: self.state.into(),
            tgt_mode: self.tgt_mode.into(),
            is_assist: self.is_assist,
            is_offense: self.is_offense,
            hisec: self.hisec,
            lowsec: self.lowsec,
            discharge_attr_id: self.discharge_attr_id,
            duration_attr_id: self.duration_attr_id,
            range_attr_id: self.range_attr_id,
            falloff_attr_id: self.falloff_attr_id,
            track_attr_id: self.track_attr_id,
            chance_attr_id: self.chance_attr_id,
            resist_attr_id: self.resist_attr_id,
            mod_build_status: self.mod_build_status.into(),
            mods: self.mods.iter().map(|v| (*v).into()).collect(),
            stop_ids: self.stop_ids,
        }
    }
}

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
struct AttrMod {
    afor_attr_id: rc::ReeInt,
    aggr_mode: ModAggrMode,
    op: ModOp,
    afee_filter: ModAfeeFilter,
    afee_attr_id: rc::ReeInt,
}
impl From<rc::adt::AttrMod> for AttrMod {
    fn from(value: rc::adt::AttrMod) -> Self {
        AttrMod {
            afor_attr_id: value.afor_attr_id,
            aggr_mode: value.aggr_mode.into(),
            op: value.op.into(),
            afee_filter: value.afee_filter.into(),
            afee_attr_id: value.afee_attr_id,
        }
    }
}
impl Into<rc::adt::AttrMod> for AttrMod {
    fn into(self) -> rc::adt::AttrMod {
        rc::adt::AttrMod {
            afor_attr_id: self.afor_attr_id,
            aggr_mode: self.aggr_mode.into(),
            op: self.op.into(),
            afee_filter: self.afee_filter.into(),
            afee_attr_id: self.afee_attr_id,
        }
    }
}
