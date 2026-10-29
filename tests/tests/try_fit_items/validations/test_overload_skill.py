from fw.api import ValOptions


def test_known_failures(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.required_thermodynamics_skill)
    eve_module_id = client.mk_eve_module(attrs={eve_attr_id: 5})
    eve_skill_id = client.mk_eve_item(id_=consts.EveItem.thermodynamics)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_skill(type_id=eve_skill_id, level=0)
    api_module1 = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.overload)
    api_module2 = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.overload)
    # Verification - modules are tried at most online
    api_fit.check_try_fit_items(
        val_options=ValOptions(overload_skill=[api_module1.id]),
        passing=[],
        failing=[eve_module_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(overload_skill=[api_module2.id]),
        passing=[],
        failing=[eve_module_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(overload_skill=[api_module1.id, api_module2.id]),
        passing=[eve_module_id],
        failing=[])
