use crate::svc::calc::modifier::raw_mod::RawModifier;

pub(in crate::svc::calc::modifier) enum RawModTemplateStatus {
    Template(RawModifier),
    NoModifier,
    ItemDependent,
}
