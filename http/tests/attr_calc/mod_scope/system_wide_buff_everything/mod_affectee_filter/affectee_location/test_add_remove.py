from tests import approx


def test_add_sw_fit_item_remove_sw_item_fit(client, consts):
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_ship()
    eve_module = client.mk_eve_item(attrs={eve_affectee_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_sw_effect.id)
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship.id)
    api_module = api_fit.add_mod(type_id=eve_module.id)
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)
    api_sw_effect.remove()
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(7.5)
    api_module.remove()
    api_module.update(status_code=404)
    api_fit.remove()


def test_add_fit_sw_item_remove_item_sw_fit(client, consts):
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_ship()
    eve_module = client.mk_eve_item(attrs={eve_affectee_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship.id)
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_sw_effect.id)
    api_module = api_fit.add_mod(type_id=eve_module.id)
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)
    api_module.remove()
    api_module.update(status_code=404)
    api_sw_effect.remove()
    api_fit.remove()


def test_add_fit_item_sw_remove_fit_sw(client, consts):
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_ship()
    eve_module = client.mk_eve_item(attrs={eve_affectee_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship.id)
    api_module = api_fit.add_mod(type_id=eve_module.id)
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(7.5)
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_sw_effect.id)
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)
    api_fit.remove()
    api_module.update(status_code=404)
    api_sw_effect.remove()


def test_add_sw_fit_item_state_remove_state_item_fit_sw(client, consts):
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_ship()
    eve_module = client.mk_eve_item(attrs={eve_affectee_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_sw_effect.id, state=False)
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship.id)
    api_module = api_fit.add_mod(type_id=eve_module.id)
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(7.5)
    api_sw_effect.change_sw_effect(state=True)
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)
    api_sw_effect.change_sw_effect(state=False)
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(7.5)
    api_module.remove()
    api_module.update(status_code=404)
    api_fit.remove()
    api_sw_effect.remove()
