from fw.api import ValOptions


def test_kfs_replaced(client, consts):
    # Known failures of a charge loaded into a module should not apply to charges tried in its place
    eve_ban_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge1_id = client.mk_eve_charge()
    eve_charge2_id = client.mk_eve_charge(attrs={eve_ban_attr_id: 1})
    eve_charge3_id = client.mk_eve_charge(attrs={eve_ban_attr_id: 1})
    eve_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        charge_type_id=eve_charge2_id,
        state=consts.ApiModuleState.disabled)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=[api_module.charge.id]),
        passing=[eve_charge1_id],
        failing=[eve_charge2_id, eve_charge3_id])


def test_kfs_restored(client, consts):
    # Charge which is put back after trying charges in its module should keep its known failures
    eve_grp1_id = client.mk_eve_charge_group()
    eve_grp2_id = client.mk_eve_charge_group()
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_group1)
    eve_ban_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge1_id = client.mk_eve_charge(grp_id=eve_grp1_id, attrs={eve_ban_attr_id: 1})
    eve_charge2_id = client.mk_eve_charge(grp_id=eve_grp2_id)
    eve_charge3_id = client.mk_eve_charge(grp_id=eve_grp1_id, attrs={eve_ban_attr_id: 1})
    eve_module1_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_grp1_id, eve_capacity_attr_id: 1})
    eve_module2_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_grp2_id, eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(
        type_id=eve_module1_id,
        charge_type_id=eve_charge1_id,
        state=consts.ApiModuleState.disabled)
    api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.disabled)
    # Verification - charge 2 can be loaded only into module 2, which is tried after charge 1 is
    # put back into module 1
    api_fit.check_try_fit_items(
        val_options=ValOptions(charge_group=True, sec_zone_unactivable=[api_module1.charge.id]),
        passing=[eve_charge2_id],
        failing=[eve_charge3_id])


def test_fitted_modules(client, consts):
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_id = client.mk_eve_charge()
    eve_launcher_id = client.mk_eve_module(attrs={eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification - charges are tried only in fitted modules, tried modules are removed before
    # charges are tried
    api_fit.check_try_fit_items(val_options=ValOptions(), passing=[eve_launcher_id], failing=[eve_charge_id])
    # Action
    api_fit.add_module(type_id=eve_launcher_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(), passing=[eve_launcher_id, eve_charge_id], failing=[])


def test_module_capacity(client, consts):
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_id = client.mk_eve_charge()
    eve_module1_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 0})
    eve_module2_id = client.mk_eve_item(attrs={eve_capacity_attr_id: -1})
    eve_module3_id = client.mk_eve_item()
    eve_module4_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 0.1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module1_id)
    api_fit.add_module(type_id=eve_module2_id)
    api_fit.add_module(type_id=eve_module3_id)
    # Verification - charges are tried only in modules with positive capacity
    api_fit.check_try_fit_items(val_options=ValOptions(), passing=[], failing=[eve_charge_id])
    # Action
    api_fit.add_module(type_id=eve_module4_id)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(), passing=[eve_charge_id], failing=[])


def test_module_types(client, consts):
    eve_grp1_id = client.mk_eve_charge_group()
    eve_grp2_id = client.mk_eve_charge_group()
    eve_grp3_id = client.mk_eve_charge_group()
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_group1)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge1_id = client.mk_eve_charge(grp_id=eve_grp1_id)
    eve_charge2_id = client.mk_eve_charge(grp_id=eve_grp2_id)
    eve_charge3_id = client.mk_eve_charge(grp_id=eve_grp3_id)
    eve_module1_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_grp1_id, eve_capacity_attr_id: 1})
    eve_module2_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_grp2_id, eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module1_id)
    api_fit.add_module(type_id=eve_module2_id)
    # Verification - charges are tried in modules of every type, until one of them accepts it
    api_fit.check_try_fit_items(
        val_options=ValOptions(charge_group=True),
        passing=[eve_charge1_id, eve_charge2_id],
        failing=[eve_charge3_id])


def test_replaced(client, consts):
    eve_ban_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge1_id = client.mk_eve_charge(attrs={eve_ban_attr_id: 1})
    eve_charge2_id = client.mk_eve_charge()
    eve_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    # Verification - tried charges replace the loaded one, so its failure does not affect them, and
    # it is put back afterwards
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=True),
        passing=[eve_charge2_id],
        failing=[eve_charge1_id])
    assert api_module.update().charge.type_id == eve_charge1_id


def test_restored(client, consts):
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_effect_id = client.mk_eve_effect()
    eve_charge1_id = client.mk_eve_charge(eff_ids=[eve_effect_id])
    eve_charge2_id = client.mk_eve_charge()
    eve_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    api_charge = api_module.charge
    api_charge.change_charge(state=False, effect_modes={eve_effect_id: consts.ApiEffMode.force_run})
    # Verification - charge which is put back after trying charges in its module keeps its item ID
    # and settings
    api_fit.check_try_fit_items(val_options=ValOptions(), passing=[eve_charge2_id], failing=[])
    assert api_module.update().charge.id == api_charge.id
    api_charge.update()
    assert api_charge.type_id == eve_charge1_id
    assert api_charge.state is False
    assert api_charge.effects[eve_effect_id].mode == consts.ApiEffMode.force_run


def test_charged_module(client, consts):
    eve_ban_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge1_id = client.mk_eve_charge(attrs={eve_ban_attr_id: 1})
    eve_charge2_id = client.mk_eve_charge()
    eve_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id)
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    # Verification - charges are tried in every module, so also in the one with a failing charge,
    # which they replace
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=True),
        passing=[eve_charge2_id],
        failing=[eve_charge1_id])
    assert api_module.update().charge.type_id == eve_charge1_id


def test_module_states(client, consts):
    # Tried charges inherit state of the module they are loaded into
    eve_ban_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.activates_charge, cat_id=consts.EveEffCat.active)
    eve_charge1_id = client.mk_eve_charge()
    eve_charge2_id = client.mk_eve_charge(attrs={eve_ban_attr_id: 1})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_module2 = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.online)
    # Verification - charges are tried in every module, even if there is another one with the same
    # type ID
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_active=True),
        passing=[eve_charge1_id, eve_charge2_id],
        failing=[])
    # Action
    api_module2.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_active=True),
        passing=[eve_charge1_id],
        failing=[eve_charge2_id])


def test_module_projections(client, consts):
    # Tried charges are projected wherever the module they are loaded into is projected
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_charge_effect_id = client.mk_eve_effect(id_=consts.EveEffect.super_weapon_amarr, cat_id=consts.EveEffCat.target)
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_charge_id = client.mk_eve_charge(eff_ids=[eve_charge_effect_id], defeff_id=eve_charge_effect_id)
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_launcher1 = api_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active)
    api_launcher2 = api_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active)
    api_ship = api_sol.create_fit().set_ship(type_id=eve_ship_id)
    api_launcher1.change_module(add_proj_item_ids=[api_ship.id])
    # Verification - charges are tried in every module, even if there is another one with the same
    # type ID
    api_fit.check_try_fit_items(val_options=ValOptions(projectee_filter=True), passing=[eve_charge_id], failing=[])
    # Action
    api_launcher2.change_module(add_proj_item_ids=[api_ship.id])
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(projectee_filter=True), passing=[], failing=[eve_charge_id])
