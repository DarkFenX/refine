from pytest import approx


def test_src_addition_removal(client, consts):
    # Check that effects are applied/removed when fit-wide effect is added/removed
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_fw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect.id)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    api_fw_effect.remove()
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_src_state_change(client, consts):
    # Check that effects are applied/removed when fit-wide effect is enabled/disabled
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_fw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect.id, state=False)
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    api_fw_effect.change_fw_effect(state=True)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    api_fw_effect.change_fw_effect(state=False)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
