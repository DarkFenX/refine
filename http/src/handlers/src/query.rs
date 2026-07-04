use serde::Deserialize;

use crate::info::HSrcInfoMode;

#[derive(Deserialize)]
pub(crate) struct HSrcInfoParams {
    pub(super) src: Option<HSrcInfoMode>,
}
