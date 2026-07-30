use crate::ad::{
    AAttrId, AEffectBuff, AEffectCatId, AEffectId, AEffectModifiers, AEffectStopIds, AEffectWeaponsTimerApplication,
    AState,
};

pub struct AEffect {
    pub id: AEffectId,
    pub category: AEffectCatId,
    pub state: AState,
    pub modifiers: AEffectModifiers = AEffectModifiers::new(),
    pub stopped_effect_ids: AEffectStopIds = AEffectStopIds::new(),
    pub buff: Option<AEffectBuff> = None,
    pub weapons_timer: Option<AEffectWeaponsTimerApplication> = None,
    pub is_assist: bool = false,
    pub is_offense: bool = false,
    pub banned_in_hisec: bool = false,
    pub banned_in_lowsec: bool = false,
    pub discharge_attr_id: Option<AAttrId> = None,
    pub duration_attr_id: Option<AAttrId> = None,
    pub range_attr_id: Option<AAttrId> = None,
    pub falloff_attr_id: Option<AAttrId> = None,
    pub track_attr_id: Option<AAttrId> = None,
    pub chance_attr_id: Option<AAttrId> = None,
    pub resist_attr_id: Option<AAttrId> = None,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde-ad")]
mod custom_serde_ad {
    use serde::{
        de::{Deserialize, Deserializer, Error, SeqAccess, Visitor},
        ser::{Serialize, SerializeTuple, Serializer},
    };

    use super::*;

    const FIELDS: usize = 18;

    impl Serialize for AEffect {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut tuple = serializer.serialize_tuple(FIELDS)?;
            tuple.serialize_element(&self.id)?;
            tuple.serialize_element(&self.category)?;
            tuple.serialize_element(&self.state)?;
            tuple.serialize_element(&self.modifiers)?;
            tuple.serialize_element(&self.stopped_effect_ids)?;
            tuple.serialize_element(&self.buff)?;
            tuple.serialize_element(&self.weapons_timer)?;
            tuple.serialize_element(&self.is_assist)?;
            tuple.serialize_element(&self.is_offense)?;
            tuple.serialize_element(&self.banned_in_hisec)?;
            tuple.serialize_element(&self.banned_in_lowsec)?;
            tuple.serialize_element(&self.discharge_attr_id)?;
            tuple.serialize_element(&self.duration_attr_id)?;
            tuple.serialize_element(&self.range_attr_id)?;
            tuple.serialize_element(&self.falloff_attr_id)?;
            tuple.serialize_element(&self.track_attr_id)?;
            tuple.serialize_element(&self.chance_attr_id)?;
            tuple.serialize_element(&self.resist_attr_id)?;
            tuple.end()
        }
    }

    impl<'de> Deserialize<'de> for AEffect {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl;

            impl<'de> Visitor<'de> for VisitorImpl {
                type Value = AEffect;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("sequence with 18 elements")
                }

                fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
                where
                    S: SeqAccess<'de>,
                {
                    Ok(Self::Value {
                        id: seq.next_element()?.ok_or(Error::invalid_length(0, &self))?,
                        category: seq.next_element()?.ok_or(Error::invalid_length(1, &self))?,
                        state: seq.next_element()?.ok_or(Error::invalid_length(2, &self))?,
                        modifiers: seq.next_element()?.ok_or(Error::invalid_length(3, &self))?,
                        stopped_effect_ids: seq.next_element()?.ok_or(Error::invalid_length(4, &self))?,
                        buff: seq.next_element()?.ok_or(Error::invalid_length(5, &self))?,
                        weapons_timer: seq.next_element()?.ok_or(Error::invalid_length(6, &self))?,
                        is_assist: seq.next_element()?.ok_or(Error::invalid_length(7, &self))?,
                        is_offense: seq.next_element()?.ok_or(Error::invalid_length(8, &self))?,
                        banned_in_hisec: seq.next_element()?.ok_or(Error::invalid_length(9, &self))?,
                        banned_in_lowsec: seq.next_element()?.ok_or(Error::invalid_length(10, &self))?,
                        discharge_attr_id: seq.next_element()?.ok_or(Error::invalid_length(11, &self))?,
                        duration_attr_id: seq.next_element()?.ok_or(Error::invalid_length(12, &self))?,
                        range_attr_id: seq.next_element()?.ok_or(Error::invalid_length(13, &self))?,
                        falloff_attr_id: seq.next_element()?.ok_or(Error::invalid_length(14, &self))?,
                        track_attr_id: seq.next_element()?.ok_or(Error::invalid_length(15, &self))?,
                        chance_attr_id: seq.next_element()?.ok_or(Error::invalid_length(16, &self))?,
                        resist_attr_id: seq.next_element()?.ok_or(Error::invalid_length(17, &self))?,
                    })
                }
            }

            deserializer.deserialize_tuple(FIELDS, VisitorImpl)
        }
    }
}
