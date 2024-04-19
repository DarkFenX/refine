use crate::{
    ss::{
        svc::debug::{check_attr, check_item},
        SsView,
    },
    util::DebugResult,
};

use super::SsDependencyRegister;

impl SsDependencyRegister {
    pub(in crate::ss) fn debug_consistency_check(&self, ss_view: &SsView) -> DebugResult {
        for (src_attr_spec, tgt_attr_specs) in self.data.iter() {
            check_item(ss_view, &src_attr_spec.item_id)?;
            check_attr(ss_view, &src_attr_spec.attr_id)?;
            for tgt_attr_spec in tgt_attr_specs {
                check_item(ss_view, &tgt_attr_spec.item_id)?;
                check_attr(ss_view, &tgt_attr_spec.attr_id)?;
            }
        }
        for (item_id, attr_specs) in self.item_src_map.iter() {
            check_item(ss_view, item_id)?;
            for attr_spec in attr_specs {
                check_item(ss_view, &attr_spec.item_id)?;
                check_attr(ss_view, &attr_spec.attr_id)?;
            }
        }
        for (item_id, spec_map) in self.item_tgt_map.iter() {
            check_item(ss_view, item_id)?;
            for (src_attr_spec, tgt_attr_specs) in spec_map.iter() {
                check_item(ss_view, &src_attr_spec.item_id)?;
                check_attr(ss_view, &src_attr_spec.attr_id)?;
                for tgt_attr_spec in tgt_attr_specs {
                    check_item(ss_view, &tgt_attr_spec.item_id)?;
                    check_attr(ss_view, &tgt_attr_spec.attr_id)?;
                }
            }
        }
        Ok(())
    }
}
