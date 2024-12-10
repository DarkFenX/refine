from tests import approx


def test_resist_aggr_max(client, consts):
    # Here we test that aggregation chooses value based on post-resist effect strength. There are
    # no such effects in the game - you'd need two different items which apply the same buff, but
    # which define different resistance attribute ID
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_remote_resist_attr_id = client.mk_eve_attr(id_=consts.EveAttr.remote_resistance_id)
    eve_resist_attr1_id = client.mk_eve_attr()
    eve_resist_attr2_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.min,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect1_id = client.mk_eve_item(
        attrs={
            eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: -80,
            eve_remote_resist_attr_id: eve_resist_attr1_id},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_sw_effect2_id = client.mk_eve_item(
        attrs={
            eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: -30,
            eve_remote_resist_attr_id: eve_resist_attr2_id},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 150, eve_resist_attr1_id: 0.1, eve_resist_attr2_id: 0.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_sw_effect1_id)
    api_sw_effect2 = api_sol.add_sw_effect(type_id=eve_sw_effect2_id)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_ship.update()
    # First system-wide buff is stronger before resists, second is stronger after, second is applied
    assert api_ship.attrs[eve_affectee_attr_id].dogma == approx(127.5)
    api_mod = api_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(-30)
    assert api_mod.resist_mult == approx(0.5)
    assert api_mod.applied_val == approx(-15)
    assert api_mod.affectors.one().item_id == api_sw_effect2.id
    assert api_mod.affectors.one().attr_id == eve_buff_val_attr_id
