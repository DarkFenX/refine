use crate::stats::StatTimeOptions;

#[derive(Copy, Clone, Default)]
pub struct StatOptionIncomingJam {
    pub time_options: StatTimeOptions = StatTimeOptions::default(),
}
