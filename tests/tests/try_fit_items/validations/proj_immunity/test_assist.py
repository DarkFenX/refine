from fw import Muta, approx
from fw.api import ValOptions


def test_src_module_tgt_ship_project_unproject(client, consts):
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_assistance)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_assistance=True)
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_src_charge_id = client.mk_eve_charge(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_other_charge_id = client.mk_eve_charge()
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_launcher = api_src_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[eve_src_charge_id, eve_other_charge_id],
        failing=[])
    # Action
    api_src_launcher.change_module(add_proj_item_ids=[api_tgt_item.id])
    # Verification - charges are projected wherever the module they are loaded into is projected
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[eve_other_charge_id],
        failing=[eve_src_charge_id])
    # Action
    api_src_launcher.change_module(rm_proj_item_ids=[api_tgt_item.id])
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[eve_src_charge_id, eve_other_charge_id],
        failing=[])


def test_tgt_drone_fighter(client, consts):
    # Also check multiple projectee items
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_assistance)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_assistance=True)
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_src_charge_id = client.mk_eve_charge(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_item1_id = client.mk_eve_item(attrs={eve_immunity_attr_id: 1})
    eve_tgt_item2_id = client.mk_eve_item(attrs={eve_immunity_attr_id: 0})
    eve_tgt_blocker_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_launcher = api_src_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_fit.add_module(type_id=eve_tgt_blocker_id, state=consts.ApiModuleState.active)
    api_tgt_drone = api_tgt_fit.add_drone(type_id=eve_tgt_item1_id)
    api_tgt_fighter = api_tgt_fit.add_fighter(type_id=eve_tgt_item1_id)
    api_src_launcher.change_module(add_proj_item_ids=[api_tgt_drone.id, api_tgt_fighter.id])
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[],
        failing=[eve_src_charge_id])
    # Action
    api_tgt_drone.change_drone(type_id=eve_tgt_item2_id)
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[],
        failing=[eve_src_charge_id])
    # Action
    api_tgt_fighter.change_fighter(type_id=eve_tgt_item2_id)
    # Verification - check passes for minions despite fit having assist blocker module
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[eve_src_charge_id],
        failing=[])


def test_tgt_ship_blocker(client, consts):
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_assistance)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_assistance=True)
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_src_charge_id = client.mk_eve_charge(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_blocker_id = client.mk_eve_item(attrs={eve_immunity_attr_id: 1})
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    api_tgt_blocker = api_tgt_fit.add_module(type_id=eve_blocker_id, state=consts.ApiModuleState.online)
    api_src_fit = api_sol.create_fit()
    api_src_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active, proj_item_ids=[api_tgt_item.id])
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[eve_src_charge_id],
        failing=[])
    # Action
    api_tgt_blocker.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[],
        failing=[eve_src_charge_id])
    # Action
    api_tgt_blocker.remove()
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[eve_src_charge_id],
        failing=[])


def test_tgt_flag_values(client, consts):
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_assistance)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_assistance=True)
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_src_charge_id = client.mk_eve_charge(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_item1_id = client.mk_eve_item(attrs={eve_immunity_attr_id: -1})
    eve_tgt_item2_id = client.mk_eve_item(attrs={eve_immunity_attr_id: 0})
    eve_tgt_item3_id = client.mk_eve_item(attrs={eve_immunity_attr_id: 0.1})
    eve_tgt_item4_id = client.mk_eve_item(attrs={eve_immunity_attr_id: 50.3})
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
    api_src_launcher.change_module(add_proj_item_ids=[api_tgt_item2.id, api_tgt_item5.id])
    # Verification - no attr or value 0 doesn't fail validation
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[eve_src_charge_id],
        failing=[])
    # Action
    api_src_launcher.change_module(add_proj_item_ids=[api_tgt_item1.id])
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[],
        failing=[eve_src_charge_id])
    # Action
    api_src_launcher.change_module(rm_proj_item_ids=[api_tgt_item1.id], add_proj_item_ids=[api_tgt_item3.id])
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[],
        failing=[eve_src_charge_id])
    # Action
    api_src_launcher.change_module(rm_proj_item_ids=[api_tgt_item3.id], add_proj_item_ids=[api_tgt_item4.id])
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[],
        failing=[eve_src_charge_id])


