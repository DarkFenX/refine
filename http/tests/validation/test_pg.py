from tests import check_no_field


def test_fail_single(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_output)
    eve_effect_id = client.mk_eve_online_effect()
    eve_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 150}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.pg])
    assert api_val.passed is False
    assert api_val.details.pg.used == 150
    assert api_val.details.pg.output == 125
    assert len(api_val.details.pg.users) == 1
    assert api_val.details.pg.users[api_module.id] == 150


def test_fail_multiple(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_output)
    eve_effect_id = client.mk_eve_online_effect()
    eve_module1_id = client.mk_eve_item(attrs={eve_use_attr_id: 50}, eff_ids=[eve_effect_id])
    eve_module2_id = client.mk_eve_item(attrs={eve_use_attr_id: 100}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_mod(type_id=eve_module1_id, state=consts.ApiState.online)
    api_module2 = api_fit.add_mod(type_id=eve_module2_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.pg])
    assert api_val.passed is False
    assert api_val.details.pg.used == 150
    assert api_val.details.pg.output == 125
    assert len(api_val.details.pg.users) == 2
    assert api_val.details.pg.users[api_module1.id] == 50
    assert api_val.details.pg.users[api_module2.id] == 100


def test_modified_use(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_output)
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
    api_module = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.pg])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.pg])
    assert api_val.passed is False
    assert api_val.details.pg.used == 150
    assert api_val.details.pg.output == 125
    assert len(api_val.details.pg.users) == 1
    assert api_val.details.pg.users[api_module.id] == 150


def test_modified_output(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_output)
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
    api_module = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.pg])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.pg])
    assert api_val.passed is False
    assert api_val.details.pg.used == 150
    assert api_val.details.pg.output == 100
    assert len(api_val.details.pg.users) == 1
    assert api_val.details.pg.users[api_module.id] == 150


def test_sum_rounding(client, consts):
    # Individual PG attribute values are rounded to 2nd decimal digit; check that total sum of users
    # is rounded; if there would be no rounding, one of sums of 0.1 elements would lead to float
    # inaccuracies
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_output)
    eve_effect_id = client.mk_eve_online_effect()
    eve_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 0.1}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 0.15})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    for i in range(1, 21):
        # Action
        api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.online)
        if i == 1:
            continue
        # Verification
        api_val = api_fit.validate(include=[consts.ApiValType.pg])
        assert api_val.passed is False
        assert api_val.details.pg.used == round(i / 10, 1)
        assert api_val.details.pg.output == 0.15
        assert len(api_val.details.pg.users) == i


def test_no_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_output)
    eve_effect_id = client.mk_eve_online_effect()
    eve_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 5}, eff_ids=[eve_effect_id])
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_output_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.pg])
    assert api_val.passed is False
    assert api_val.details.pg.used == 5
    assert api_val.details.pg.output is None
    assert len(api_val.details.pg.users) == 1
    assert api_val.details.pg.users[api_module.id] == 5


def test_unloaded_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_output)
    eve_effect_id = client.mk_eve_online_effect()
    eve_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 5}, eff_ids=[eve_effect_id])
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_output_attr_id: 5})
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.pg])
    assert api_val.passed is False
    assert api_val.details.pg.used == 5
    assert api_val.details.pg.output is None
    assert len(api_val.details.pg.users) == 1
    assert api_val.details.pg.users[api_module.id] == 5


def test_unloaded_user(client, consts):
    # Just check that nothing crashes, unloaded items are not supposed to even be registered
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_output)
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_use_attr_id: 5})
    eve_module_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.pg])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_non_positive(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_output)
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
    api_module2 = api_fit.add_mod(type_id=eve_module2_id, state=consts.ApiState.online)
    api_fit.add_mod(type_id=eve_module3_id, state=consts.ApiState.online)
    # Verification - items with negative and 0 use are not exposed
    api_val = api_fit.validate(include=[consts.ApiValType.pg])
    assert api_val.passed is False
    assert api_val.details.pg.used == 140
    assert api_val.details.pg.output == 125
    assert len(api_val.details.pg.users) == 1
    assert api_val.details.pg.users[api_module2.id] == 150


def test_no_attr_use(client, consts):
    eve_use_attr_id = consts.EveAttr.power
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_output)
    eve_effect_id = client.mk_eve_online_effect()
    eve_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 150}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.pg])
    assert api_val.passed is True


def test_no_attr_output(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    eve_output_attr_id = consts.EveAttr.power_output
    eve_effect_id = client.mk_eve_online_effect()
    eve_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 150}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.pg])
    assert api_val.passed is False
    assert api_val.details.pg.used == 150
    assert api_val.details.pg.output is None
    assert len(api_val.details.pg.users) == 1
    assert api_val.details.pg.users[api_module.id] == 150


def test_criterion_state(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_output)
    eve_effect_id = client.mk_eve_online_effect()
    eve_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 150}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.offline)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.pg])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_module.change_mod(state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.pg])
    assert api_val.passed is False
    assert api_val.details.pg.used == 150
    assert api_val.details.pg.output == 125
    assert len(api_val.details.pg.users) == 1
    assert api_val.details.pg.users[api_module.id] == 150
    # Action
    api_module.change_mod(state=consts.ApiState.offline)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.pg])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_criterion_effect(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_output)
    eve_effect_id = client.mk_eve_online_effect()
    eve_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 150}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.pg])
    assert api_val.passed is False
    assert api_val.details.pg.used == 150
    assert api_val.details.pg.output == 125
    assert len(api_val.details.pg.users) == 1
    assert api_val.details.pg.users[api_module.id] == 150
    # Action
    api_module.change_mod(effect_modes={eve_effect_id: consts.ApiEffMode.force_stop})
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.pg])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_module.change_mod(state=consts.ApiState.online, effect_modes={eve_effect_id: consts.ApiEffMode.full_compliance})
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.pg])
    assert api_val.passed is False
    assert api_val.details.pg.used == 150
    assert api_val.details.pg.output == 125
    assert len(api_val.details.pg.users) == 1
    assert api_val.details.pg.users[api_module.id] == 150
    # Action
    api_module.change_mod(effect_modes={eve_effect_id: consts.ApiEffMode.force_stop})
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.pg])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_criterion_module(client, consts):
    # Validation applies only to modules
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_output)
    eve_effect_id = client.mk_eve_online_effect()
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 150}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.pg])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
