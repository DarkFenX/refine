from pytest import approx


def test_self_state_switch(client, consts):
    # Check that fleet effects are applied/removed when module carrying them changes state
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_mod = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_mod = api_fit.add_mod(type_id=eve_mod.id, state=consts.ApiState.online)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    api_mod.change_mod(state=consts.ApiState.active)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    api_mod.change_mod(state=consts.ApiState.online)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_self_fleet_add_remove(client, consts):
    # Check that fleet effects stay even after a fit has been removed from a fleet
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_mod = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5})
    client.create_sources()
    api_ss = client.create_ss()
    api_fleet = api_ss.create_fleet()
    api_fit = api_ss.create_fit()
    api_fit.set_fleet(fleet_id=api_fleet.id)
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_fit.add_mod(type_id=eve_mod.id, state=consts.ApiState.active)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    api_fit.set_fleet(fleet_id=None)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
