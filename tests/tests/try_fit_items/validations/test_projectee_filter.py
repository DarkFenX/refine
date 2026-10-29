from fw.api import ValOptions


def test_src_direct_dd(client, consts):
    # Also check multiple projectee items
    eve_capships_skill_id = client.mk_eve_item(id_=consts.EveItem.capital_ships)
    eve_other_skill_id = client.mk_eve_item()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect_id = client.mk_eve_effect(id_=consts.EveEffect.super_weapon_amarr, cat_id=consts.EveEffCat.target)
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_src_charge_id = client.mk_eve_charge(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_other_charge_id = client.mk_eve_charge()
    eve_cap_ship_id = client.mk_eve_ship(srqs={eve_capships_skill_id: 1})
    eve_freighter_id = client.mk_eve_ship(grp_id=consts.EveItemGrp.freighter)
    eve_jf_id = client.mk_eve_ship(grp_id=consts.EveItemGrp.jump_freighter)
    eve_other_ship1_id = client.mk_eve_ship()
    eve_other_ship2_id = client.mk_eve_ship(srqs={eve_other_skill_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_launcher = api_src_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active)
    api_cap_ship = api_sol.create_fit().set_ship(type_id=eve_cap_ship_id)
    api_freighter = api_sol.create_fit().set_ship(type_id=eve_freighter_id)
    api_jf = api_sol.create_fit().set_ship(type_id=eve_jf_id)
    api_other_ship1 = api_sol.create_fit().set_ship(type_id=eve_other_ship1_id)
    api_other_ship2 = api_sol.create_fit().set_ship(type_id=eve_other_ship2_id)
    api_src_launcher.change_module(add_proj_item_ids=[api_other_ship1.id])
    # Verification - charges are projected wherever the module they are loaded into is projected
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(projectee_filter=True),
        passing=[eve_other_charge_id],
        failing=[eve_src_charge_id])
    # Action
    api_src_launcher.change_module(add_proj_item_ids=[api_cap_ship.id, api_freighter.id, api_jf.id, api_other_ship2.id])
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(projectee_filter=True),
        passing=[eve_other_charge_id],
        failing=[eve_src_charge_id])
    # Action
    api_src_launcher.change_module(rm_proj_item_ids=[api_other_ship1.id, api_other_ship2.id])
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(projectee_filter=True),
        passing=[eve_src_charge_id, eve_other_charge_id],
        failing=[])


def test_src_standup_vorton(client, consts):
    eve_tgt_list_attr_id = client.mk_eve_attr(id_=consts.EveAttr.tgt_filter_typelist_id)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect_id = client.mk_eve_effect(id_=consts.EveEffect.lightning_weapon, cat_id=consts.EveEffCat.target)
    eve_ship1_id = client.mk_eve_ship()
    eve_ship2_id = client.mk_eve_ship()
    eve_item_list_id = client.mk_eve_item_list(inc_type_ids=[eve_ship2_id])
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_src_charge_id = client.mk_eve_charge(
        attrs={eve_tgt_list_attr_id: eve_item_list_id},
        eff_ids=[eve_src_effect_id],
        defeff_id=eve_src_effect_id)
    eve_other_charge_id = client.mk_eve_charge()
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_launcher = api_src_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship1_id)
    api_src_launcher.change_module(add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(projectee_filter=True),
        passing=[eve_other_charge_id],
        failing=[eve_src_charge_id])
    # Action
    api_tgt_ship.change_ship(type_id=eve_ship2_id)
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(projectee_filter=True),
        passing=[eve_src_charge_id, eve_other_charge_id],
        failing=[])


