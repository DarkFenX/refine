#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum CmdResp {
    NoData,
    SingleId(SingleIdResp),
}

#[derive(serde::Serialize)]
pub(crate) struct SingleIdResp {
    id: reefast::ReeId,
}
impl SingleIdResp {
    pub(crate) fn new(id: reefast::ReeId) -> Self {
        Self { id }
    }
}
