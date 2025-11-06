#[derive(Copy, Clone, Default, serde::Deserialize)]
pub(in crate::cmd) struct HStatOptionCapSim {
    pub(in crate::cmd) cap_perc: Option<rc::AttrVal>,
}
