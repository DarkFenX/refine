pub(in crate::sde) enum ReadParseFailReason {
    Read(std::io::Error),
}
impl From<std::io::Error> for ReadParseFailReason {
    fn from(error: std::io::Error) -> Self {
        Self::Read(error)
    }
}