def test_filter_reference_values(client, consts):
    eve_tgt_list_attr_id = client.mk_eve_attr(id_=consts.EveAttr.tgt_filter_typelist_id)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect_id = client.mk_eve_effect(id_=consts.EveEffect.lightning_weapon, cat_id=consts.EveEffCat.target)
    eve_ship_id = client.mk_eve_ship()
    eve_item_list_id = client.mk_eve_item_list(inc_type_ids=[eve_ship_id])
    client.mk_eve_item_list(id_=eve_item_list_id - 1)
    client.mk_eve_item_list(id_=eve_item_list_id + 1)
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_charge1_id = client.mk_eve_charge(
        attrs={eve_tgt_list_attr_id: eve_item_list_id - 0.6},
        eff_ids=[eve_src_effect_id],
        defeff_id=eve_src_effect_id)
    eve_charge2_id = client.mk_eve_charge(
        attrs={eve_tgt_list_attr_id: eve_item_list_id - 0.4},
        eff_ids=[eve_src_effect_id],
        defeff_id=eve_src_effect_id)
    eve_charge3_id = client.mk_eve_charge(
        attrs={eve_tgt_list_attr_id: eve_item_list_id + 0.4},
        eff_ids=[eve_src_effect_id],
        defeff_id=eve_src_effect_id)
    eve_charge4_id = client.mk_eve_charge(
        attrs={eve_tgt_list_attr_id: eve_item_list_id + 0.6},
        eff_ids=[eve_src_effect_id],
        defeff_id=eve_src_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_launcher = api_src_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_src_launcher.change_module(add_proj_item_ids=[api_tgt_ship.id])
    # Verification - ID is rounded to the closest value, which is the type list ID for charges 2 and
    # 3, and empty lists for charges 1 and 4
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(projectee_filter=True),
        passing=[eve_charge2_id, eve_charge3_id],
        failing=[eve_charge1_id, eve_charge4_id])


def test_empty_list(client, consts):
    eve_tgt_list_attr_id = client.mk_eve_attr(id_=consts.EveAttr.tgt_filter_typelist_id)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect_id = client.mk_eve_effect(id_=consts.EveEffect.lightning_weapon, cat_id=consts.EveEffCat.target)
    eve_ship_id = client.mk_eve_ship()
    eve_item_list_id = client.mk_eve_item_list()
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_src_charge_id = client.mk_eve_charge(
        attrs={eve_tgt_list_attr_id: eve_item_list_id},
        eff_ids=[eve_src_effect_id],
        defeff_id=eve_src_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_launcher = api_src_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_src_launcher.change_module(add_proj_item_ids=[api_tgt_ship.id])
    # Verification - if list is defined and fetched but is empty, validation fails
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(projectee_filter=True),
        passing=[],
        failing=[eve_src_charge_id])


def test_tgt_mutation(client, consts):
    eve_tgt_list_attr_id = client.mk_eve_attr(id_=consts.EveAttr.tgt_filter_typelist_id)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect_id = client.mk_eve_effect(id_=consts.EveEffect.lightning_weapon, cat_id=consts.EveEffCat.target)
    eve_tgt_base_drone_id = client.mk_eve_drone()
    eve_tgt_mutated_drone_id = client.mk_eve_drone()
    eve_tgt_mutator_id = client.mk_eve_mutator(items=[([eve_tgt_base_drone_id], eve_tgt_mutated_drone_id)])
    eve_item_list1_id = client.mk_eve_item_list(inc_type_ids=[eve_tgt_base_drone_id])
    eve_item_list2_id = client.mk_eve_item_list(inc_type_ids=[eve_tgt_mutated_drone_id])
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_charge1_id = client.mk_eve_charge(
        attrs={eve_tgt_list_attr_id: eve_item_list1_id},
        eff_ids=[eve_src_effect_id],
        defeff_id=eve_src_effect_id)
    eve_charge2_id = client.mk_eve_charge(
        attrs={eve_tgt_list_attr_id: eve_item_list2_id},
        eff_ids=[eve_src_effect_id],
        defeff_id=eve_src_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_launcher = api_src_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_drone = api_tgt_fit.add_drone(type_id=eve_tgt_base_drone_id)
    api_src_launcher.change_module(add_proj_item_ids=[api_tgt_drone.id])
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(projectee_filter=True),
        passing=[eve_charge1_id],
        failing=[eve_charge2_id])
    # Action
    api_tgt_drone.change_drone(mutation=eve_tgt_mutator_id)
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(projectee_filter=True),
        passing=[eve_charge2_id],
        failing=[eve_charge1_id])
    # Action
    api_tgt_drone.change_drone(mutation=None)
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(projectee_filter=True),
        passing=[eve_charge1_id],
        failing=[eve_charge2_id])


