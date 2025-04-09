use crate::sol::SecStatus;

#[derive(thiserror::Error, Debug)]
#[error("sec status {sec_status} is out of allowed range [-10, 5]")]
pub struct SecStatusError {
    pub sec_status: SecStatus,
}
