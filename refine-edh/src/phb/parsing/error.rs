pub(in crate::phb) enum ReadParseFailReason {
    Read(std::io::Error),
    Parse(serde_json::Error),
}
impl From<serde_json::Error> for ReadParseFailReason {
    fn from(error: serde_json::Error) -> Self {
        match error.is_io() {
            true => Self::Read(error.into()),
            false => Self::Parse(error),
        }
    }
}
