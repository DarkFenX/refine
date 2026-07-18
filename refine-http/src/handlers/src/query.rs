#[derive(serde::Deserialize)]
pub(crate) struct SrcInfoParams {
    pub(super) src: Option<rs::src::SrcInfoMode>,
}
