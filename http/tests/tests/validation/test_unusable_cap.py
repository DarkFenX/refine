"""
Main use-case of this validation is not to allow cap-restricted modules on some ships, e.g. tackle
or ewar modules on sieged FAXes.
"""

from tests import Muta, approx, check_no_field
from tests.fw.api import ValOptions


def test_state(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_use_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use_attr_id)
    eve_module1_id = client.mk_eve_item(attrs={eve_use_attr_id: 1000}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_module2_id = client.mk_eve_item(attrs={eve_use_attr_id: 700}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_module3_id = client.mk_eve_item(attrs={eve_use_attr_id: 500}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.disabled)
    api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.disabled)
    api_fit.add_module(type_id=eve_module3_id, state=consts.ApiModuleState.disabled)
    # Verification - validation checks even modules which are not active
    api_val = api_fit.validate(options=ValOptions(unusable_cap=True))
    assert api_val.passed is False
    assert api_val.details.unusable_cap == (approx(750), {api_module1.id: approx(1000)})
    # Action
    api_module1.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(unusable_cap=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_multiple_items(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_use_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use_attr_id)
    eve_module1_id = client.mk_eve_item(attrs={eve_use_attr_id: 1000}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_module2_id = client.mk_eve_item(attrs={eve_use_attr_id: 1200}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_module(type_id=eve_module1_id)
    api_module2 = api_fit.add_module(type_id=eve_module2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unusable_cap=True))
    assert api_val.passed is False
    assert api_val.details.unusable_cap == (approx(750), {api_module1.id: approx(1000), api_module2.id: approx(1200)})


def test_multiple_effects(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_use1_attr_id = client.mk_eve_attr()
    eve_use2_attr_id = client.mk_eve_attr()
    eve_effect1_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use1_attr_id)
    eve_effect2_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use2_attr_id)
    eve_module1_id = client.mk_eve_item(
        attrs={eve_use1_attr_id: 1000, eve_use2_attr_id: 1200},
        eff_ids=[eve_effect1_id, eve_effect2_id],
        defeff_id=eve_effect1_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module1_id)
    # Verification - highest value is used
    api_val = api_fit.validate(options=ValOptions(unusable_cap=True))
    assert api_val.passed is False
    assert api_val.details.unusable_cap == (approx(750), {api_module.id: approx(1200)})


def test_known_failures(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_use_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use_attr_id)
    eve_module1_id = client.mk_eve_item(attrs={eve_use_attr_id: 1000}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_module2_id = client.mk_eve_item(attrs={eve_use_attr_id: 1200}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_other_id = client.mk_eve_item(attrs={eve_use_attr_id: 1400}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_implant(type_id=eve_other_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_module(type_id=eve_module1_id)
    api_module2 = api_fit.add_module(type_id=eve_module2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unusable_cap=(True, [api_module1.id])))
    assert api_val.passed is False
    assert api_val.details.unusable_cap == (approx(750), {api_module2.id: approx(1200)})
    api_val = api_fit.validate(options=ValOptions(unusable_cap=(True, [api_module2.id])))
    assert api_val.passed is False
    assert api_val.details.unusable_cap == (approx(750), {api_module1.id: approx(1000)})
    api_val = api_fit.validate(options=ValOptions(unusable_cap=(True, [api_module1.id, api_module2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(unusable_cap=(True, [api_module1.id, api_other.id, api_module2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_negative_use(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_use_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use_attr_id)
    eve_module_id = client.mk_eve_item(attrs={eve_use_attr_id: -1000}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unusable_cap=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_modified_use(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_use_attr_id = client.mk_eve_attr()
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_use_attr_id)
    eve_implant_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: -30}, eff_ids=[eve_implant_effect_id])
    eve_use_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_use_attr_id: 1000},
        eff_ids=[eve_use_effect_id],
        defeff_id=eve_use_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unusable_cap=True))
    assert api_val.passed is False
    assert api_val.details.unusable_cap == (approx(750), {api_module.id: approx(1000)})
    # Action
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unusable_cap=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_implant.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(unusable_cap=True))
    assert api_val.passed is False
    assert api_val.details.unusable_cap == (approx(750), {api_module.id: approx(1000)})


def test_modified_max(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_use_attr_id = client.mk_eve_attr()
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_max_attr_id)
    eve_implant_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 50}, eff_ids=[eve_implant_effect_id])
    eve_use_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_use_attr_id: 1000},
        eff_ids=[eve_use_effect_id],
        defeff_id=eve_use_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unusable_cap=True))
    assert api_val.passed is False
    assert api_val.details.unusable_cap == (approx(750), {api_module.id: approx(1000)})
    # Action
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unusable_cap=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_implant.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(unusable_cap=True))
    assert api_val.passed is False
    assert api_val.details.unusable_cap == (approx(750), {api_module.id: approx(1000)})


def test_mutation_attr(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_use_attr_id = client.mk_eve_attr()
    eve_use_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use_attr_id)
    eve_base_module_id = client.mk_eve_item(
        attrs={eve_use_attr_id: 1000},
        eff_ids=[eve_use_effect_id],
        defeff_id=eve_use_effect_id)
    eve_mutated_module_id = client.mk_eve_item(
        attrs={eve_use_attr_id: 1200},
        eff_ids=[eve_use_effect_id],
        defeff_id=eve_use_effect_id)
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_module_id], eve_mutated_module_id)],
        attrs={eve_use_attr_id: (0, 1.5)})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_base_module_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unusable_cap=True))
    assert api_val.passed is False
    assert api_val.details.unusable_cap == (approx(750), {api_module.id: approx(1000)})
    # Action
    api_module.change_module(mutation=(eve_mutator_id, {eve_use_attr_id: Muta.roll_to_api(val=0.9)}))
    # Verification
    api_val = api_fit.validate(options=ValOptions(unusable_cap=True))
    assert api_val.passed is False
    assert api_val.details.unusable_cap == (approx(750), {api_module.id: approx(1620)})
    # Action
    api_module.change_module(mutation={eve_use_attr_id: Muta.roll_to_api(val=0.2)})
    # Verification
    api_val = api_fit.validate(options=ValOptions(unusable_cap=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(mutation=None)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unusable_cap=True))
    assert api_val.passed is False
    assert api_val.details.unusable_cap == (approx(750), {api_module.id: approx(1000)})


