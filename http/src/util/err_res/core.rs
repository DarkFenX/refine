#[derive(Debug)]
pub(crate) struct HCoreError {
    error: rc::Error,
}
impl HCoreError {
    pub(crate) fn get_kind(&self) -> &rc::ErrorKind {
        &self.error.kind
    }
    pub(crate) fn get_code(&self) -> String {
        let code = match &self.error.kind {
            rc::ErrorKind::DhHttpInvalidBaseUrl(_, _) => "COR-001",
            rc::ErrorKind::SrcADataGenFailed(_) => "COR-002",
            rc::ErrorKind::FitNotFound(_) => "COR-003",
            rc::ErrorKind::ItemNotFound(_) => "COR-004",
            rc::ErrorKind::SolItemKindNotFound(_) => "COR-005",
            rc::ErrorKind::FitIdAllocFailed => "COR-006",
            rc::ErrorKind::ItemIdAllocFailed => "COR-007",
            rc::ErrorKind::InvalidSkillLevel(_) => "COR-008",
            rc::ErrorKind::UnexpectedItemKind(_, _, _) => "COR-009",
            rc::ErrorKind::ModuleSlotTaken(_, _, _) => "COR-010",
            rc::ErrorKind::AAttrNotFound(_) => "COR-011",
            rc::ErrorKind::AItemNotLoaded(_) => "COR-012",
            rc::ErrorKind::CustomModCalc => "COR-014",
            rc::ErrorKind::ItemNotProjectable(_) => "COR-015",
            rc::ErrorKind::FleetIdAllocFailed => "COR-016",
            rc::ErrorKind::FleetNotFound(_) => "COR-017",
            rc::ErrorKind::ProjecteeNotFound(_, _) => "COR-018",
            rc::ErrorKind::UnremovableItemKind(_) => "COR-019",
            rc::ErrorKind::NotSideEffect(_) => "COR-020",
        };
        code.to_string()
    }
}
impl From<rc::Error> for HCoreError {
    fn from(core_error: rc::Error) -> Self {
        Self { error: core_error }
    }
}
impl std::error::Error for HCoreError {}
impl std::fmt::Display for HCoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "core library error: {}", self.error)
    }
}
