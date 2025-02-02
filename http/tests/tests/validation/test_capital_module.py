from tests import check_no_field


def test_main(client, consts):
    eve_skill_id = client.mk_eve_item(id_=consts.EveItem.capital_ships)
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.hi_power)
    eve_cap_module_id = client.mk_eve_item(attrs={eve_attr_id: 4000}, eff_ids=[eve_effect_id])
    eve_subcap_module_id = client.mk_eve_item(attrs={eve_attr_id: 3500}, eff_ids=[eve_effect_id])
    eve_subcap_ship_id = client.mk_eve_ship()
    eve_cap_ship_id = client.mk_eve_ship(srqs={eve_skill_id: 1})
    eve_struct_id = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_subcap_ship_id)
    api_cap_module = api_fit.add_mod(type_id=eve_cap_module_id)
    api_fit.add_mod(type_id=eve_subcap_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.capital_module])
    assert api_val.passed is False
    assert api_val.details.capital_module == [api_cap_module.id]
    # Action
    api_fit.set_ship(type_id=eve_cap_ship_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.capital_module])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.capital_module])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_subcap_ship_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.capital_module])
    assert api_val.passed is False
    assert api_val.details.capital_module == [api_cap_module.id]
