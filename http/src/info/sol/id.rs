use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct HSolInfoId {
    id: String,
}
impl HSolInfoId {
    pub(in crate::info::sol) fn from_sol_id(sol_id: String) -> Self {
        Self { id: sol_id }
    }
}
