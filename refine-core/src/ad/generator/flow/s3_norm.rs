use crate::{
    ad::{
        ADataGenerator,
        generator::rels::{KeyPart, Pk},
    },
    ed::{EAttrId, EDataCont, EFloat, EItemAttr, EItemId},
    util::RSet,
};

impl ADataGenerator {
    pub(in crate::ad::generator) fn normalize(&mut self) {
        self.move_basic_attrs();
    }
    fn move_basic_attrs(&mut self) {
        let mut seen_pks = RSet::new();
        for item_attr in self.e_data.item_attrs.data.iter() {
            let pk = item_attr.get_pk();
            seen_pks.insert(pk);
        }
        let attr_eids = self.e_data.attrs.data.iter().map(|v| v.id).collect();
        for item in self.e_data.items.data.iter() {
            move_basic_attr(
                item.id,
                EAttrId::CAPACITY,
                item.capacity,
                &mut self.e_data.item_attrs,
                &attr_eids,
                &seen_pks,
            );
            move_basic_attr(
                item.id,
                EAttrId::MASS,
                item.mass,
                &mut self.e_data.item_attrs,
                &attr_eids,
                &seen_pks,
            );
            move_basic_attr(
                item.id,
                EAttrId::RADIUS,
                item.radius,
                &mut self.e_data.item_attrs,
                &attr_eids,
                &seen_pks,
            );
            move_basic_attr(
                item.id,
                EAttrId::VOLUME,
                item.volume,
                &mut self.e_data.item_attrs,
                &attr_eids,
                &seen_pks,
            );
        }
    }
}

fn move_basic_attr(
    item_id: EItemId,
    attr_id: EAttrId,
    basic_value: EFloat,
    e_data_item_attrs: &mut EDataCont<EItemAttr>,
    attr_ids: &RSet<EAttrId>,
    seen_pks: &RSet<Vec<KeyPart>>,
) {
    if !attr_ids.contains(&attr_id) {
        return;
    }
    let item_attr = EItemAttr {
        item_id,
        attr_id,
        value: basic_value,
    };
    let pk = item_attr.get_pk();
    if !seen_pks.contains(&pk) {
        e_data_item_attrs.data.push(item_attr)
    }
}
