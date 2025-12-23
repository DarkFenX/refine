from fw import Muta, approx, check_no_field
from fw.api import ValOptions


def test_main(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.required_thermodynamics_skill)
    eve_module1_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    eve_module2_id = client.mk_eve_item(attrs={eve_attr_id: 2})
    eve_module3_id = client.mk_eve_item(attrs={eve_attr_id: 5})
    eve_skill_id = client.mk_eve_item(id_=consts.EveItem.thermodynamics)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_skill = api_fit.add_skill(type_id=eve_skill_id, level=0)
    api_module1 = api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.overload)
    api_module2 = api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.overload)
    api_module3 = api_fit.add_module(type_id=eve_module3_id, state=consts.ApiModuleState.overload)
    # Verification
    api_val = api_fit.validate(options=ValOptions(overload_skill=True))
    assert api_val.passed is False
    assert api_val.details.overload_skill.td_lvl == 0
    assert api_val.details.overload_skill.module_reqs == {api_module1.id: 1, api_module2.id: 2, api_module3.id: 5}
    # Action
    api_skill.change_skill(level=1)
    # Verification
    api_val = api_fit.validate(options=ValOptions(overload_skill=True))
    assert api_val.passed is False
    assert api_val.details.overload_skill.td_lvl == 1
    assert api_val.details.overload_skill.module_reqs == {api_module2.id: 2, api_module3.id: 5}
    # Action
    api_skill.change_skill(level=5)
    # Verification
    api_val = api_fit.validate(options=ValOptions(overload_skill=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_known_failures(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.required_thermodynamics_skill)
    eve_module_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    eve_other_id = client.mk_eve_item()
    eve_skill_id = client.mk_eve_item(id_=consts.EveItem.thermodynamics)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_skill = api_fit.add_skill(type_id=eve_skill_id, level=0)
    api_other = api_fit.add_implant(type_id=eve_other_id)
    api_module1 = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.overload)
    api_module2 = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.overload)
    # Verification
    api_val = api_fit.validate(options=ValOptions(overload_skill=(True, [api_module1.id])))
    assert api_val.passed is False
    assert api_val.details.overload_skill.td_lvl == 0
    assert api_val.details.overload_skill.module_reqs == {api_module2.id: 1}
    api_val = api_fit.validate(options=ValOptions(overload_skill=(True, [api_module2.id])))
    assert api_val.passed is False
    assert api_val.details.overload_skill.td_lvl == 0
    assert api_val.details.overload_skill.module_reqs == {api_module1.id: 1}
    api_val = api_fit.validate(options=ValOptions(overload_skill=(True, [api_module1.id, api_module2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(
        overload_skill=(True, [api_module1.id, api_other.id, api_module2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_skill.remove()
    # Verification - there is separate logic for no-skill case, check it here
    api_val = api_fit.validate(options=ValOptions(overload_skill=(True, [api_module1.id])))
    assert api_val.passed is False
    assert api_val.details.overload_skill.td_lvl is None
    assert api_val.details.overload_skill.module_reqs == {api_module2.id: 1}
    api_val = api_fit.validate(options=ValOptions(overload_skill=(True, [api_module2.id])))
    assert api_val.passed is False
    assert api_val.details.overload_skill.td_lvl is None
    assert api_val.details.overload_skill.module_reqs == {api_module1.id: 1}
    api_val = api_fit.validate(options=ValOptions(overload_skill=(True, [api_module1.id, api_module2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(
        overload_skill=(True, [api_module1.id, api_other.id, api_module2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_non_positive(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.required_thermodynamics_skill)
    eve_module1_id = client.mk_eve_item(attrs={eve_attr_id: 0})
    eve_module2_id = client.mk_eve_item(attrs={eve_attr_id: -2})
    eve_skill_id = client.mk_eve_item(id_=consts.EveItem.thermodynamics)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_skill = api_fit.add_skill(type_id=eve_skill_id, level=0)
    api_module1 = api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.overload)
    api_module2 = api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.overload)
    # Verification
    api_val = api_fit.validate(options=ValOptions(overload_skill=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_skill.remove()
    # Verification - 2nd module required level is changed to 0 because of detail of implementation
    # (needed thermodynamics level is stored as unsigned int)
    api_val = api_fit.validate(options=ValOptions(overload_skill=True))
    assert api_val.passed is False
    assert api_val.details.overload_skill.td_lvl is None
    assert api_val.details.overload_skill.module_reqs == {api_module1.id: 0, api_module2.id: 0}


def test_rounding(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.required_thermodynamics_skill)
    eve_module1_id = client.mk_eve_item(attrs={eve_attr_id: 1.4})
    eve_module2_id = client.mk_eve_item(attrs={eve_attr_id: 1.6})
    eve_skill_id = client.mk_eve_item(id_=consts.EveItem.thermodynamics)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_skill = api_fit.add_skill(type_id=eve_skill_id, level=0)
    api_module1 = api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.overload)
    api_module2 = api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.overload)
    # Verification
    api_val = api_fit.validate(options=ValOptions(overload_skill=True))
    assert api_val.passed is False
    assert api_val.details.overload_skill.td_lvl == 0
    assert api_val.details.overload_skill.module_reqs == {api_module1.id: 1, api_module2.id: 2}
    # Action
    api_skill.change_skill(level=1)
    # Verification
    api_val = api_fit.validate(options=ValOptions(overload_skill=True))
    assert api_val.passed is False
    assert api_val.details.overload_skill.td_lvl == 1
    assert api_val.details.overload_skill.module_reqs == {api_module2.id: 2}
    # Action
    api_skill.change_skill(level=2)
    # Verification
    api_val = api_fit.validate(options=ValOptions(overload_skill=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_modified_req(client, consts):
    eve_req_attr_id = client.mk_eve_attr(id_=consts.EveAttr.required_thermodynamics_skill)
    eve_module_id = client.mk_eve_item(attrs={eve_req_attr_id: 2})
    eve_skill_id = client.mk_eve_item(id_=consts.EveItem.thermodynamics)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_req_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 3}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_skill = api_fit.add_skill(type_id=eve_skill_id, level=1)
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.overload)
    # Verification
    assert api_module.update().attrs[eve_req_attr_id].modified == approx(3)
    api_val = api_fit.validate(options=ValOptions(overload_skill=True))
    assert api_val.passed is False
    assert api_val.details.overload_skill.td_lvl == 1
    assert api_val.details.overload_skill.module_reqs == {api_module.id: 2}
    # Action
    api_skill.change_skill(level=2)
    # Verification
    assert api_module.update().attrs[eve_req_attr_id].modified == approx(3)
    api_val = api_fit.validate(options=ValOptions(overload_skill=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_implant.remove()
    # Verification
    assert api_module.update().attrs[eve_req_attr_id].modified == approx(2)
    api_val = api_fit.validate(options=ValOptions(overload_skill=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_skill.change_skill(level=1)
    # Verification
    assert api_module.update().attrs[eve_req_attr_id].modified == approx(2)
    api_val = api_fit.validate(options=ValOptions(overload_skill=True))
    assert api_val.passed is False
    assert api_val.details.overload_skill.td_lvl == 1
    assert api_val.details.overload_skill.module_reqs == {api_module.id: 2}


def test_mutation_req(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.required_thermodynamics_skill)
    eve_base_module_id = client.mk_eve_item(attrs={eve_attr_id: 2})
    eve_mutated_module_id = client.mk_eve_item(attrs={eve_attr_id: 3})
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_module_id], eve_mutated_module_id)],
        attrs={eve_attr_id: (0.5, 1.5)})
    eve_skill_id = client.mk_eve_item(id_=consts.EveItem.thermodynamics)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_skill = api_fit.add_skill(type_id=eve_skill_id, level=1)
    api_module = api_fit.add_module(
        type_id=eve_base_module_id,
        state=consts.ApiModuleState.overload,
        mutation=(eve_mutator_id, {eve_attr_id: Muta.roll_to_api(val=0.9)}))
    # Verification
    assert api_module.update().attrs[eve_attr_id].modified == approx(4.2)
    api_val = api_fit.validate(options=ValOptions(overload_skill=True))
    assert api_val.passed is False
    assert api_val.details.overload_skill.td_lvl == 1
    assert api_val.details.overload_skill.module_reqs == {api_module.id: 3}
    # Action
    api_skill.change_skill(level=3)
    # Verification
    assert api_module.update().attrs[eve_attr_id].modified == approx(4.2)
    api_val = api_fit.validate(options=ValOptions(overload_skill=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_skill.change_skill(level=1)
    api_module.change_module(mutation={eve_attr_id: Muta.roll_to_api(val=0)})
    # Verification
    assert api_module.update().attrs[eve_attr_id].modified == approx(1.5)
    api_val = api_fit.validate(options=ValOptions(overload_skill=True))
    assert api_val.passed is False
    assert api_val.details.overload_skill.td_lvl == 1
    assert api_val.details.overload_skill.module_reqs == {api_module.id: 3}
    # Action
    api_module.change_module(mutation=None)
    # Verification
    assert api_module.update().attrs[eve_attr_id].modified == approx(2)
    api_val = api_fit.validate(options=ValOptions(overload_skill=True))
    assert api_val.passed is False
    assert api_val.details.overload_skill.td_lvl == 1
    assert api_val.details.overload_skill.module_reqs == {api_module.id: 2}


def test_no_skill(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.required_thermodynamics_skill)
    eve_module_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.overload)
    # Verification
    api_val = api_fit.validate(options=ValOptions(overload_skill=True))
    assert api_val.passed is False
    assert api_val.details.overload_skill.td_lvl is None
    assert api_val.details.overload_skill.module_reqs == {api_module.id: 1}


def test_no_value(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.required_thermodynamics_skill)
    eve_module_id = client.mk_eve_item()
    client.mk_eve_item(id_=consts.EveItem.thermodynamics)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.overload)
    # Verification
    api_val = api_fit.validate(options=ValOptions(overload_skill=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_no_attr(client, consts):
    eve_attr_id = consts.EveAttr.required_thermodynamics_skill
    eve_module_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    eve_skill_id = client.mk_eve_item(id_=consts.EveItem.thermodynamics)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_skill(type_id=eve_skill_id, level=0)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.overload)
    # Verification - no attribute means level can't be fetched, which means validation always passes
    api_val = api_fit.validate(options=ValOptions(overload_skill=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_not_loaded_module(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.required_thermodynamics_skill)
    eve_module_id = client.alloc_item_id()
    eve_skill_id = client.mk_eve_item(id_=consts.EveItem.thermodynamics)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_skill(type_id=eve_skill_id, level=0)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.overload)
    # Verification
    api_val = api_fit.validate(options=ValOptions(overload_skill=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_not_loaded_skill(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.required_thermodynamics_skill)
    eve_module_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    eve_skill_id = consts.EveItem.thermodynamics
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_skill = api_fit.add_skill(type_id=eve_skill_id, level=0)
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.overload)
    # Verification
    api_val = api_fit.validate(options=ValOptions(overload_skill=True))
    assert api_val.passed is False
    assert api_val.details.overload_skill.td_lvl == 0
    assert api_val.details.overload_skill.module_reqs == {api_module.id: 1}
    # Action
    api_skill.change_skill(level=1)
    # Verification
    api_val = api_fit.validate(options=ValOptions(overload_skill=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_criterion_module_state(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.required_thermodynamics_skill)
    eve_module_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    eve_skill_id = client.mk_eve_item(id_=consts.EveItem.thermodynamics)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_skill(type_id=eve_skill_id, level=0)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification
    api_val = api_fit.validate(options=ValOptions(overload_skill=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
