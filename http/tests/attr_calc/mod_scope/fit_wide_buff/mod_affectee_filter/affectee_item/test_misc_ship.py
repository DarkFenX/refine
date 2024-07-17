from tests import approx


def test_replace_root(client, consts):
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.mod_titan_effect_generator, cat_id=consts.EveEffCat.active)
    eve_fw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 30},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship1 = client.mk_eve_ship(attrs={eve_affectee_attr.id: 200})
    eve_ship2 = client.mk_eve_ship(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship1 = api_fit.set_ship(type_id=eve_ship1.id)
    api_fit.add_fw_effect(type_id=eve_fw_effect.id)
    assert api_ship1.update().attrs[eve_affectee_attr.id].dogma == approx(260)
    api_ship2 = api_fit.set_ship(type_id=eve_ship2.id)
    assert api_ship2.update().attrs[eve_affectee_attr.id].dogma == approx(130)
