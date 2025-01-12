#[derive(serde::Serialize)]
pub(crate) struct HValidInfoSimple {
    passed: bool,
}
impl From<bool> for HValidInfoSimple {
    fn from(passed: bool) -> Self {
        Self { passed }
    }
}
