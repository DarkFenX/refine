from fw.api import ValOptions


def test_known_failures(client, consts):
    eve_slot_effect_id = client.mk_eve_effect(id_=consts.EveEffect.hi_power)
    eve_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_slot_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.online)
    api_module2 = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.online)
    # Verification - modules are tried in a state they can have, offline for modules without online
    # effect
    api_fit.check_try_fit_items(
        val_options=ValOptions(module_state=[api_module1.id]),
        passing=[],
        failing=[eve_module_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(module_state=[api_module2.id]),
        passing=[],
        failing=[eve_module_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(module_state=[api_module1.id, api_module2.id]),
        passing=[eve_module_id],
        failing=[])
