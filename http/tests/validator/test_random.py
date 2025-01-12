

def test_random(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu_output)
    eve_effect_id = client.mk_eve_online_effect()
    eve_module1_id = client.mk_eve_item(attrs={eve_use_attr_id: 50}, eff_ids=[eve_effect_id])
    eve_module2_id = client.mk_eve_item(attrs={eve_use_attr_id: 100}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_mod1 = api_fit.add_mod(type_id=eve_module1_id, state=consts.ApiState.online)
    api_mod2 = api_fit.add_mod(type_id=eve_module2_id, state=consts.ApiState.online)
    api_val = api_fit.validate()
    assert api_val.passed is False
    assert api_val.details.cpu.used == 150
    assert api_val.details.cpu.output == 125
    assert api_val.details.cpu.users[api_mod1.id] == 50
    assert api_val.details.cpu.users[api_mod2.id] == 100
