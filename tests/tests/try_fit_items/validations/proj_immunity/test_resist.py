from fw import Muta, approx
from fw.api import ValOptions


def test_src_module_tgt_ship_project_unproject(client, consts):
    eve_resist_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, resist_attr_id=eve_resist_attr_id)
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_src_charge_id = client.mk_eve_charge(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_other_charge_id = client.mk_eve_charge()
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_resist_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_launcher = api_src_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(resist_immunity=True),
        passing=[eve_src_charge_id, eve_other_charge_id],
        failing=[])
    # Action
    api_src_launcher.change_module(add_proj_item_ids=[api_tgt_item.id])
    # Verification - charges are projected wherever the module they are loaded into is projected
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(resist_immunity=True),
        passing=[eve_other_charge_id],
        failing=[eve_src_charge_id])
    # Action
    api_src_launcher.change_module(rm_proj_item_ids=[api_tgt_item.id])
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(resist_immunity=True),
        passing=[eve_src_charge_id, eve_other_charge_id],
        failing=[])


def test_tgt_drone_fighter(client, consts):
    # Also check multiple projectee items
    eve_resist_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, resist_attr_id=eve_resist_attr_id)
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_src_charge_id = client.mk_eve_charge(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_item_id = client.mk_eve_item(attrs={eve_resist_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_launcher = api_src_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_drone = api_tgt_fit.add_drone(type_id=eve_tgt_item_id)
    api_tgt_fighter = api_tgt_fit.add_fighter(type_id=eve_tgt_item_id)
    api_src_launcher.change_module(add_proj_item_ids=[api_tgt_drone.id, api_tgt_fighter.id])
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(resist_immunity=True),
        passing=[],
        failing=[eve_src_charge_id])


def test_resist_values(client, consts):
    # Citadels have non-0 multiplier of 0.00001, and still block application of effects onto them.
    # The lib uses higher threshold of 0.0001, which we check here.
    eve_resist_attr_id = client.mk_eve_attr(def_val=1)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, resist_attr_id=eve_resist_attr_id)
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_src_charge_id = client.mk_eve_charge(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_item1_id = client.mk_eve_item(attrs={eve_resist_attr_id: -0.00011})
    eve_tgt_item2_id = client.mk_eve_item(attrs={eve_resist_attr_id: -0.0001})
    eve_tgt_item3_id = client.mk_eve_item(attrs={eve_resist_attr_id: 0.0001})
    eve_tgt_item4_id = client.mk_eve_item(attrs={eve_resist_attr_id: 0.00011})
    eve_tgt_item5_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_launcher = api_src_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item1 = api_tgt_fit.add_drone(type_id=eve_tgt_item1_id)
    api_tgt_item2 = api_tgt_fit.add_drone(type_id=eve_tgt_item2_id)
    api_tgt_item3 = api_tgt_fit.add_drone(type_id=eve_tgt_item3_id)
    api_tgt_item4 = api_tgt_fit.add_drone(type_id=eve_tgt_item4_id)
    api_tgt_item5 = api_tgt_fit.add_drone(type_id=eve_tgt_item5_id)
    api_src_launcher.change_module(add_proj_item_ids=[api_tgt_item4.id, api_tgt_item5.id])
    # Verification - values above threshold do not fail validation
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(resist_immunity=True),
        passing=[eve_src_charge_id],
        failing=[])
    # Action
    api_src_launcher.change_module(add_proj_item_ids=[api_tgt_item1.id])
    # Verification - any value below threshold fails validation
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(resist_immunity=True),
        passing=[],
        failing=[eve_src_charge_id])
    # Action
    api_src_launcher.change_module(rm_proj_item_ids=[api_tgt_item1.id], add_proj_item_ids=[api_tgt_item2.id])
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(resist_immunity=True),
        passing=[],
        failing=[eve_src_charge_id])
    # Action
    api_src_launcher.change_module(rm_proj_item_ids=[api_tgt_item2.id], add_proj_item_ids=[api_tgt_item3.id])
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(resist_immunity=True),
        passing=[],
        failing=[eve_src_charge_id])


