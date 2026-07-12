#[derive(Clone, Eq, PartialEq, Hash, Debug, derive_more::Display)]
pub struct SrcAlias(String);

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl From<String> for SrcAlias {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl From<&str> for SrcAlias {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
impl Into<String> for SrcAlias {
    fn into(self) -> String {
        self.0
    }
}
impl Into<String> for &SrcAlias {
    fn into(self) -> String {
        self.0.clone()
    }
}
