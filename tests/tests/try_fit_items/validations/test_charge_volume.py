from fw import approx
from fw.api import ValOptions


def test_bundled(client, consts):
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_vol_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_charge1_id = client.mk_eve_charge(attrs={eve_vol_attr_id: 1})
    eve_charge2_id = client.mk_eve_charge(attrs={eve_vol_attr_id: 1.2})
    eve_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(charge_volume=True),
        passing=[eve_charge1_id],
        failing=[eve_charge2_id])


def test_separate(client, consts):
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_vol_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_charge1_id = client.mk_eve_charge(attrs={eve_vol_attr_id: 1})
    eve_charge2_id = client.mk_eve_charge(attrs={eve_vol_attr_id: 1.2})
    eve_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(charge_volume=True),
        passing=[eve_charge1_id],
        failing=[eve_charge2_id])


def test_known_failures(client, consts):
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_vol_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_charge1_id = client.mk_eve_charge(attrs={eve_vol_attr_id: 1})
    eve_charge2_id = client.mk_eve_charge(attrs={eve_vol_attr_id: 1.2})
    eve_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    api_module2 = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge2_id)
    api_module3 = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge2_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(charge_volume=[api_module2.charge.id, api_module3.charge.id]),
        passing=[eve_charge1_id],
        failing=[eve_charge2_id])


def test_modified_module(client, consts):
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_vol_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_charge1_id = client.mk_eve_charge(attrs={eve_vol_attr_id: 1.2})
    eve_charge2_id = client.mk_eve_charge(attrs={eve_vol_attr_id: 1})
    eve_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1})
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_capacity_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 2}, eff_ids=[eve_effect_id])
    eve_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_ship)
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge2_id)
    # Verification
    assert api_module.update().attrs[eve_capacity_attr_id].modified == approx(2)
    api_fit.check_try_fit_items(
        val_options=ValOptions(charge_volume=True),
        passing=[eve_charge2_id],
        failing=[eve_charge1_id])


def test_modified_charge(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_vol_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_charge1_id = client.mk_eve_charge(attrs={eve_vol_attr_id: 1.2}, srqs={eve_skill_id: 1})
    eve_charge2_id = client.mk_eve_charge(attrs={eve_vol_attr_id: 1}, srqs={eve_skill_id: 1})
    eve_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1})
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_vol_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 0.8}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_implant(type_id=eve_implant_id)
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge2_id)
    # Verification
    assert api_module.charge.update().attrs[eve_vol_attr_id].modified == approx(0.8)
    api_fit.check_try_fit_items(
        val_options=ValOptions(charge_volume=True),
        passing=[eve_charge2_id],
        failing=[eve_charge1_id])


def test_mutation_module(client, consts):
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_vol_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_charge1_id = client.mk_eve_charge(attrs={eve_vol_attr_id: 1})
    eve_charge2_id = client.mk_eve_charge(attrs={eve_vol_attr_id: 0.8})
    eve_base_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1})
    eve_mutated_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 0.8})
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_module_id], eve_mutated_module_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_base_module_id, mutation=eve_mutator_id, charge_type_id=eve_charge2_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(charge_volume=True),
        passing=[eve_charge2_id],
        failing=[eve_charge1_id])


def test_no_value_charge(client, consts):
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_charge_id = client.mk_eve_charge()
    eve_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(charge_volume=True), passing=[eve_charge_id], failing=[])
