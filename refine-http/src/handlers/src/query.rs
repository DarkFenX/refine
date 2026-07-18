use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct SrcInfoParams {
    pub(super) src: Option<rs::src::SrcInfoMode>,
}
