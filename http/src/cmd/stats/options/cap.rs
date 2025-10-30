#[derive(Copy, Clone, Default, serde::Deserialize)]
pub(in crate::cmd) struct HStatOptionCapRegen {
    pub(in crate::cmd) cap_perc: Option<rc::AttrVal>,
}
