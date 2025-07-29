from tests import Muta, check_no_field
from tests.fw.api import FitStatsOptions, ValOptions


def test_fail_single(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu_output)
    eve_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.online)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
    assert api_stats.cpu == (150, 125)
    api_val = api_fit.validate(options=ValOptions(cpu=True))
    assert api_val.passed is False
    assert api_val.details.cpu.used == 150
    assert api_val.details.cpu.max == 125
    assert api_val.details.cpu.users == {api_module.id: 150}


def test_fail_multiple_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu_output)
    eve_module1_id = client.mk_eve_item(attrs={eve_use_attr_id: 50})
    eve_module2_id = client.mk_eve_item(attrs={eve_use_attr_id: 100})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.online)
    api_module2 = api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.online)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
    assert api_stats.cpu == (150, 125)
    api_val = api_fit.validate(options=ValOptions(cpu=True))
    assert api_val.passed is False
    assert api_val.details.cpu.used == 150
    assert api_val.details.cpu.max == 125
    assert api_val.details.cpu.users == {api_module1.id: 50, api_module2.id: 100}


def test_fail_multiple_struct(client, consts):
    # Test service CPU use as well
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu_output)
    eve_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 50})
    eve_service_id = client.mk_eve_item(attrs={eve_use_attr_id: 100})
    eve_struct_id = client.mk_eve_struct(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.online)
    api_service = api_fit.add_service(type_id=eve_service_id, state=consts.ApiServiceState.online)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
    assert api_stats.cpu == (150, 125)
    api_val = api_fit.validate(options=ValOptions(cpu=True))
    assert api_val.passed is False
    assert api_val.details.cpu.used == 150
    assert api_val.details.cpu.max == 125
    assert api_val.details.cpu.users == {api_module.id: 50, api_service.id: 100}


