from tests import Muta, approx, check_no_field
from tests.fw.api import ValOptions


def test_same_value_module(client, consts):
    # As of 2025-03-18, only PANIC uses it
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_type_fitted)
    eve_module_id = client.mk_eve_item(attrs={eve_limit_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module_id, state=consts.ApiServiceState.offline)
    api_module2 = api_fit.add_module(type_id=eve_module_id, state=consts.ApiServiceState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(max_type_fitted=True))
    assert api_val.passed is False
    assert api_val.details.max_type_fitted == {eve_module_id: [2, {api_module1.id: 1, api_module2.id: 1}]}


def test_same_value_service(client, consts):
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_type_fitted)
    eve_service_id = client.mk_eve_item(attrs={eve_limit_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_service1 = api_fit.add_service(type_id=eve_service_id, state=consts.ApiServiceState.offline)
    api_service2 = api_fit.add_service(type_id=eve_service_id, state=consts.ApiServiceState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(max_type_fitted=True))
    assert api_val.passed is False
    assert api_val.details.max_type_fitted == {eve_service_id: [2, {api_service1.id: 1, api_service2.id: 1}]}


def test_known_failures(client, consts):
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_type_fitted)
    eve_service_id = client.mk_eve_item(attrs={eve_limit_attr_id: 1})
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_implant(type_id=eve_other_id)
    api_service1 = api_fit.add_service(type_id=eve_service_id, state=consts.ApiServiceState.offline)
    api_service2 = api_fit.add_service(type_id=eve_service_id, state=consts.ApiServiceState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(max_type_fitted=(True, [api_service1.id])))
    assert api_val.passed is False
    assert api_val.details.max_type_fitted == {eve_service_id: [2, {api_service2.id: 1}]}
    api_val = api_fit.validate(options=ValOptions(max_type_fitted=(True, [api_service2.id])))
    assert api_val.passed is False
    assert api_val.details.max_type_fitted == {eve_service_id: [2, {api_service1.id: 1}]}
    api_val = api_fit.validate(options=ValOptions(max_type_fitted=(True, [api_service1.id, api_service2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(
        max_type_fitted=(True, [api_service1.id, api_other.id, api_service2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_switch_type_id(client, consts):
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_type_fitted)
    eve_module1_id = client.mk_eve_item(attrs={eve_limit_attr_id: 1})
    eve_module2_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module1_id, state=consts.ApiServiceState.offline)
    api_module2 = api_fit.add_module(type_id=eve_module1_id, state=consts.ApiServiceState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(max_type_fitted=True))
    assert api_val.passed is False
    assert api_val.details.max_type_fitted == {eve_module1_id: [2, {api_module1.id: 1, api_module2.id: 1}]}
    # Action
    api_module2.change_module(type_id=eve_module2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(max_type_fitted=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module2.change_module(type_id=eve_module1_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(max_type_fitted=True))
    assert api_val.passed is False
    assert api_val.details.max_type_fitted == {eve_module1_id: [2, {api_module1.id: 1, api_module2.id: 1}]}


def test_rounding(client, consts):
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_type_fitted)
    eve_service1_id = client.mk_eve_item(attrs={eve_limit_attr_id: 0.6})
    eve_service2_id = client.mk_eve_item(attrs={eve_limit_attr_id: 1.4})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_service1 = api_fit.add_service(type_id=eve_service1_id, state=consts.ApiServiceState.offline)
    api_service2 = api_fit.add_service(type_id=eve_service1_id, state=consts.ApiServiceState.offline)
    api_service3 = api_fit.add_service(type_id=eve_service2_id, state=consts.ApiServiceState.offline)
    api_service4 = api_fit.add_service(type_id=eve_service2_id, state=consts.ApiServiceState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(max_type_fitted=True))
    assert api_val.passed is False
    assert api_val.details.max_type_fitted == {
        eve_service1_id: [2, {api_service1.id: 1, api_service2.id: 1}],
        eve_service2_id: [2, {api_service3.id: 1, api_service4.id: 1}]}


def test_non_positive(client, consts):
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_type_fitted)
    eve_service1_id = client.mk_eve_item(attrs={eve_limit_attr_id: -2})
    eve_service2_id = client.mk_eve_item(attrs={eve_limit_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_service1 = api_fit.add_service(type_id=eve_service1_id, state=consts.ApiServiceState.offline)
    api_service2 = api_fit.add_service(type_id=eve_service2_id, state=consts.ApiServiceState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(max_type_fitted=True))
    assert api_val.passed is False
    assert api_val.details.max_type_fitted == {
        eve_service1_id: [1, {api_service1.id: 0}],
        eve_service2_id: [1, {api_service2.id: 0}]}
    # Action
    api_service1.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(max_type_fitted=True))
    assert api_val.passed is False
    assert api_val.details.max_type_fitted == {eve_service2_id: [1, {api_service2.id: 0}]}
    # Action
    api_service2.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(max_type_fitted=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_modified(client, consts):
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_type_fitted)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_limit_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_service_id = client.mk_eve_item(attrs={eve_limit_attr_id: 1, eve_mod_attr_id: 1}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_service1 = api_fit.add_service(type_id=eve_service_id, state=consts.ApiServiceState.offline)
    api_service2 = api_fit.add_service(type_id=eve_service_id, state=consts.ApiServiceState.offline)
    # Verification
    assert api_service1.update().attrs[eve_limit_attr_id].extra == approx(2)
    assert api_service2.update().attrs[eve_limit_attr_id].extra == approx(2)
    api_val = api_fit.validate(options=ValOptions(max_type_fitted=True))
    assert api_val.passed is False
    assert api_val.details.max_type_fitted == {eve_service_id: [2, {api_service1.id: 1, api_service2.id: 1}]}


def test_mutation_limit_priority(client, consts):
    # Unrealistic scenario, just checking what happens
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_type_fitted)
    eve_base_module_id = client.mk_eve_item(attrs={eve_limit_attr_id: 2})
    eve_mutated_module_id = client.mk_eve_item(attrs={eve_limit_attr_id: 1})
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_module_id], eve_mutated_module_id)],
        attrs={eve_limit_attr_id: (1, 5)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(
        type_id=eve_base_module_id,
        state=consts.ApiServiceState.offline,
        mutation=eve_mutator_id)
    api_module2 = api_fit.add_module(
        type_id=eve_base_module_id,
        state=consts.ApiServiceState.offline,
        mutation=(eve_mutator_id, {eve_limit_attr_id: Muta.roll_to_api(val=1)}))
    # Verification
    assert api_module1.update().attrs[eve_limit_attr_id].extra == approx(1)
    assert api_module2.update().attrs[eve_limit_attr_id].extra == approx(5)
    api_val = api_fit.validate(options=ValOptions(max_type_fitted=True))
    assert api_val.passed is False
    assert api_val.details.max_type_fitted == {eve_mutated_module_id: [2, {api_module1.id: 1, api_module2.id: 1}]}
    # Action
    api_module2.change_module(mutation=None)
    # Verification
    assert api_module1.update().attrs[eve_limit_attr_id].extra == approx(1)
    assert api_module2.update().attrs[eve_limit_attr_id].extra == approx(2)
    api_val = api_fit.validate(options=ValOptions(max_type_fitted=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module1.change_module(mutation=None)
    # Verification
    assert api_module1.update().attrs[eve_limit_attr_id].extra == approx(2)
    assert api_module2.update().attrs[eve_limit_attr_id].extra == approx(2)
    api_val = api_fit.validate(options=ValOptions(max_type_fitted=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_mutation_limit_inheritance(client, consts):
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_type_fitted)
    eve_base_module_id = client.mk_eve_item(attrs={eve_limit_attr_id: 1})
    eve_mutated_module_id = client.mk_eve_item()
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_module_id], eve_mutated_module_id)],
        attrs={eve_limit_attr_id: (1, 5)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(
        type_id=eve_base_module_id,
        state=consts.ApiServiceState.offline,
        mutation=eve_mutator_id)
    api_module2 = api_fit.add_module(
        type_id=eve_base_module_id,
        state=consts.ApiServiceState.offline,
        mutation=(eve_mutator_id, {eve_limit_attr_id: Muta.roll_to_api(val=1)}))
    # Verification
    assert api_module1.update().attrs[eve_limit_attr_id].extra == approx(1)
    assert api_module2.update().attrs[eve_limit_attr_id].extra == approx(5)
    api_val = api_fit.validate(options=ValOptions(max_type_fitted=True))
    assert api_val.passed is False
    assert api_val.details.max_type_fitted == {eve_mutated_module_id: [2, {api_module1.id: 1, api_module2.id: 1}]}
    # Action
    api_module2.change_module(mutation=None)
    # Verification
    assert api_module1.update().attrs[eve_limit_attr_id].extra == approx(1)
    assert api_module2.update().attrs[eve_limit_attr_id].extra == approx(1)
    api_val = api_fit.validate(options=ValOptions(max_type_fitted=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module1.change_module(mutation=None)
    # Verification
    assert api_module1.update().attrs[eve_limit_attr_id].extra == approx(1)
    assert api_module2.update().attrs[eve_limit_attr_id].extra == approx(1)
    api_val = api_fit.validate(options=ValOptions(max_type_fitted=True))
    assert api_val.passed is False
    assert api_val.details.max_type_fitted == {eve_base_module_id: [2, {api_module1.id: 1, api_module2.id: 1}]}


def test_not_loaded(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.max_type_fitted)
    eve_service_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_service(type_id=eve_service_id, state=consts.ApiServiceState.offline)
    api_fit.add_service(type_id=eve_service_id, state=consts.ApiServiceState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(max_type_fitted=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_criterion_item_state(client, consts):
    # Ghost modules and services are affected
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_type_fitted)
    eve_item_id = client.mk_eve_item(attrs={eve_limit_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_item_id, state=consts.ApiModuleState.ghost)
    api_service = api_fit.add_service(type_id=eve_item_id, state=consts.ApiServiceState.ghost)
    # Verification
    api_val = api_fit.validate(options=ValOptions(max_type_fitted=True))
    assert api_val.passed is False
    assert api_val.details.max_type_fitted == {eve_item_id: [2, {api_module.id: 1, api_service.id: 1}]}


def test_criterion_item_kind(client, consts):
    # Validation applies only to modules and services
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_type_fitted)
    eve_item_id = client.mk_eve_item(attrs={eve_limit_attr_id: 0})
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_fighter_id = client.mk_eve_item(
        attrs={eve_autocharge_attr_id: eve_item_id, eve_limit_attr_id: 0},
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
    api_fit.add_rig(type_id=eve_item_id)
    api_fit.set_ship(type_id=eve_item_id)
    api_fit.add_skill(type_id=eve_item_id, level=5)
    api_fit.set_stance(type_id=eve_item_id)
    api_fit.add_subsystem(type_id=eve_item_id)
    # Verification
    assert len(api_fighter.autocharges) == 1
    api_val = api_fit.validate(options=ValOptions(max_type_fitted=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
