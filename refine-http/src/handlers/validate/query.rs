use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct ValInfoParams {
    pub(super) validation: Option<rs::val::ValInfoMode>,
}
