from fw.api import ValOptions


def test_bundled(client, consts):
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge1_id = client.mk_eve_charge(attrs={eve_size_attr_id: 2})
    eve_charge2_id = client.mk_eve_charge(attrs={eve_size_attr_id: 3})
    eve_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1, eve_size_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(charge_size=True),
        passing=[eve_charge1_id],
        failing=[eve_charge2_id])


def test_separate(client, consts):
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge1_id = client.mk_eve_charge(attrs={eve_size_attr_id: 2})
    eve_charge2_id = client.mk_eve_charge(attrs={eve_size_attr_id: 3})
    eve_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1, eve_size_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(charge_size=True),
        passing=[eve_charge1_id],
        failing=[eve_charge2_id])


def test_known_failures(client, consts):
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge1_id = client.mk_eve_charge(attrs={eve_size_attr_id: 2})
    eve_charge2_id = client.mk_eve_charge(attrs={eve_size_attr_id: 3})
    eve_module_id = client.mk_eve_item(attrs={eve_size_attr_id: 2, eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    api_module2 = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge2_id)
    api_module3 = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge2_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(charge_size=[api_module2.charge.id, api_module3.charge.id]),
        passing=[eve_charge1_id],
        failing=[eve_charge2_id])


def test_modified_module(client, consts):
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge1_id = client.mk_eve_charge(attrs={eve_size_attr_id: 2})
    eve_charge2_id = client.mk_eve_charge(attrs={eve_size_attr_id: 1})
    eve_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1, eve_size_attr_id: 1})
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_size_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 2}, eff_ids=[eve_effect_id])
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
        val_options=ValOptions(charge_size=True),
        passing=[eve_charge2_id],
        failing=[eve_charge1_id])


def test_modified_charge(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge1_id = client.mk_eve_charge(attrs={eve_size_attr_id: 2}, srqs={eve_skill_id: 1})
    eve_charge2_id = client.mk_eve_charge(attrs={eve_size_attr_id: 1}, srqs={eve_skill_id: 1})
    eve_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1, eve_size_attr_id: 1})
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_size_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 1}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_implant(type_id=eve_implant_id)
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    # Action
    api_module.change_module(charge_type_id=eve_charge2_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(charge_size=True),
        passing=[eve_charge2_id],
        failing=[eve_charge1_id])


def test_mutation_module(client, consts):
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge1_id = client.mk_eve_charge(attrs={eve_size_attr_id: 1})
    eve_charge2_id = client.mk_eve_charge(attrs={eve_size_attr_id: 2})
    eve_base_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1, eve_size_attr_id: 1})
    eve_mutated_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1, eve_size_attr_id: 2})
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_module_id], eve_mutated_module_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_base_module_id, charge_type_id=eve_charge1_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(charge_size=True),
        passing=[eve_charge1_id],
        failing=[eve_charge2_id])
    # Action
    api_module.change_module(mutation=eve_mutator_id)
    api_module.change_module(charge_type_id=eve_charge2_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(charge_size=True),
        passing=[eve_charge2_id],
        failing=[eve_charge1_id])


def test_non_positive(client, consts):
    # Size of 0 is actually used in EVE, small RASB charges
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge1_id = client.mk_eve_charge(attrs={eve_size_attr_id: 0})
    eve_charge2_id = client.mk_eve_charge(attrs={eve_size_attr_id: -3})
    eve_module1_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1, eve_size_attr_id: 0})
    eve_module2_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1, eve_size_attr_id: -3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module1_id, charge_type_id=eve_charge2_id)
    api_module2 = api_fit.add_module(type_id=eve_module2_id, charge_type_id=eve_charge1_id)
    # Action
    api_module1.change_module(charge_type_id=eve_charge1_id)
    api_module2.change_module(charge_type_id=eve_charge2_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(charge_size=True),
        passing=[eve_charge1_id, eve_charge2_id],
        failing=[])


def test_no_value_module(client, consts):
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_id = client.mk_eve_charge(attrs={eve_size_attr_id: 1})
    eve_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(charge_size=True),
        passing=[eve_charge_id],
        failing=[])


def test_no_attr(client, consts):
    eve_size_attr_id = consts.EveAttr.charge_size
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_id = client.mk_eve_charge(attrs={eve_size_attr_id: 1})
    eve_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1, eve_size_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification - validation is ignored if attribute does not exist
    api_fit.check_try_fit_items(
        val_options=ValOptions(charge_size=True),
        passing=[eve_charge_id],
        failing=[])
