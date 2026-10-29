from fw import check_no_field
from fw.api import ValOptions


def test_skipped_types(client, consts):
    eve_booster_id = client.mk_eve_booster()
    eve_drone_id = client.mk_eve_drone()
    eve_fighter_id = client.mk_eve_fighter()
    eve_implant_id = client.mk_eve_implant()
    eve_module_id = client.mk_eve_module()
    eve_rig_id = client.mk_eve_rig()
    eve_service_id = client.mk_eve_service()
    eve_subsystem_id = client.mk_eve_subsystem()
    eve_char_id = client.mk_eve_item(grp_id=consts.EveItemGrp.character)
    eve_ship_id = client.mk_eve_ship()
    eve_skill_id = client.mk_eve_item(cat_id=consts.EveItemCat.skill)
    eve_stance_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, grp_id=consts.EveItemGrp.ship_modifier)
    eve_undetectable_id = client.mk_eve_item()
    eve_unknown_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification - with no validations enabled, items which are tried pass, while unknown items,
    # items of undetectable kind and items of kinds which are not tried fail
    api_fit.check_try_fit_items(
        val_options=ValOptions(),
        passing=[
            eve_booster_id,
            eve_drone_id,
            eve_fighter_id,
            eve_implant_id,
            eve_module_id,
            eve_rig_id,
            eve_service_id,
            eve_subsystem_id],
        failing=[eve_char_id, eve_ship_id, eve_skill_id, eve_stance_id, eve_undetectable_id, eve_unknown_id])


def test_module_without_online_effect(client, consts):
    eve_grp_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_group_online)
    eve_slot_effect_id = client.mk_eve_effect(id_=consts.EveEffect.hi_power)
    eve_module1_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.module,
        grp_id=eve_grp_id,
        attrs={eve_limit_attr_id: 1},
        eff_ids=[eve_slot_effect_id])
    eve_module2_id = client.mk_eve_module(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.online)
    # Verification - modules are tried in the highest state they can have, capped at online; module
    # without online effect is tried offline, and does not count towards the online limit
    api_fit.check_try_fit_items(
        val_options=ValOptions(max_group_online=True),
        passing=[eve_module1_id],
        failing=[eve_module2_id])


def test_fit_unchanged(client, consts):
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_booster_id = client.mk_eve_booster()
    eve_charge1_id = client.mk_eve_charge()
    eve_charge2_id = client.mk_eve_charge()
    eve_drone_id = client.mk_eve_drone()
    eve_fighter_id = client.mk_eve_fighter()
    eve_implant_id = client.mk_eve_implant()
    eve_module_id = client.mk_eve_module()
    eve_rig_id = client.mk_eve_rig()
    eve_service_id = client.mk_eve_service()
    eve_subsystem_id = client.mk_eve_subsystem()
    eve_launcher_id = client.mk_eve_module(attrs={eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_launcher = api_fit.add_module(type_id=eve_launcher_id, charge_type_id=eve_charge1_id)
    # Verification - tried items are removed from fit after trying them
    api_fit.check_try_fit_items(
        val_options=ValOptions(),
        passing=[
            eve_booster_id,
            eve_charge2_id,
            eve_drone_id,
            eve_fighter_id,
            eve_implant_id,
            eve_module_id,
            eve_rig_id,
            eve_service_id,
            eve_subsystem_id],
        failing=[])
    api_fit.update()
    assert len(api_fit.modules) == 1
    assert len(api_fit.modules.high) == 1
    assert api_fit.modules.high[0].id == api_launcher.id
    assert api_launcher.update().charge.type_id == eve_charge1_id
    with check_no_field():
        api_fit.boosters  # ruff:ignore[useless-expression]
    with check_no_field():
        api_fit.drones  # ruff:ignore[useless-expression]
    with check_no_field():
        api_fit.fighters  # ruff:ignore[useless-expression]
    with check_no_field():
        api_fit.implants  # ruff:ignore[useless-expression]
    with check_no_field():
        api_fit.rigs  # ruff:ignore[useless-expression]
    with check_no_field():
        api_fit.services  # ruff:ignore[useless-expression]
    with check_no_field():
        api_fit.subsystems  # ruff:ignore[useless-expression]
