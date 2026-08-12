pub(in crate::sde) enum ReadParseFailReason {
    Read(std::io::Error),
    Parse(serde_json::Error),
}
impl From<std::io::Error> for ReadParseFailReason {
    fn from(error: std::io::Error) -> Self {
        Self::Read(error)
    }
}
impl From<serde_json::Error> for ReadParseFailReason {
    fn from(error: serde_json::Error) -> Self {
        Self::Parse(error)
    }
}