def test_blocker_flag_values(client, consts):
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_assistance, def_val=1)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_assistance=True)
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_src_charge_id = client.mk_eve_charge(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_blocker1_id = client.mk_eve_item(attrs={eve_immunity_attr_id: -1})
    eve_blocker2_id = client.mk_eve_item(attrs={eve_immunity_attr_id: 0})
    eve_blocker3_id = client.mk_eve_item(attrs={eve_immunity_attr_id: 0.1})
    eve_blocker4_id = client.mk_eve_item(attrs={eve_immunity_attr_id: 50.3})
    eve_blocker5_id = client.mk_eve_item()
    eve_tgt_ship_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_launcher = api_src_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id)
    api_tgt_blocker = api_tgt_fit.add_module(type_id=eve_blocker1_id, state=consts.ApiModuleState.active)
    api_src_launcher.change_module(add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[],
        failing=[eve_src_charge_id])
    # Action
    api_tgt_blocker.change_module(type_id=eve_blocker2_id)
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[eve_src_charge_id],
        failing=[])
    # Action
    api_tgt_blocker.change_module(type_id=eve_blocker3_id)
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[],
        failing=[eve_src_charge_id])
    # Action
    api_tgt_blocker.change_module(type_id=eve_blocker4_id)
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[],
        failing=[eve_src_charge_id])
    # Action
    api_tgt_blocker.change_module(type_id=eve_blocker5_id)
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[eve_src_charge_id],
        failing=[])


def test_tgt_modified(client, consts):
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_assistance)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_assistance=True)
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_src_charge_id = client.mk_eve_charge(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 0})
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_immunity_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_mod_item_id = client.mk_eve_item(attrs={eve_mod_attr_id: 1}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_launcher = api_src_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    api_src_launcher.change_module(add_proj_item_ids=[api_tgt_item.id])
    # Verification
    assert api_tgt_item.update().attrs[eve_immunity_attr_id].modified == approx(0)
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[eve_src_charge_id],
        failing=[])
    # Action
    api_mod_item = api_tgt_fit.add_module(type_id=eve_mod_item_id)
    # Verification
    assert api_tgt_item.update().attrs[eve_immunity_attr_id].modified == approx(1)
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[],
        failing=[eve_src_charge_id])
    # Action
    api_mod_item.remove()
    # Verification
    assert api_tgt_item.update().attrs[eve_immunity_attr_id].modified == approx(0)
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[eve_src_charge_id],
        failing=[])


def test_tgt_mutation(client, consts):
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_assistance)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_assistance=True)
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_src_charge_id = client.mk_eve_charge(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_base_item_id = client.mk_eve_item(attrs={eve_immunity_attr_id: 1})
    eve_tgt_mutated_item_id = client.mk_eve_item(attrs={eve_immunity_attr_id: 0})
    eve_tgt_mutator_id = client.mk_eve_mutator(
        items=[([eve_tgt_base_item_id], eve_tgt_mutated_item_id)],
        attrs={eve_immunity_attr_id: (0, 2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_launcher = api_src_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.add_drone(type_id=eve_tgt_base_item_id)
    api_src_launcher.change_module(add_proj_item_ids=[api_tgt_item.id])
    # Verification
    assert api_tgt_item.update().attrs[eve_immunity_attr_id].modified == approx(1)
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[],
        failing=[eve_src_charge_id])
    # Action
    api_tgt_item.change_drone(mutation=eve_tgt_mutator_id)
    # Verification
    assert api_tgt_item.update().attrs[eve_immunity_attr_id].modified == approx(1)
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[],
        failing=[eve_src_charge_id])
    # Action
    api_tgt_item.change_drone(mutation={eve_immunity_attr_id: Muta.roll_to_api(val=0)})
    # Verification
    assert api_tgt_item.update().attrs[eve_immunity_attr_id].modified == approx(0)
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[eve_src_charge_id],
        failing=[])
    # Action
    api_tgt_item.change_drone(mutation={eve_immunity_attr_id: Muta.roll_to_api(val=0.1)})
    # Verification
    assert api_tgt_item.update().attrs[eve_immunity_attr_id].modified == approx(0.2)
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[],
        failing=[eve_src_charge_id])
    # Action
    api_tgt_item.change_drone(mutation=None)
    # Verification
    assert api_tgt_item.update().attrs[eve_immunity_attr_id].modified == approx(1)
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[],
        failing=[eve_src_charge_id])


