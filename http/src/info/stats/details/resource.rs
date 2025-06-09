#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HStatRes {
    used: rc::AttrVal,
    output: Option<rc::AttrVal>,
}
impl From<rc::stats::StatRes> for HStatRes {
    fn from(core_stat: rc::stats::StatRes) -> Self {
        Self {
            used: core_stat.used,
            output: core_stat.output,
        }
    }
}
