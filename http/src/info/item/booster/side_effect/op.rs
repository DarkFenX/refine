#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HSideEffectOp {
    Add,
    Perc,
}
