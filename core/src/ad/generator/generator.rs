use super::{error::ADataGeneratorError, support::AdgSupport};
use crate::{
    ad::AData,
    ed::{EData, EveDataHandler},
};

/// Conducts adapted data generation.
pub(crate) struct ADataGenerator {
    pub(super) e_data: EData,
    pub(super) a_data: AData,
    pub(super) support: AdgSupport,
}
impl ADataGenerator {
    pub(crate) fn new() -> Self {
        Self {
            e_data: EData::new(),
            a_data: AData::new(),
            support: AdgSupport::new(),
        }
    }
    pub(crate) fn generate(mut self, ed_handler: &dyn EveDataHandler) -> Result<AData, ADataGeneratorError> {
        self.fetch_data(ed_handler)?;
        self.dedup_pks();
        self.normalize();
        self.support.fill(&self.e_data);
        self.clean_unused()?;
        self.validate();
        self.convert_main();
        self.customize();
        self.convert_aux();
        Ok(self.a_data)
    }
}
