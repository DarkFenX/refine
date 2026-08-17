#[derive(Debug, thiserror::Error)]
pub enum BackrefRenderError {
    #[error("referenced command #{0} does not have results recorded")]
    NotFound(usize),
    #[error("referenced command #{0} exists, but does not have fit ID info")]
    NoFitId(usize),
    #[error("referenced command #{0} exists, but does not have fleet ID info")]
    NoFleetId(usize),
    #[error("referenced command #{0} exists, but does not have item ID info")]
    NoItemId(usize),
    #[error("referenced command #{0} exists, but does not have charge item ID info")]
    NoChargeItemId(usize),
}
