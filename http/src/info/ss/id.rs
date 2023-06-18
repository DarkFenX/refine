#[derive(serde::Serialize)]
pub(crate) struct HSsInfoId {
    pub(crate) id: String,
}
impl From<String> for HSsInfoId {
    fn from(core_id: String) -> Self {
        Self { id: core_id }
    }
}
