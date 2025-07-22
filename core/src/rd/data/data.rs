use crate::{
    ad,
    rd::{RAttr, REntityContainer},
};

pub(crate) struct RData {
    attrs: REntityContainer<ad::AAttrId, RAttr>,
}
