from pytest import approx


def test_affector_addition_removal(client, consts):
    # Check that effects are applied/removed when system-wide effect is added/removed
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
    eve_sw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_affectee_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    assert api_ship.update().attrs[eve_affectee_attr.id].dogma == approx(7.5)
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_sw_effect.id)
    assert api_ship.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)
    api_sw_effect.remove()
    assert api_ship.update().attrs[eve_affectee_attr.id].dogma == approx(7.5)


def test_fit_addition_removal(client, consts):
    # Check that effects are applied/removed when fit is added/removed with system-wide effect
    # already being there
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
    eve_sw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_affectee_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_sw_effect.id)
    api_fit1 = api_sol.create_fit()
    api_ship1 = api_fit1.set_ship(type_id=eve_ship.id)
    assert api_ship1.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)
    api_fit2 = api_sol.create_fit()
    api_ship2 = api_fit2.set_ship(type_id=eve_ship.id)
    assert api_ship1.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)
    assert api_ship2.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)
    api_fit2.remove()
    assert api_ship1.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)


def test_affector_state_change(client, consts):
    # Check that effects are applied/removed when system-wide effect is enabled/disabled
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
    eve_sw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_affectee_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_sw_effect.id, state=False)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    assert api_ship.update().attrs[eve_affectee_attr.id].dogma == approx(7.5)
    api_sw_effect.change_sw_effect(state=True)
    assert api_ship.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)
    api_sw_effect.change_sw_effect(state=False)
    assert api_ship.update().attrs[eve_affectee_attr.id].dogma == approx(7.5)
