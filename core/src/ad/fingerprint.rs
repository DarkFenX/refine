use crate::def::VERSION;

#[derive(Clone, Eq, PartialEq, Hash, derive_more::Display)]
pub struct AFingerprint(String);
impl AFingerprint {
    pub(crate) fn new(eve_data_version: String, cacher_version: String) -> Self {
        Self(format!("ed{eve_data_version}_adc{cacher_version}_core{VERSION}"))
    }
    pub fn get_str(&self) -> &str {
        &self.0
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AFingerprint {
    pub fn from_string(fingerprint: String) -> Self {
        Self(fingerprint)
    }
    pub fn into_string(self) -> String {
        self.0
    }
}
