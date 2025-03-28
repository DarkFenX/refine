#[derive(Debug)]
pub(crate) struct StrMsgError {
    pub(crate) msg: String,
}
impl std::error::Error for StrMsgError {}
impl std::fmt::Display for StrMsgError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
