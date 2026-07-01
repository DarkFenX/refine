use std::collections::hash_map::Entry;

use crate::{ad::ADataGenerator, nd::N_BUFF_MAP};

impl ADataGenerator {
    pub(super) fn customize_buffs(&mut self) {
        for n_buff in N_BUFF_MAP.values() {
            if let Some(buff_maker) = n_buff.adg_make_buff_fn {
                let a_buff = buff_maker();
                match self.a_data.buffs.data.entry(a_buff.id) {
                    Entry::Occupied(_) => {
                        tracing::info!("buff {}: already exists, not replacing", a_buff.id);
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(a_buff);
                    }
                }
            }
        }
    }
}
