from fw.api import ValOptions


def test_bundled(client, consts):
    eve_grp1_id = client.mk_eve_charge_group()
    eve_grp2_id = client.mk_eve_charge_group()
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge1_id = client.mk_eve_charge(grp_id=eve_grp1_id)
    eve_charge2_id = client.mk_eve_charge(grp_id=eve_grp2_id)
    eve_module_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_grp1_id, eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(charge_group=True),
        passing=[eve_charge1_id],
        failing=[eve_charge2_id])


def test_separate(client, consts):
    eve_grp1_id = client.mk_eve_charge_group()
    eve_grp2_id = client.mk_eve_charge_group()
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge1_id = client.mk_eve_charge(grp_id=eve_grp1_id)
    eve_charge2_id = client.mk_eve_charge(grp_id=eve_grp2_id)
    eve_module_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_grp1_id, eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(charge_group=True),
        passing=[eve_charge1_id],
        failing=[eve_charge2_id])


def test_multiple_different(client, consts):
    eve_grp1_id = client.mk_eve_charge_group()
    eve_grp2_id = client.mk_eve_charge_group()
    eve_grp3_id = client.mk_eve_charge_group()
    eve_group_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.charge_group3, unit_id=consts.EveAttrUnit.group_id)
    eve_group_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.charge_group4, unit_id=consts.EveAttrUnit.group_id)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge1_id = client.mk_eve_charge(grp_id=eve_grp1_id)
    eve_charge2_id = client.mk_eve_charge(grp_id=eve_grp2_id)
    eve_charge3_id = client.mk_eve_charge(grp_id=eve_grp3_id)
    eve_module_id = client.mk_eve_item(attrs={
        eve_group_attr1_id: eve_grp1_id,
        eve_group_attr2_id: eve_grp2_id,
        eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(charge_group=True),
        passing=[eve_charge1_id, eve_charge2_id],
        failing=[eve_charge3_id])


def test_known_failures(client, consts):
    eve_grp1_id = client.mk_eve_charge_group()
    eve_grp2_id = client.mk_eve_charge_group()
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge1_id = client.mk_eve_charge(grp_id=eve_grp1_id)
    eve_charge2_id = client.mk_eve_charge(grp_id=eve_grp2_id)
    eve_module_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_grp1_id, eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    api_module2 = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge2_id)
    api_module3 = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge2_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(charge_group=[api_module2.charge.id, api_module3.charge.id]),
        passing=[eve_charge1_id],
        failing=[eve_charge2_id])


def test_rounding(client, consts):
    eve_grp1_id = client.mk_eve_charge_group()
    eve_grp2_id = client.mk_eve_charge_group()
    eve_grp3_id = client.mk_eve_charge_group()
    eve_group_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.charge_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_group_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.charge_group2, unit_id=consts.EveAttrUnit.group_id)
    eve_group_attr3_id = client.mk_eve_attr(id_=consts.EveAttr.charge_group3, unit_id=consts.EveAttrUnit.group_id)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge1_id = client.mk_eve_charge(grp_id=eve_grp1_id)
    eve_charge2_id = client.mk_eve_charge(grp_id=eve_grp2_id)
    eve_charge3_id = client.mk_eve_charge(grp_id=eve_grp3_id)
    eve_module_id = client.mk_eve_item(attrs={
        eve_group_attr1_id: eve_grp1_id - 0.4,
        eve_group_attr2_id: eve_grp2_id,
        eve_group_attr3_id: eve_grp1_id + 0.4,
        eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(charge_group=True),
        passing=[eve_charge1_id, eve_charge2_id],
        failing=[eve_charge3_id])


def test_modified(client, consts):
    eve_grp1_id = client.mk_eve_charge_group()
    eve_grp2_id = client.mk_eve_charge_group()
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge1_id = client.mk_eve_charge(grp_id=eve_grp1_id)
    eve_charge2_id = client.mk_eve_charge(grp_id=eve_grp2_id)
    eve_module_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_grp2_id, eve_capacity_attr_id: 1})
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_group_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: eve_grp1_id}, eff_ids=[eve_effect_id])
    eve_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_ship)
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    # Action
    api_module.change_module(charge_type_id=eve_charge2_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(charge_group=True),
        passing=[eve_charge2_id],
        failing=[eve_charge1_id])


def test_mutation(client, consts):
    eve_grp1_id = client.mk_eve_charge_group()
    eve_grp2_id = client.mk_eve_charge_group()
    eve_grp3_id = client.mk_eve_charge_group()
    eve_group_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.charge_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_group_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.charge_group2, unit_id=consts.EveAttrUnit.group_id)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge1_id = client.mk_eve_charge(grp_id=eve_grp1_id)
    eve_charge2_id = client.mk_eve_charge(grp_id=eve_grp2_id)
    eve_charge3_id = client.mk_eve_charge(grp_id=eve_grp3_id)
    eve_base_module_id = client.mk_eve_item(attrs={
        eve_group_attr1_id: eve_grp1_id,
        eve_group_attr2_id: eve_grp2_id,
        eve_capacity_attr_id: 1})
    eve_mutated_module_id = client.mk_eve_item(attrs={
        eve_group_attr1_id: eve_grp3_id,
        eve_group_attr2_id: eve_grp2_id,
        eve_capacity_attr_id: 1})
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_module_id], eve_mutated_module_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_base_module_id, charge_type_id=eve_charge1_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(charge_group=True),
        passing=[eve_charge1_id, eve_charge2_id],
        failing=[eve_charge3_id])
    # Action
    api_module.change_module(mutation=eve_mutator_id)
    api_module.change_module(charge_type_id=eve_charge3_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(charge_group=True),
        passing=[eve_charge2_id, eve_charge3_id],
        failing=[eve_charge1_id])


def test_no_attr(client, consts):
    eve_grp1_id = client.mk_eve_charge_group()
    eve_grp2_id = client.mk_eve_charge_group()
    eve_group_attr1_id = consts.EveAttr.charge_group1
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_id = client.mk_eve_charge(grp_id=eve_grp1_id)
    # Create an item which has the group, just to prevent the group from being cleaned up
    client.mk_eve_charge(grp_id=eve_grp2_id)
    eve_module_id = client.mk_eve_item(attrs={eve_group_attr1_id: eve_grp2_id, eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification - limits defined with attribute which doesn't exist are ignored. Without it,
    # module has no limit, so validation passes
    api_fit.check_try_fit_items(
        val_options=ValOptions(charge_group=True),
        passing=[eve_charge_id],
        failing=[])
