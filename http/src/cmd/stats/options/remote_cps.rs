#[derive(Copy, Clone, educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(in crate::cmd) struct HStatOptionItemRemoteCps {
    #[serde(default)]
    #[educe(Default = false)]
    pub(in crate::cmd) ignore_state: bool,
}
