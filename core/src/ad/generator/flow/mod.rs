pub(in crate::ad::generator) use s1_fetch::fetch_data;
pub(in crate::ad::generator) use s2_pk::dedup_pks;
pub(in crate::ad::generator) use s3_norm::normalize;
pub(in crate::ad::generator) use s4_clean::clean_unused;
pub(in crate::ad::generator) use s5_valid::validate;
pub(in crate::ad::generator) use s6_conv_pre::convert_pre;
pub(in crate::ad::generator) use s7_custom::customize;
pub(in crate::ad::generator) use s8_conv_post::convert_post;

mod s1_fetch;
mod s2_pk;
mod s3_norm;
mod s4_clean;
mod s5_valid;
mod s6_conv_pre;
mod s7_custom;
mod s8_conv_post;
