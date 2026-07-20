use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct HValidInfoParams {
    pub(super) validation: Option<rs::val::ValInfoMode>,
}
