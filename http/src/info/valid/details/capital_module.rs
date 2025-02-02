#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(in crate::info::valid) struct HCapitalModValFail {
    #[serde(flatten)]
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    data: Vec<rc::SolItemId>,
}
impl HCapitalModValFail {
    pub(in crate::info::valid) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
impl From<&Vec<rc::SolCapitalModValFail>> for HCapitalModValFail {
    fn from(core_val_fails: &Vec<rc::SolCapitalModValFail>) -> Self {
        Self {
            data: core_val_fails.iter().map(|v| v.item_id).collect(),
        }
    }
}
