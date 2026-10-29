from fw.api import ValOptions


def test_known_failures(client, consts):
    eve_cloak_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.can_cloak)
    eve_cloak_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_cloaking)
    eve_cloak_effect_id = client.mk_eve_effect(id_=consts.EveEffect.cloaking, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_module(eff_ids=[eve_cloak_effect_id], defeff_id=eve_cloak_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_cloak_attr1_id: 0, eve_cloak_attr2_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_module2 = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification - modules are tried at most online
    api_fit.check_try_fit_items(
        val_options=ValOptions(cloaking_blocked=[api_module1.id]),
        passing=[],
        failing=[eve_module_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(cloaking_blocked=[api_module2.id]),
        passing=[],
        failing=[eve_module_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(cloaking_blocked=[api_module1.id, api_module2.id]),
        passing=[eve_module_id],
        failing=[])
