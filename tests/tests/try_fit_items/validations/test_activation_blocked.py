from fw.api import ValOptions


def test_known_failures(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_module(attrs={eve_attr_id: 1}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_module2 = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification - modules are tried at most online
    api_fit.check_try_fit_items(
        val_options=ValOptions(activation_blocked=[api_module1.id]),
        passing=[],
        failing=[eve_module_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(activation_blocked=[api_module2.id]),
        passing=[],
        failing=[eve_module_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(activation_blocked=[api_module1.id, api_module2.id]),
        passing=[eve_module_id],
        failing=[])
