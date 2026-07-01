use std::collections::hash_map::Entry;

use crate::{ad::ADataGenerator, nd::N_ATTR_MAP};

impl ADataGenerator {
    pub(super) fn customize_attrs(&mut self) {
        for n_attr in N_ATTR_MAP.values() {
            if let Some(attr_maker) = n_attr.adg_make_attr_fn {
                let a_attr = attr_maker();
                match self.a_data.attrs.data.entry(a_attr.id) {
                    Entry::Occupied(_) => {
                        tracing::info!("attr {}: already exists, not replacing", a_attr.id);
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(a_attr);
                    }
                }
            }
        }
    }
}
