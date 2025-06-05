#[derive(serde::Serialize)]
pub(crate) struct HSolInfoId {
    id: String,
}
impl From<String> for HSolInfoId {
    fn from(sol_id: String) -> Self {
        Self { id: sol_id }
    }
}
