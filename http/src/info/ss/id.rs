#[derive(serde::Serialize)]
pub(crate) struct HSsInfoId {
    pub(crate) id: String,
}
impl From<String> for HSsInfoId {
    fn from(ss_id: String) -> Self {
        Self { id: ss_id }
    }
}
