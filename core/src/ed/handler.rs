use crate::ed::{EData, EResult};

/// EVE data handler interface definition.
pub trait EveDataHandler: std::fmt::Debug {
    /// Get main EVE data.
    ///
    /// This method should return an error only when it is impossible to fetch the data altogether.
    /// In case of a less impactful error (such as inability to deserialize one specific item within
    /// a big array of data), the error should be recorded as a meaningful warning message and
    /// stored in [`EDataCont::warns`](crate::ed::EDataCont::warns) for appropriate entity.
    fn get_data(&self) -> EResult<EData>;
    /// Get version of the data.
    fn get_data_version(&self) -> EResult<String>;
}
