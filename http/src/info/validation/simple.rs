use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct HValResultSimple {
    passed: bool,
}
impl From<bool> for HValResultSimple {
    fn from(passed: bool) -> Self {
        Self { passed }
    }
}
