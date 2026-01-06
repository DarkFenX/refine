use super::DependencyRegister;
use crate::{
    dbg::{DebugResult, check_attr_rid, check_effect_rid, check_item_uid},
    ud::UData,
};

impl DependencyRegister {
    pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        for (affector_spec, affector_data) in self.data.iter() {
            check_item_uid(u_data, affector_spec.item_uid, true)?;
            check_attr_rid(u_data, affector_spec.attr_rid)?;
            for affectee_spec in affector_data {
                check_item_uid(u_data, affectee_spec.item_uid, true)?;
                check_attr_rid(u_data, affectee_spec.attr_rid)?;
            }
        }
        for (item_uid, item_data) in self.anonymous_by_item.iter() {
            check_item_uid(u_data, *item_uid, true)?;
            for (attr1_rid, attr2_rid) in item_data {
                check_attr_rid(u_data, *attr1_rid)?;
                check_attr_rid(u_data, *attr2_rid)?;
            }
        }
        for (source, specs) in self.by_source.iter() {
            check_item_uid(u_data, source.item_uid, true)?;
            check_effect_rid(u_data, source.effect_rid)?;
            for (affector_spec, affectee_spec) in specs {
                check_item_uid(u_data, affector_spec.item_uid, true)?;
                check_attr_rid(u_data, affector_spec.attr_rid)?;
                check_item_uid(u_data, affectee_spec.item_uid, true)?;
                check_attr_rid(u_data, affectee_spec.attr_rid)?;
            }
        }
        for (&item_uid, sources) in self.source_by_item.iter() {
            check_item_uid(u_data, item_uid, true)?;
            for source in sources {
                check_item_uid(u_data, source.item_uid, true)?;
                check_effect_rid(u_data, source.effect_rid)?;
            }
        }
        Ok(())
    }
}
