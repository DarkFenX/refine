use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct SrcParams {
    #[serde(default)]
    src: rs::src::SrcInfoMode,
}
impl SrcParams {
    pub(super) fn into_info_mode(self) -> rs::src::SrcInfoMode {
        self.src
    }
}
