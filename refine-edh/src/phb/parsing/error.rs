pub(in crate::phb) enum ReadParseFailReason {
    ReadFailed(String),
    ParseFailed(String),
}
impl From<serde_json::Error> for ReadParseFailReason {
    fn from(error: serde_json::Error) -> Self {
        match error.is_io() {
            true => Self::ReadFailed(error.to_string()),
            false => Self::ParseFailed(error.to_string()),
        }
    }
}
