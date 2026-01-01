use crate::svc::vast::{StatTimeOptions, StatTimeOptionsBurst};

pub(super) const CAP_TRANSFER_OPTIONS: StatTimeOptions = StatTimeOptions::Burst(StatTimeOptionsBurst { .. });
