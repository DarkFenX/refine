from pytest import approx


def test_postperc_max(client, consts):
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect1 = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: -40},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_sw_effect2 = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 30},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr.id: 100})
    client.create_sources()
    api_ss = client.create_ss()
    api_ss.add_sw_effect(type_id=eve_sw_effect1.id)
    api_ss.add_sw_effect(type_id=eve_sw_effect2.id)
    api_fit = api_ss.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(130)
