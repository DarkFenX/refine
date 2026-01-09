use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::info::item::item_booster::side_effect) enum HSideEffectOp {
    Add,
    Perc,
}
