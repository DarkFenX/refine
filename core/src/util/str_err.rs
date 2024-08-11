#[derive(Debug)]
pub(crate) struct StrMsgError {
    msg: String,
}
impl StrMsgError {
    pub(crate) fn new(msg: String) -> Self {
        Self { msg }
    }
}
impl std::error::Error for StrMsgError {}
impl std::fmt::Display for StrMsgError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