def test_equal(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu_output)
    eve_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 150})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.online)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
    assert api_stats.cpu == (150, 150)
    api_val = api_fit.validate(options=ValOptions(cpu=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_known_failures(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu_output)
    eve_module1_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_module2_id = client.mk_eve_item(attrs={eve_use_attr_id: 100})
    eve_module3_id = client.mk_eve_item(attrs={eve_use_attr_id: -10})
    eve_module4_id = client.mk_eve_item(attrs={eve_use_attr_id: 0})
    eve_module5_id = client.mk_eve_item(attrs={eve_use_attr_id: 0.5})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_implant(type_id=eve_other_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.online)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
    assert api_stats.cpu == (150, 125)
    api_val = api_fit.validate(options=ValOptions(cpu=(True, [api_module1.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module2 = api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.online)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
    assert api_stats.cpu == (250, 125)
    api_val = api_fit.validate(options=ValOptions(cpu=(True, [api_module1.id])))
    assert api_val.passed is False
    assert api_val.details.cpu.used == 250
    assert api_val.details.cpu.max == 125
    assert api_val.details.cpu.users == {api_module2.id: 100}
    api_val = api_fit.validate(options=ValOptions(cpu=(True, [api_module2.id])))
    assert api_val.passed is False
    assert api_val.details.cpu.used == 250
    assert api_val.details.cpu.max == 125
    assert api_val.details.cpu.users == {api_module1.id: 150}
    api_val = api_fit.validate(options=ValOptions(cpu=(True, [api_module1.id, api_module2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(cpu=(True, [api_module1.id, api_other.id, api_module2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module3 = api_fit.add_module(type_id=eve_module3_id, state=consts.ApiModuleState.online)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
    assert api_stats.cpu == (240, 125)
    api_val = api_fit.validate(options=ValOptions(cpu=(True, [api_module1.id, api_module2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module3.remove()
    api_module4 = api_fit.add_module(type_id=eve_module4_id, state=consts.ApiModuleState.online)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
    assert api_stats.cpu == (250, 125)
    api_val = api_fit.validate(options=ValOptions(cpu=(True, [api_module1.id, api_module2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module4.remove()
    api_module5 = api_fit.add_module(type_id=eve_module5_id, state=consts.ApiModuleState.online)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
    assert api_stats.cpu == (250.5, 125)
    api_val = api_fit.validate(options=ValOptions(cpu=(True, [api_module1.id, api_module2.id])))
    assert api_val.passed is False
    assert api_val.details.cpu.used == 250.5
    assert api_val.details.cpu.max == 125
    assert api_val.details.cpu.users == {api_module5.id: 0.5}


def test_modified_use(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu_output)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_use_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: -50}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.online)
    # Verification
    assert api_module.update().attrs[eve_use_attr_id].extra == 150
    api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
    assert api_stats.cpu == (150, 125)
    api_val = api_fit.validate(options=ValOptions(cpu=True))
    assert api_val.passed is False
    assert api_val.details.cpu.used == 150
    assert api_val.details.cpu.max == 125
    assert api_val.details.cpu.users == {api_module.id: 150}
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    assert api_module.update().attrs[eve_use_attr_id].extra == 75
    api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
    assert api_stats.cpu == (75, 125)
    api_val = api_fit.validate(options=ValOptions(cpu=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_modified_max(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu_output)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_max_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 120})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 50}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.online)
    # Verification
    assert api_ship.update().attrs[eve_max_attr_id].extra == 120
    api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
    assert api_stats.cpu == (150, 120)
    api_val = api_fit.validate(options=ValOptions(cpu=True))
    assert api_val.passed is False
    assert api_val.details.cpu.used == 150
    assert api_val.details.cpu.max == 120
    assert api_val.details.cpu.users == {api_module.id: 150}
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    assert api_ship.update().attrs[eve_max_attr_id].extra == 180
    api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
    assert api_stats.cpu == (150, 180)
    api_val = api_fit.validate(options=ValOptions(cpu=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_mutation_use(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu_output)
    eve_base_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 120})
    eve_mutated_module_id = client.mk_eve_item()
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_module_id], eve_mutated_module_id)],
        attrs={eve_use_attr_id: (0.8, 1.2)})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_base_module_id, state=consts.ApiModuleState.online)
    # Verification
    assert api_module.update().attrs[eve_use_attr_id].extra == 120
    api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
    assert api_stats.cpu == (120, 125)
    api_val = api_fit.validate(options=ValOptions(cpu=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(mutation=(eve_mutator_id, {eve_use_attr_id: Muta.roll_to_api(val=0.7)}))
    # Verification
    assert api_module.update().attrs[eve_use_attr_id].extra == 129.6
    api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
    assert api_stats.cpu == (129.6, 125)
    api_val = api_fit.validate(options=ValOptions(cpu=True))
    assert api_val.passed is False
    assert api_val.details.cpu.used == 129.6
    assert api_val.details.cpu.max == 125
    assert api_val.details.cpu.users == {api_module.id: 129.6}
    # Action
    api_module.change_module(mutation={eve_use_attr_id: Muta.roll_to_api(val=0.8)})
    # Verification
    assert api_module.update().attrs[eve_use_attr_id].extra == 134.4
    api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
    assert api_stats.cpu == (134.4, 125)
    api_val = api_fit.validate(options=ValOptions(cpu=True))
    assert api_val.passed is False
    assert api_val.details.cpu.used == 134.4
    assert api_val.details.cpu.max == 125
    assert api_val.details.cpu.users == {api_module.id: 134.4}
    # Action
    api_module.change_module(mutation=None)
    # Verification
    assert api_module.update().attrs[eve_use_attr_id].extra == 120
    api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
    assert api_stats.cpu == (120, 125)
    api_val = api_fit.validate(options=ValOptions(cpu=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_rounding(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu_output)
    eve_module1_id = client.mk_eve_item(attrs={eve_use_attr_id: 0.006})
    eve_module2_id = client.mk_eve_item(attrs={eve_use_attr_id: 5.227})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 5.234})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.online)
    api_module2 = api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.online)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
    assert api_stats.cpu == (5.24, 5.23)
    api_val = api_fit.validate(options=ValOptions(cpu=True))
    assert api_val.passed is False
    assert api_val.details.cpu.used == 5.24
    assert api_val.details.cpu.max == 5.23
    assert api_val.details.cpu.users == {api_module1.id: 0.01, api_module2.id: 5.23}


def test_sum_rounding(client, consts):
    # Individual CPU attribute values are rounded to 2nd decimal digit; check that total sum of
    # users is rounded; if there would be no rounding, one of sums of 0.1 elements would lead to
    # float inaccuracies
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu_output)
    eve_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 0.1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0.15})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    for i in range(1, 21):
        # Action
        api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.online)
        if i == 1:
            continue
        # Verification
        api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
        assert api_stats.cpu == (round(i / 10, 1), 0.15)
        api_val = api_fit.validate(options=ValOptions(cpu=True))
        assert api_val.passed is False
        assert api_val.details.cpu.used == round(i / 10, 1)
        assert api_val.details.cpu.max == 0.15
        assert len(api_val.details.cpu.users) == i


def test_no_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu)
    client.mk_eve_attr(id_=consts.EveAttr.cpu_output)
    eve_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.online)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
    assert api_stats.cpu == (5, None)
    api_val = api_fit.validate(options=ValOptions(cpu=True))
    assert api_val.passed is False
    assert api_val.details.cpu.used == 5
    assert api_val.details.cpu.max is None
    assert api_val.details.cpu.users == {api_module.id: 5}


def test_not_loaded_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu)
    client.mk_eve_attr(id_=consts.EveAttr.cpu_output)
    eve_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 5})
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.online)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
    assert api_stats.cpu == (5, None)
    api_val = api_fit.validate(options=ValOptions(cpu=True))
    assert api_val.passed is False
    assert api_val.details.cpu.used == 5
    assert api_val.details.cpu.max is None
    assert api_val.details.cpu.users == {api_module.id: 5}


def test_not_loaded_user(client, consts):
    # Just check that nothing crashes, not loaded items are not supposed to even be registered
    client.mk_eve_attr(id_=consts.EveAttr.cpu)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu_output)
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    eve_module_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.online)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
    assert api_stats.cpu == (0, 125)
    api_val = api_fit.validate(options=ValOptions(cpu=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_non_positive(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu_output)
    eve_module1_id = client.mk_eve_item(attrs={eve_use_attr_id: 0})
    eve_module2_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_module3_id = client.mk_eve_item(attrs={eve_use_attr_id: -10})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.online)
    api_module2 = api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.online)
    api_fit.add_module(type_id=eve_module3_id, state=consts.ApiModuleState.online)
    # Verification - items with negative and 0 use are not exposed
    api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
    assert api_stats.cpu == (140, 125)
    api_val = api_fit.validate(options=ValOptions(cpu=True))
    assert api_val.passed is False
    assert api_val.details.cpu.used == 140
    assert api_val.details.cpu.max == 125
    assert api_val.details.cpu.users == {api_module2.id: 150}


