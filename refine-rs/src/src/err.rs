#[derive(thiserror::Error, Debug)]
#[error("")]
pub struct AliasFoundError {
    pub src_alias: String,
}
