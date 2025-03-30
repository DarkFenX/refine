use crate::sol::SecStatus;

#[derive(Debug)]
pub struct SecStatusError {
    pub sec_status: SecStatus,
}
impl std::error::Error for SecStatusError {}
impl std::fmt::Display for SecStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "sec status {} is out of allowed range [-10, 5]", self.sec_status)
    }
}
