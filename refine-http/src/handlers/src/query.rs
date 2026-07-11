use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct HSrcInfoParams {
    pub(super) src: Option<rs::SrcInfoMode>,
}
