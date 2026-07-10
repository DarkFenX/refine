use serde::Deserialize;

use crate::info::HValidInfoMode;

#[derive(Deserialize)]
pub(crate) struct HValidInfoParams {
    pub(in crate::handlers::validate) validation: Option<HValidInfoMode>,
}
