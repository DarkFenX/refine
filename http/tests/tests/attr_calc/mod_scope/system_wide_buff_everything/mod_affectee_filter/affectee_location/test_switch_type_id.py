from tests import approx, check_no_field


def test_affected_child_module(client, consts):
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship()
    eve_module1_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 7.5})
    eve_module2_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 5})
    eve_module3_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module1_id)
    api_sol.add_sw_effect(type_id=eve_sw_effect_id)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_module.change_module(type_id=eve_module2_id)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(25)
    # Action
    api_module.change_module(type_id=eve_module3_id)
    # Verification
    api_module.update()
    with check_no_field():
        api_module.attrs  # noqa: B018
    # Action
    api_module.change_module(type_id=eve_module1_id)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
