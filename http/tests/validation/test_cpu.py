from tests import check_no_field


def test_fail_single(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu_output)
    eve_effect_id = client.mk_eve_online_effect()
    eve_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 150}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_mod = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.cpu])
    assert api_val.passed is False
    assert api_val.details.cpu.used == 150
    assert api_val.details.cpu.output == 125
    assert len(api_val.details.cpu.users) == 1
    assert api_val.details.cpu.users[api_mod.id] == 150


def test_fail_multiple(client, consts):
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
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.cpu])
    assert api_val.passed is False
    assert api_val.details.cpu.used == 150
    assert api_val.details.cpu.output == 125
    assert len(api_val.details.cpu.users) == 2
    assert api_val.details.cpu.users[api_mod1.id] == 50
    assert api_val.details.cpu.users[api_mod2.id] == 100


def test_modified_use(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu_output)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_use_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_online_effect_id = client.mk_eve_online_effect()
    eve_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 100},eff_ids=[eve_online_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 50},eff_ids=[eve_mod_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_mod = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.cpu])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.cpu])
    assert api_val.passed is False
    assert api_val.details.cpu.used == 150
    assert api_val.details.cpu.output == 125
    assert len(api_val.details.cpu.users) == 1
    assert api_val.details.cpu.users[api_mod.id] == 150


def test_modified_output(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu_output)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_output_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_online_effect_id = client.mk_eve_online_effect()
    eve_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 150},eff_ids=[eve_online_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 200})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: -50},eff_ids=[eve_mod_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_mod = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.cpu])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.cpu])
    assert api_val.passed is False
    assert api_val.details.cpu.used == 150
    assert api_val.details.cpu.output == 100
    assert len(api_val.details.cpu.users) == 1
    assert api_val.details.cpu.users[api_mod.id] == 150


def test_no_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu)
    client.mk_eve_attr(id_=consts.EveAttr.cpu_output)
    eve_effect_id = client.mk_eve_online_effect()
    eve_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 5}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_mod = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.cpu])
    assert api_val.passed is False
    assert api_val.details.cpu.used == 5
    assert api_val.details.cpu.output == 0
    assert len(api_val.details.cpu.users) == 1
    assert api_val.details.cpu.users[api_mod.id] == 5


def test_unloaded_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu)
    client.mk_eve_attr(id_=consts.EveAttr.cpu_output)
    eve_effect_id = client.mk_eve_online_effect()
    eve_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 5}, eff_ids=[eve_effect_id])
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_mod = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.cpu])
    assert api_val.passed is False
    assert api_val.details.cpu.used == 5
    assert api_val.details.cpu.output == 0
    assert len(api_val.details.cpu.users) == 1
    assert api_val.details.cpu.users[api_mod.id] == 5


def test_non_positive(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu_output)
    eve_effect_id = client.mk_eve_online_effect()
    eve_module1_id = client.mk_eve_item(attrs={eve_use_attr_id: 0}, eff_ids=[eve_effect_id])
    eve_module2_id = client.mk_eve_item(attrs={eve_use_attr_id: 150}, eff_ids=[eve_effect_id])
    eve_module3_id = client.mk_eve_item(attrs={eve_use_attr_id: -10}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_mod(type_id=eve_module1_id, state=consts.ApiState.online)
    api_mod2 = api_fit.add_mod(type_id=eve_module2_id, state=consts.ApiState.online)
    api_fit.add_mod(type_id=eve_module3_id, state=consts.ApiState.online)
    # Verification - items with negative and 0 use are not exposed
    api_val = api_fit.validate(include=[consts.ApiValType.cpu])
    assert api_val.passed is False
    assert api_val.details.cpu.used == 140
    assert api_val.details.cpu.output == 125
    assert len(api_val.details.cpu.users) == 1
    assert api_val.details.cpu.users[api_mod2.id] == 150
