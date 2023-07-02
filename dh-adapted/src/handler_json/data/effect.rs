use super::enums::{CModAfeeFilter, CModBuildStatus, CModOp, CState, CTgtMode};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CEffect {
    id: rc::EEffectId,
    state: CState,
    tgt_mode: CTgtMode,
    is_assist: bool,
    is_offense: bool,
    hisec: Option<bool>,
    lowsec: Option<bool>,
    discharge_attr_id: Option<rc::EAttrId>,
    duration_attr_id: Option<rc::EAttrId>,
    range_attr_id: Option<rc::EAttrId>,
    falloff_attr_id: Option<rc::EAttrId>,
    track_attr_id: Option<rc::EAttrId>,
    chance_attr_id: Option<rc::EAttrId>,
    resist_attr_id: Option<rc::EAttrId>,
    mod_build_status: CModBuildStatus,
    mods: Vec<CAttrMod>,
    stop_ids: Vec<rc::EEffectId>,
}
impl From<&rc::ad::AEffect> for CEffect {
    fn from(a_effect: &rc::ad::AEffect) -> Self {
        CEffect {
            id: a_effect.id,
            state: (&a_effect.state).into(),
            tgt_mode: (&a_effect.tgt_mode).into(),
            is_assist: a_effect.is_assist,
            is_offense: a_effect.is_offense,
            hisec: a_effect.hisec,
            lowsec: a_effect.lowsec,
            discharge_attr_id: a_effect.discharge_attr_id,
            duration_attr_id: a_effect.duration_attr_id,
            range_attr_id: a_effect.range_attr_id,
            falloff_attr_id: a_effect.falloff_attr_id,
            track_attr_id: a_effect.track_attr_id,
            chance_attr_id: a_effect.chance_attr_id,
            resist_attr_id: a_effect.resist_attr_id,
            mod_build_status: (&a_effect.mod_build_status).into(),
            mods: a_effect.mods.iter().map(|v| v.into()).collect(),
            stop_ids: a_effect.stop_ids.clone(),
        }
    }
}
impl Into<rc::ad::AEffect> for &CEffect {
    fn into(self) -> rc::ad::AEffect {
        rc::ad::AEffect {
            id: self.id,
            state: (&self.state).into(),
            tgt_mode: (&self.tgt_mode).into(),
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
            mod_build_status: (&self.mod_build_status).into(),
            mods: self.mods.iter().map(|v| v.into()).collect(),
            stop_ids: self.stop_ids.clone(),
        }
    }
}

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
struct CAttrMod {
    afor_attr_id: rc::EAttrId,
    op: CModOp,
    afee_filter: CModAfeeFilter,
    afee_attr_id: rc::EAttrId,
}
impl From<&rc::ad::AAttrMod> for CAttrMod {
    fn from(a_modifier: &rc::ad::AAttrMod) -> Self {
        CAttrMod {
            afor_attr_id: a_modifier.afor_attr_id,
            op: (&a_modifier.op).into(),
            afee_filter: (&a_modifier.afee_filter).into(),
            afee_attr_id: a_modifier.afee_attr_id,
        }
    }
}
impl Into<rc::ad::AAttrMod> for &CAttrMod {
    fn into(self) -> rc::ad::AAttrMod {
        rc::ad::AAttrMod {
            afor_attr_id: self.afor_attr_id,
            op: (&self.op).into(),
            afee_filter: (&self.afee_filter).into(),
            afee_attr_id: self.afee_attr_id,
        }
    }
}
