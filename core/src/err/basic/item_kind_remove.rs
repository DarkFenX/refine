#[derive(thiserror::Error, Debug)]
#[error("{item_kind} cannot be manually removed")]
pub struct ItemKindRemoveError {
    pub item_kind: &'static str,
}
