use serde::Serialize;

use super::resp::HCmdResp;

#[derive(Serialize)]
#[serde(transparent)]
pub(crate) struct HCmdResps {
    data: Vec<HCmdResp>,
}
impl HCmdResps {
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }
    pub(crate) fn append(&mut self, resp: HCmdResp) {
        self.data.push(resp);
    }
}