def test_criterion_not_assist(client, consts):
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_assistance)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect1_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_assistance=False)
    eve_src_effect2_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_assistance=True)
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_src_charge1_id = client.mk_eve_charge(eff_ids=[eve_src_effect1_id], defeff_id=eve_src_effect1_id)
    eve_src_charge2_id = client.mk_eve_charge(eff_ids=[eve_src_effect2_id], defeff_id=eve_src_effect2_id)
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_launcher = api_src_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    api_src_launcher.change_module(add_proj_item_ids=[api_tgt_item.id])
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[eve_src_charge1_id],
        failing=[eve_src_charge2_id])


def test_criterion_effect_cat(client, consts):
    # Only targeted effects are subject for the validation
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_assistance)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect1_id = client.mk_eve_effect(
        # AoE web just to enforce effect have some buffs, to let it go through generic projection
        # filters and reach validation service
        id_=consts.EveEffect.doomsday_aoe_web,
        cat_id=consts.EveEffCat.active,
        is_assistance=True)
    eve_src_effect2_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_assistance=True)
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_src_charge1_id = client.mk_eve_charge(eff_ids=[eve_src_effect1_id], defeff_id=eve_src_effect1_id)
    eve_src_charge2_id = client.mk_eve_charge(eff_ids=[eve_src_effect2_id], defeff_id=eve_src_effect2_id)
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_launcher = api_src_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    api_src_launcher.change_module(add_proj_item_ids=[api_tgt_item.id])
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[eve_src_charge1_id],
        failing=[eve_src_charge2_id])


def test_known_failures(client, consts):
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_assistance)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_assistance=True)
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_src_charge_id = client.mk_eve_charge(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_other_charge_id = client.mk_eve_charge()
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 1})
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
        val_options=ValOptions(assist_immunity=[api_src_launcher1.charge.id]),
        passing=[],
        failing=[eve_other_charge_id, eve_src_charge_id])
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=[api_src_launcher2.charge.id]),
        passing=[],
        failing=[eve_other_charge_id, eve_src_charge_id])
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=[api_src_launcher1.charge.id, api_src_launcher2.charge.id]),
        passing=[eve_other_charge_id],
        failing=[eve_src_charge_id])


def test_assist_immunity(client, consts):
    # Check that target flagged as offense modifier immune doesn't trigger this validation
    eve_immunity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_assistance=True)
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_src_charge_id = client.mk_eve_charge(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_launcher = api_src_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    api_src_launcher.change_module(add_proj_item_ids=[api_tgt_item.id])
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[eve_src_charge_id],
        failing=[])


def test_no_attr(client, consts):
    eve_immunity_attr_id = consts.EveAttr.disallow_assistance
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_assistance=True)
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_src_charge_id = client.mk_eve_charge(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_tgt_item_id = client.mk_eve_ship(attrs={eve_immunity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_launcher = api_src_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item_id)
    api_src_launcher.change_module(add_proj_item_ids=[api_tgt_item.id])
    # Verification - if attribute cannot be found, the validation passes regardless of defined value
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(assist_immunity=True),
        passing=[eve_src_charge_id],
        failing=[])


def test_not_loaded_tgt(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.disallow_assistance)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, is_assistance=True)
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
        val_options=ValOptions(assist_immunity=True),
        passing=[eve_src_charge_id],
        failing=[])