def test_tgt_modified(client, consts):
    eve_resist_attr_id = client.mk_eve_attr()
    eve_mod_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, resist_attr_id=eve_resist_attr_id)
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_src_charge_id = client.mk_eve_charge(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_resist_attr_id: 0.1})
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_resist_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_mod_item_id = client.mk_eve_item(attrs={eve_mod_attr_id: 0}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_launcher = api_src_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    api_src_launcher.change_module(add_proj_item_ids=[api_tgt_item.id])
    # Verification
    assert api_tgt_item.update().attrs[eve_resist_attr_id].modified == approx(0.1)
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(resist_immunity=True),
        passing=[eve_src_charge_id],
        failing=[])
    # Action
    api_mod_item = api_tgt_fit.add_module(type_id=eve_mod_item_id)
    # Verification
    assert api_tgt_item.update().attrs[eve_resist_attr_id].modified == approx(0)
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(resist_immunity=True),
        passing=[],
        failing=[eve_src_charge_id])
    # Action
    api_mod_item.remove()
    # Verification
    assert api_tgt_item.update().attrs[eve_resist_attr_id].modified == approx(0.1)
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(resist_immunity=True),
        passing=[eve_src_charge_id],
        failing=[])


def test_src_modified(client, consts):
    # Any changes to remote resist definition are ignored, and unmodified value is used
    eve_resist_def_attr_id = client.mk_eve_attr(id_=consts.EveAttr.remote_resistance_id)
    eve_resist_attr1_id = client.mk_eve_attr()
    eve_resist_attr2_id = client.mk_eve_attr()
    eve_resist_attr3_id = client.mk_eve_attr()
    eve_mod_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target)
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_src_charge1_id = client.mk_eve_charge(
        attrs={eve_resist_def_attr_id: eve_resist_attr1_id},
        eff_ids=[eve_src_effect_id],
        defeff_id=eve_src_effect_id)
    eve_src_charge2_id = client.mk_eve_charge(
        attrs={eve_resist_def_attr_id: eve_resist_attr2_id},
        eff_ids=[eve_src_effect_id],
        defeff_id=eve_src_effect_id)
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_resist_attr1_id: 0, eve_resist_attr2_id: 1, eve_resist_attr3_id: 0})
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_resist_def_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_mod_item_id = client.mk_eve_item(attrs={eve_mod_attr_id: eve_resist_attr3_id}, eff_ids=[eve_mod_effect_id])
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    api_src_fit = api_sol.create_fit()
    api_mod_item = api_src_fit.add_implant(type_id=eve_mod_item_id)
    api_src_fit.set_ship(type_id=eve_ship_id)
    api_src_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active, proj_item_ids=[api_tgt_item.id])
    api_src_launcher = api_src_fit.add_module(
        type_id=eve_launcher_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_src_charge2_id,
        proj_item_ids=[api_tgt_item.id])
    # Verification - validation results are the same as if attributes were not modified
    assert api_src_launcher.charge.update().attrs[eve_resist_def_attr_id].modified == approx(eve_resist_attr3_id)
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(resist_immunity=True),
        passing=[eve_src_charge2_id],
        failing=[eve_src_charge1_id])
    # Action
    api_mod_item.remove()
    # Verification
    assert api_src_launcher.charge.update().attrs[eve_resist_def_attr_id].modified == approx(eve_resist_attr2_id)
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(resist_immunity=True),
        passing=[eve_src_charge2_id],
        failing=[eve_src_charge1_id])