def test_mutation_effect(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_use_attr_id = client.mk_eve_attr()
    eve_use_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use_attr_id)
    eve_base_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 1000})
    eve_mutated_module_id = client.mk_eve_item(eff_ids=[eve_use_effect_id], defeff_id=eve_use_effect_id)
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_module_id], eve_mutated_module_id)])
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_base_module_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unusable_cap=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(mutation=eve_mutator_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unusable_cap=True))
    assert api_val.passed is False
    assert api_val.details.unusable_cap == (approx(750), {api_module.id: approx(1000)})
    # Action
    api_module.change_module(mutation=None)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unusable_cap=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_no_ship(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_use_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use_attr_id)
    eve_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 1000}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unusable_cap=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_no_attr_use(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_use_attr_id = client.alloc_attr_id()
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use_attr_id)
    eve_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 1000}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module_id)
    # Verification - when use attribute doesn't exist, use is assumed to be 0
    api_val = api_fit.validate(options=ValOptions(unusable_cap=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_no_attr_max(client, consts):
    eve_max_attr_id = consts.EveAttr.capacitor_capacity
    eve_use_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use_attr_id)
    eve_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 100}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification - no cap amount attribute is considered as its value of 0
    api_val = api_fit.validate(options=ValOptions(unusable_cap=True))
    assert api_val.passed is False
    assert api_val.details.unusable_cap == (approx(0), {api_module.id: approx(100)})


def test_not_loaded_ship(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_use_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use_attr_id)
    eve_module_id = client.mk_eve_item(attrs={eve_use_attr_id: 1000}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unusable_cap=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_not_loaded_item(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_module_id = client.alloc_item_id()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unusable_cap=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_criterion_item_kind(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_use_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use_attr_id)
    eve_module_id = client.mk_eve_item()
    eve_item_id = client.mk_eve_item(
        attrs={eve_use_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_fighter_id = client.mk_eve_item(
        attrs={eve_autocharge_attr_id: eve_item_id, eve_use_attr_id: 1000},
        eff_ids=[eve_autocharge_effect_id, eve_effect_id])
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_max_attr_id: 750, eve_use_attr_id: 1000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
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
    api_fit.add_service(type_id=eve_item_id, state=consts.ApiServiceState.online)
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_skill(type_id=eve_item_id, level=5)
    api_fit.set_stance(type_id=eve_item_id)
    api_fit.add_subsystem(type_id=eve_item_id)
    # Verification
    assert len(api_fighter.autocharges) == 1
    api_val = api_fit.validate(options=ValOptions(unusable_cap=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
