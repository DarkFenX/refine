use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum HSideEffectOp {
    Add,
    Perc,
}