def test_tgt_mutation(client, consts):
    eve_resist_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, resist_attr_id=eve_resist_attr_id)
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_src_charge_id = client.mk_eve_charge(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_base_item_id = client.mk_eve_item(attrs={eve_resist_attr_id: 1})
    eve_tgt_mutated_item_id = client.mk_eve_item(attrs={eve_resist_attr_id: 0})
    eve_tgt_mutator_id = client.mk_eve_mutator(
        items=[([eve_tgt_base_item_id], eve_tgt_mutated_item_id)],
        attrs={eve_resist_attr_id: (0, 2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_launcher = api_src_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.add_drone(type_id=eve_tgt_base_item_id)
    api_src_launcher.change_module(add_proj_item_ids=[api_tgt_item.id])
    # Verification
    assert api_tgt_item.update().attrs[eve_resist_attr_id].modified == approx(1)
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(resist_immunity=True),
        passing=[eve_src_charge_id],
        failing=[])
    # Action
    api_tgt_item.change_drone(mutation=eve_tgt_mutator_id)
    # Verification
    assert api_tgt_item.update().attrs[eve_resist_attr_id].modified == approx(1)
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(resist_immunity=True),
        passing=[eve_src_charge_id],
        failing=[])
    # Action
    api_tgt_item.change_drone(mutation={eve_resist_attr_id: Muta.roll_to_api(val=0)})
    # Verification
    assert api_tgt_item.update().attrs[eve_resist_attr_id].modified == approx(0)
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(resist_immunity=True),
        passing=[],
        failing=[eve_src_charge_id])
    # Action
    api_tgt_item.change_drone(mutation={eve_resist_attr_id: Muta.roll_to_api(val=0.1)})
    # Verification
    assert api_tgt_item.update().attrs[eve_resist_attr_id].modified == approx(0.2)
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(resist_immunity=True),
        passing=[eve_src_charge_id],
        failing=[])
    # Action
    api_tgt_item.change_drone(mutation=None)
    # Verification
    assert api_tgt_item.update().attrs[eve_resist_attr_id].modified == approx(1)
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(resist_immunity=True),
        passing=[eve_src_charge_id],
        failing=[])


def test_criterion_effect_cat(client, consts):
    # Only targeted effects are subject for the validation
    eve_resist_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect1_id = client.mk_eve_effect(
        # AoE web just to enforce effect have some buffs, to let it go through generic projection
        # filters and reach validation service
        id_=consts.EveEffect.doomsday_aoe_web,
        cat_id=consts.EveEffCat.active,
        resist_attr_id=eve_resist_attr_id)
    eve_src_effect2_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, resist_attr_id=eve_resist_attr_id)
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_src_charge1_id = client.mk_eve_charge(eff_ids=[eve_src_effect1_id], defeff_id=eve_src_effect1_id)
    eve_src_charge2_id = client.mk_eve_charge(eff_ids=[eve_src_effect2_id], defeff_id=eve_src_effect2_id)
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_resist_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_launcher = api_src_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    api_src_launcher.change_module(add_proj_item_ids=[api_tgt_item.id])
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(resist_immunity=True),
        passing=[eve_src_charge1_id],
        failing=[eve_src_charge2_id])


def test_no_attr(client, consts):
    eve_resist_attr_id = client.alloc_attr_id()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, resist_attr_id=eve_resist_attr_id)
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_src_charge_id = client.mk_eve_charge(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_resist_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_launcher = api_src_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    api_src_launcher.change_module(add_proj_item_ids=[api_tgt_item.id])
    # Verification - if resistance attribute cannot be found, assume target is not immune to
    # modifications
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(resist_immunity=True),
        passing=[eve_src_charge_id],
        failing=[])


def test_known_failures(client, consts):
    eve_resist_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, resist_attr_id=eve_resist_attr_id)
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_src_charge_id = client.mk_eve_charge(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_other_charge_id = client.mk_eve_charge()
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_resist_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    api_src_fit = api_sol.create_fit()
    api_src_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active, proj_item_ids=[api_tgt_item.id])
    api_src_launcher1 = api_src_fit.add_module(
        type_id=eve_launcher_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_src_charge_id,
        proj_item_ids=[api_tgt_item.id])
    api_src_launcher2 = api_src_fit.add_module(
        type_id=eve_launcher_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_src_charge_id,
        proj_item_ids=[api_tgt_item.id])
    # Verification - charges are tried in the module without charge
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(resist_immunity=[api_src_launcher1.charge.id]),
        passing=[],
        failing=[eve_other_charge_id, eve_src_charge_id])
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(resist_immunity=[api_src_launcher2.charge.id]),
        passing=[],
        failing=[eve_other_charge_id, eve_src_charge_id])
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(resist_immunity=[api_src_launcher1.charge.id, api_src_launcher2.charge.id]),
        passing=[eve_other_charge_id],
        failing=[eve_src_charge_id])


def test_not_loaded_tgt(client, consts):
    eve_resist_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, resist_attr_id=eve_resist_attr_id)
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_src_charge_id = client.mk_eve_charge(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_item_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_launcher = api_src_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    api_src_launcher.change_module(add_proj_item_ids=[api_tgt_item.id])
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(resist_immunity=True),
        passing=[eve_src_charge_id],
        failing=[])