def test_no_value_use(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu_output)
    eve_module1_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_module2_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.online)
    api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.online)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
    assert api_stats.cpu == (150, 125)
    api_val = api_fit.validate(options=ValOptions(cpu=True))
    assert api_val.passed is False
    assert api_val.details.cpu.used == 150
    assert api_val.details.cpu.max == 125
    assert api_val.details.cpu.users == {api_module1.id: 150}


def test_no_value_max(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu)
    client.mk_eve_attr(id_=consts.EveAttr.cpu_output)
    eve_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.online)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
    assert api_stats.cpu == (150, 0)
    api_val = api_fit.validate(options=ValOptions(cpu=True))
    assert api_val.passed is False
    assert api_val.details.cpu.used == 150
    assert api_val.details.cpu.max == 0
    assert api_val.details.cpu.users == {api_module.id: 150}


def test_criterion_module_state(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu_output)
    eve_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.offline)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
    assert api_stats.cpu == (0, 125)
    api_val = api_fit.validate(options=ValOptions(cpu=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
    assert api_stats.cpu == (150, 125)
    api_val = api_fit.validate(options=ValOptions(cpu=True))
    assert api_val.passed is False
    assert api_val.details.cpu.used == 150
    assert api_val.details.cpu.max == 125
    assert api_val.details.cpu.users == {api_module.id: 150}
    # Action
    api_module.change_module(state=consts.ApiModuleState.offline)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
    assert api_stats.cpu == (0, 125)
    api_val = api_fit.validate(options=ValOptions(cpu=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_criterion_item_kind(client, consts):
    # Validation applies only to modules
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cpu_output)
    eve_item_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125, eve_use_attr_id: 150})
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_fighter_id = client.mk_eve_item(
        attrs={eve_autocharge_attr_id: eve_item_id, eve_use_attr_id: 150},
        eff_ids=[eve_autocharge_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_booster(type_id=eve_item_id)
    api_fit.set_character(type_id=eve_item_id)
    api_fit.add_drone(type_id=eve_item_id, state=consts.ApiMinionState.engaging)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fit.add_fw_effect(type_id=eve_item_id)
    api_fit.add_implant(type_id=eve_item_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.overload, charge_type_id=eve_item_id)
    api_fit.add_rig(type_id=eve_item_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_skill(type_id=eve_item_id, level=5)
    api_fit.set_stance(type_id=eve_item_id)
    api_fit.add_subsystem(type_id=eve_item_id)
    # Verification
    assert len(api_fighter.autocharges) == 1
    api_stats = api_fit.get_stats(options=FitStatsOptions(cpu=True))
    assert api_stats.cpu == (0, 125)
    api_val = api_fit.validate(options=ValOptions(cpu=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
