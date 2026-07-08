from fw import approx, check_no_field
from fw.api import ValOptions


def test_bundled(client, consts):
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_group, unit_id=consts.EveAttrUnit.group_id)
    eve_charge1_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_grp1_id})
    eve_charge2_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_grp2_id})
    eve_module_id = client.mk_eve_item(grp_id=eve_grp1_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module1.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module2 = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is False
    assert api_val.details.charge_parent_group == {api_module2.charge.id: (api_module2.id, eve_grp1_id, [eve_grp2_id])}
    # Action
    api_module2.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_separate(client, consts):
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_group5, unit_id=consts.EveAttrUnit.group_id)
    eve_charge1_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_grp1_id})
    eve_charge2_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_grp2_id})
    eve_module_id = client.mk_eve_item(grp_id=eve_grp1_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(charge_type_id=eve_charge1_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(charge_type_id=eve_charge2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is False
    assert api_val.details.charge_parent_group == {api_module.charge.id: (api_module.id, eve_grp1_id, [eve_grp2_id])}
    # Action
    api_module.change_module(charge_type_id=None)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_multiple_different(client, consts):
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_grp3_id = client.mk_eve_item_group()
    eve_group_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_group3, unit_id=consts.EveAttrUnit.group_id)
    eve_group_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_group4, unit_id=consts.EveAttrUnit.group_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_group_attr1_id: eve_grp1_id, eve_group_attr2_id: eve_grp3_id})
    eve_module1_id = client.mk_eve_item(grp_id=eve_grp1_id)
    eve_module2_id = client.mk_eve_item(grp_id=eve_grp2_id)
    eve_module3_id = client.mk_eve_item(grp_id=eve_grp3_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module1_id, charge_type_id=eve_charge_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module1.remove()
    api_module2 = api_fit.add_module(type_id=eve_module2_id, charge_type_id=eve_charge_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is False
    assert api_val.details.charge_parent_group == {
        api_module2.charge.id: (api_module2.id, eve_grp2_id, [eve_grp1_id, eve_grp3_id])}
    # Action
    api_module2.remove()
    api_fit.add_module(type_id=eve_module3_id, charge_type_id=eve_charge_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_known_failures(client, consts):
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_group6, unit_id=consts.EveAttrUnit.group_id)
    eve_charge1_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_grp1_id})
    eve_charge2_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_grp2_id})
    eve_module_id = client.mk_eve_item(grp_id=eve_grp1_id)
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_implant(type_id=eve_other_id)
    api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    api_module2 = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge2_id)
    api_module3 = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=(True, [api_module2.charge.id])))
    assert api_val.passed is False
    assert api_val.details.charge_parent_group == {api_module3.charge.id: (api_module3.id, eve_grp1_id, [eve_grp2_id])}
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=(True, [api_module3.charge.id])))
    assert api_val.passed is False
    assert api_val.details.charge_parent_group == {api_module2.charge.id: (api_module2.id, eve_grp1_id, [eve_grp2_id])}
    api_val = api_fit.validate(options=ValOptions(
        charge_parent_group=(True, [api_module2.charge.id, api_module3.charge.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(
        charge_parent_group=(True, [api_module2.charge.id, api_other.id, api_module3.charge.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_rounding(client, consts):
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_grp3_id = client.mk_eve_item_group()
    eve_group_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_group, unit_id=consts.EveAttrUnit.group_id)
    eve_group_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_group2, unit_id=consts.EveAttrUnit.group_id)
    eve_group_attr3_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_group3, unit_id=consts.EveAttrUnit.group_id)
    eve_charge_id = client.mk_eve_item(attrs={
        eve_group_attr1_id: eve_grp1_id - 0.4,
        eve_group_attr2_id: eve_grp2_id,
        eve_group_attr3_id: eve_grp1_id + 0.4})
    eve_module1_id = client.mk_eve_item(grp_id=eve_grp1_id)
    eve_module2_id = client.mk_eve_item(grp_id=eve_grp2_id)
    eve_module3_id = client.mk_eve_item(grp_id=eve_grp3_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module1_id, charge_type_id=eve_charge_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module1.remove()
    api_module2 = api_fit.add_module(type_id=eve_module3_id, charge_type_id=eve_charge_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is False
    assert api_val.details.charge_parent_group == {
        api_module2.charge.id: (api_module2.id, eve_grp3_id, [eve_grp1_id, eve_grp2_id])}
    # Action
    api_module2.remove()
    api_fit.add_module(type_id=eve_module2_id, charge_type_id=eve_charge_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_switch_type_id_module(client, consts):
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_group, unit_id=consts.EveAttrUnit.group_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_grp1_id})
    eve_module1_id = client.mk_eve_item(grp_id=eve_grp1_id)
    eve_module2_id = client.mk_eve_item(grp_id=eve_grp2_id)
    eve_module3_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module1_id, charge_type_id=eve_charge_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(type_id=eve_module2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is False
    assert api_val.details.charge_parent_group == {api_module.charge.id: (api_module.id, eve_grp2_id, [eve_grp1_id])}
    # Action
    api_module.change_module(type_id=eve_module3_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(type_id=eve_module2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is False
    assert api_val.details.charge_parent_group == {api_module.charge.id: (api_module.id, eve_grp2_id, [eve_grp1_id])}


def test_switch_type_id_charge(client, consts):
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_group, unit_id=consts.EveAttrUnit.group_id)
    eve_charge1_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_grp1_id})
    eve_charge2_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_grp2_id})
    eve_charge3_id = client.alloc_item_id()
    eve_module_id = client.mk_eve_item(grp_id=eve_grp1_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.charge.change_charge(type_id=eve_charge2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is False
    assert api_val.details.charge_parent_group == {api_module.charge.id: (api_module.id, eve_grp1_id, [eve_grp2_id])}
    # Action
    api_module.charge.change_charge(type_id=eve_charge3_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.charge.change_charge(type_id=eve_charge2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is False
    assert api_val.details.charge_parent_group == {api_module.charge.id: (api_module.id, eve_grp1_id, [eve_grp2_id])}


def test_modified(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_group, unit_id=consts.EveAttrUnit.group_id)
    eve_module1_id = client.mk_eve_item(grp_id=eve_grp1_id)
    eve_module2_id = client.mk_eve_item(grp_id=eve_grp2_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_grp2_id}, srqs={eve_skill_id: 1})
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_group_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: eve_grp1_id}, eff_ids=[eve_effect_id])
    eve_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_ship)
    api_module = api_fit.add_module(type_id=eve_module1_id, charge_type_id=eve_charge_id)
    # Verification
    assert api_module.charge.update().attrs[eve_group_attr_id].modified == approx(eve_grp1_id)
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is False
    assert api_val.details.charge_parent_group == {api_module.charge.id: (api_module.id, eve_grp1_id, [eve_grp2_id])}
    # Action
    api_module.change_module(type_id=eve_module2_id)
    # Verification
    assert api_module.charge.update().attrs[eve_group_attr_id].modified == approx(eve_grp1_id)
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_implant.remove()
    # Verification
    assert api_module.charge.update().attrs[eve_group_attr_id].modified == approx(eve_grp2_id)
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(type_id=eve_module1_id)
    # Verification
    assert api_module.charge.update().attrs[eve_group_attr_id].modified == approx(eve_grp2_id)
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is False
    assert api_val.details.charge_parent_group == {api_module.charge.id: (api_module.id, eve_grp1_id, [eve_grp2_id])}


def test_mutation(client, consts):
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_group, unit_id=consts.EveAttrUnit.group_id)
    eve_base_module_id = client.mk_eve_item(grp_id=eve_grp1_id)
    eve_mutated_module_id = client.mk_eve_item(grp_id=eve_grp2_id)
    eve_charge1_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_grp1_id})
    eve_charge2_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_grp2_id})
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_module_id], eve_mutated_module_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_base_module_id, charge_type_id=eve_charge1_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(mutation=eve_mutator_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is False
    assert api_val.details.charge_parent_group == {api_module.charge.id: (api_module.id, eve_grp2_id, [eve_grp1_id])}
    # Action
    api_module.change_module(charge_type_id=eve_charge2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(mutation=None)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is False
    assert api_val.details.charge_parent_group == {api_module.charge.id: (api_module.id, eve_grp1_id, [eve_grp2_id])}


def test_no_charge(client):
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_no_attr(client, consts):
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_group_attr_id = consts.EveAttr.launcher_group
    eve_charge_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_grp1_id})
    eve_module_id = client.mk_eve_item(grp_id=eve_grp2_id)
    # Create an item which has the group, just to prevent the group from being cleaned up
    client.mk_eve_item(grp_id=eve_grp1_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification - limits defined with attribute which doesn't exist are ignored
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_not_loaded_module(client, consts):
    eve_grp_id = client.mk_eve_item_group()
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_group, unit_id=consts.EveAttrUnit.group_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_grp_id})
    eve_module_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_not_loaded_charge(client):
    eve_charge_id = client.alloc_item_id()
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_state(client, consts):
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_group, unit_id=consts.EveAttrUnit.group_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_grp2_id})
    eve_module_id = client.mk_eve_item(grp_id=eve_grp1_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        charge_type_id=eve_charge_id,
        state=consts.ApiModuleState.disabled)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is False
    assert api_val.details.charge_parent_group == {api_module.charge.id: (api_module.id, eve_grp1_id, [eve_grp2_id])}
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    api_module.charge.change_charge(state=False)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is False
    assert api_val.details.charge_parent_group == {api_module.charge.id: (api_module.id, eve_grp1_id, [eve_grp2_id])}


def test_criterion_item_kind(client, consts):
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_group, unit_id=consts.EveAttrUnit.group_id)
    eve_item_id = client.mk_eve_item(grp_id=eve_grp1_id, attrs={eve_group_attr_id: eve_grp2_id})
    eve_charge_id = client.mk_eve_item(grp_id=eve_grp1_id, attrs={eve_group_attr_id: eve_grp1_id})
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_fighter_id = client.mk_eve_item(
        grp_id=eve_grp1_id,
        attrs={eve_autocharge_attr_id: eve_item_id, eve_group_attr_id: eve_grp2_id},
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
    api_fit.add_module(type_id=eve_item_id, state=consts.ApiModuleState.overload, charge_type_id=eve_charge_id)
    api_fit.add_rig(type_id=eve_item_id)
    api_fit.add_service(type_id=eve_item_id, state=consts.ApiServiceState.online)
    api_fit.set_ship(type_id=eve_item_id)
    api_fit.add_skill(type_id=eve_item_id, level=5)
    api_fit.set_stance(type_id=eve_item_id)
    api_fit.add_subsystem(type_id=eve_item_id)
    # Verification
    assert len(api_fighter.autocharges) == 1
    api_val = api_fit.validate(options=ValOptions(charge_parent_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
