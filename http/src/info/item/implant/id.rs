#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HImplantInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
}
impl From<&rc::ImplantInfo> for HImplantInfoId {
    fn from(core_implant_info: &rc::ImplantInfo) -> Self {
        Self {
            id: core_implant_info.id,
        }
    }
}
