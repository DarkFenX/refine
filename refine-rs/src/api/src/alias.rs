#[derive(Clone, Eq, PartialEq, Hash, Debug, derive_more::Display)]
pub struct SrcAlias(String);

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl From<String> for SrcAlias {
    fn from(string: String) -> Self {
        Self(string)
    }
}
impl From<&str> for SrcAlias {
    fn from(str: &str) -> Self {
        Self(str.to_string())
    }
}
impl From<SrcAlias> for String {
    fn from(alias: SrcAlias) -> String {
        alias.0
    }
}
impl From<&SrcAlias> for String {
    fn from(alias: &SrcAlias) -> String {
        alias.0.clone()
    }
}
