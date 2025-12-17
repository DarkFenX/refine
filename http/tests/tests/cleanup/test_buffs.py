from fw import approx


def test_affectee_filter_item(client, consts):
    # Ensure that hardcoded buffs defined on hardcoded effects are not cleaned up. Hardcoded buffs
    # on hardcoded effects are used on stability generators (sov upgrade effects)
    eve_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.capacitor_recharge_bonus,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_attr_id)])
    eve_fw_effect_id = client.mk_eve_item(id_=consts.EveItem.electric_stability_generator)
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_fw_effect_id)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    assert api_ship.update().attrs[eve_attr_id].dogma == approx(150)
