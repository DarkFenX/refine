from pytest import approx


def test_affected_modules(client, consts):
    # Make sure ship items (such as modules) are affected by location-filtered buff modification
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item()
    eve_module = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit1 = api_ss.create_fit()
    api_fit1.set_ship(type_id=eve_ship.id)
    api_module1 = api_fit1.add_mod(type_id=eve_module.id)
    eve_sw_effect = api_ss.add_sw_effect(type_id=eve_sw_effect.id)
    api_fit2 = api_ss.create_fit()
    api_fit2.set_ship(type_id=eve_ship.id)
    api_module2 = api_fit1.add_mod(type_id=eve_module.id)
    assert api_module1.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    assert api_module2.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    eve_sw_effect.remove()
    assert api_module1.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    assert api_module2.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
