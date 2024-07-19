from tests import approx


def test_attr1(client, consts):
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_fw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_fit.add_fw_effect(type_id=eve_fw_effect.id)
    assert api_ship.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)


def test_attr2(client, consts):
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_2_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_2_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_fw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_fit.add_fw_effect(type_id=eve_fw_effect.id)
    assert api_ship.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)


def test_attr3(client, consts):
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_3_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_3_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_fw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_fit.add_fw_effect(type_id=eve_fw_effect.id)
    assert api_ship.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)


def test_attr4(client, consts):
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_4_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_4_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_fw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_fit.add_fw_effect(type_id=eve_fw_effect.id)
    assert api_ship.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)


def test_multiple(client, consts):
    eve_buff_type_attr1 = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr1 = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_buff_type_attr2 = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_3_id)
    eve_buff_val_attr2 = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_3_value)
    eve_affectee_attr1 = client.mk_eve_attr()
    eve_affectee_attr2 = client.mk_eve_attr()
    eve_buff1 = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr1.id)])
    eve_buff2 = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr2.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_fw_effect = client.mk_eve_item(
        attrs={
            eve_buff_type_attr1.id: eve_buff1.id, eve_buff_val_attr1.id: 5,
            eve_buff_type_attr2.id: eve_buff2.id, eve_buff_val_attr2.id: 10},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_ship(attrs={eve_affectee_attr1.id: 7.5, eve_affectee_attr2.id: 10})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_fit.add_fw_effect(type_id=eve_fw_effect.id)
    api_ship.update()
    assert api_ship.attrs[eve_affectee_attr1.id].dogma == approx(37.5)
    assert api_ship.attrs[eve_affectee_attr2.id].dogma == approx(100)
