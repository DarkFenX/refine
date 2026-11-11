use crate::ad::generator::{
    AdgSupport,
    rels::{ItemFk, KeyPart},
};

pub(in crate::ad::generator) trait Pk {
    fn get_pk(&self) -> Vec<KeyPart>;
}

pub(in crate::ad::generator) trait Fk {
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Items
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn iter_item_fks_all(&self, adg_supp: &AdgSupport) -> impl Iterator<Item = ItemFk> {
        self.get_item_fk_self_restoring()
            .into_iter()
            .chain(self.get_item_fks_regular(adg_supp))
    }
    // Non-restoring FKs
    fn get_item_fks_regular(&self, _: &AdgSupport) -> Vec<ItemFk> {
        Vec::new()
    }
    // Cleanup restores the row with the FK when target of this key is alive and meets its condition
    fn get_item_fk_self_restoring(&self) -> Option<ItemFk> {
        None
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Other entities
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn get_item_list_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        Vec::new()
    }
    fn get_attr_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        Vec::new()
    }
    fn get_effect_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        Vec::new()
    }
    fn get_abil_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        Vec::new()
    }
    fn get_buff_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        Vec::new()
    }
}