def test_known_failures(client, consts):
    eve_tgt_list_attr_id = client.mk_eve_attr(id_=consts.EveAttr.tgt_filter_typelist_id)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect_id = client.mk_eve_effect(id_=consts.EveEffect.lightning_weapon, cat_id=consts.EveEffCat.target)
    eve_tgt_item_id = client.mk_eve_ship()
    eve_item_list_id = client.mk_eve_item_list()
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_src_charge_id = client.mk_eve_charge(
        attrs={eve_tgt_list_attr_id: eve_item_list_id},
        eff_ids=[eve_src_effect_id],
        defeff_id=eve_src_effect_id)
    eve_other_charge_id = client.mk_eve_charge()
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
        val_options=ValOptions(projectee_filter=[api_src_launcher1.charge.id]),
        passing=[],
        failing=[eve_other_charge_id, eve_src_charge_id])
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(projectee_filter=[api_src_launcher2.charge.id]),
        passing=[],
        failing=[eve_other_charge_id, eve_src_charge_id])
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(projectee_filter=[api_src_launcher1.charge.id, api_src_launcher2.charge.id]),
        passing=[eve_other_charge_id],
        failing=[eve_src_charge_id])


def test_no_attr(client, consts):
    eve_tgt_list_attr_id = consts.EveAttr.tgt_filter_typelist_id
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect_id = client.mk_eve_effect(id_=consts.EveEffect.lightning_weapon, cat_id=consts.EveEffCat.target)
    eve_ship1_id = client.mk_eve_ship()
    eve_ship2_id = client.mk_eve_ship()
    eve_item_list_id = client.mk_eve_item_list(inc_type_ids=[eve_ship2_id])
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_src_charge_id = client.mk_eve_charge(
        attrs={eve_tgt_list_attr_id: eve_item_list_id},
        eff_ids=[eve_src_effect_id],
        defeff_id=eve_src_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_launcher = api_src_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship1_id)
    api_src_launcher.change_module(add_proj_item_ids=[api_tgt_ship.id])
    # Verification - if attribute cannot be found, the validation passes
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(projectee_filter=True),
        passing=[eve_src_charge_id],
        failing=[])


def test_not_loaded_tgt(client, consts):
    eve_tgt_list_attr_id = client.mk_eve_attr(id_=consts.EveAttr.tgt_filter_typelist_id)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_src_effect_id = client.mk_eve_effect(id_=consts.EveEffect.lightning_weapon, cat_id=consts.EveEffCat.target)
    eve_tgt_item1_id = client.alloc_item_id()
    eve_tgt_item2_id = client.alloc_item_id()
    eve_item_list_id = client.mk_eve_item_list(inc_type_ids=[eve_tgt_item2_id])
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_src_charge_id = client.mk_eve_charge(
        attrs={eve_tgt_list_attr_id: eve_item_list_id},
        eff_ids=[eve_src_effect_id],
        defeff_id=eve_src_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_launcher = api_src_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_item = api_tgt_fit.set_ship(type_id=eve_tgt_item1_id)
    api_src_launcher.change_module(add_proj_item_ids=[api_tgt_item.id])
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(projectee_filter=True),
        passing=[],
        failing=[eve_src_charge_id])
    # Action
    api_tgt_item.change_ship(type_id=eve_tgt_item2_id)
    # Verification
    api_src_fit.check_try_fit_items(
        val_options=ValOptions(projectee_filter=True),
        passing=[eve_src_charge_id],
        failing=[])
