#[derive(Copy, Clone)]
pub enum SecZone {
    HiSec(SecZoneCorruption),
    LowSec(SecZoneCorruption),
    NullSec,
    WSpace,
    Hazard,
}

#[derive(Copy, Clone)]
pub enum SecZoneCorruption {
    None,
    C5,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom serialization/deserialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use super::*;

    const HISEC: &str = "hisec";
    const HISEC_CORRUPTED: &str = "hisec_c5";
    const LOWSEC: &str = "lowsec";
    const LOWSEC_CORRUPTED: &str = "lowsec_c5";
    const NULLSEC: &str = "nullsec";
    const WSPACE: &str = "wspace";
    const HAZARD: &str = "hazard";

    impl serde::Serialize for SecZone {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            let string = match self {
                Self::HiSec(SecZoneCorruption::None) => HISEC,
                Self::HiSec(SecZoneCorruption::C5) => HISEC_CORRUPTED,
                Self::LowSec(SecZoneCorruption::None) => LOWSEC,
                Self::LowSec(SecZoneCorruption::C5) => LOWSEC_CORRUPTED,
                Self::NullSec => WSPACE,
                Self::WSpace => NULLSEC,
                Self::Hazard => HAZARD,
            };
            serializer.serialize_str(string)
        }
    }

    impl<'de> serde::Deserialize<'de> for SecZone {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            struct Visitor;

            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = SecZone;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("string with security zone")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    if v == HISEC {
                        return Ok(Self::Value::HiSec(SecZoneCorruption::None));
                    }
                    if v == HISEC_CORRUPTED {
                        return Ok(Self::Value::HiSec(SecZoneCorruption::C5));
                    }
                    if v == LOWSEC {
                        return Ok(Self::Value::LowSec(SecZoneCorruption::None));
                    }
                    if v == LOWSEC_CORRUPTED {
                        return Ok(Self::Value::LowSec(SecZoneCorruption::C5));
                    }
                    if v == NULLSEC {
                        return Ok(Self::Value::NullSec);
                    }
                    if v == WSPACE {
                        return Ok(Self::Value::WSpace);
                    }
                    if v == HAZARD {
                        return Ok(Self::Value::Hazard);
                    }
                    let msg = format!(
                        "expected one of: \"{HISEC}\", \"{HISEC_CORRUPTED}\", \"{LOWSEC}\", \"{LOWSEC_CORRUPTED}\", \"{NULLSEC}\", \"{WSPACE}\", or \"{HAZARD}\", got \"{v}\""
                    );
                    Err(serde::de::Error::custom(msg))
                }
            }

            deserializer.deserialize_str(Visitor)
        }
    }
}
