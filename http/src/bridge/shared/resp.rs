#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum CmdResp {
    NoData,
    SingleId(SingleIdResp),
}

#[derive(serde::Serialize)]
pub(crate) struct SingleIdResp {
    id: String,
}
impl SingleIdResp {
    pub(crate) fn new(id: reefast::ReeId) -> Self {
        Self { id: id.to_string() }
    }
}
