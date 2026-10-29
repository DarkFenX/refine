from fw.api import ValOptions


def test_known_failures(client, consts):
    eve_tgt_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active)
    eve_src_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.stopper,
        loc=consts.EveModLoc.tgt_stopper,
        effect_id=eve_tgt_effect_id)
    eve_src_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_src_mod])
    eve_src_item_id = client.mk_eve_item(eff_ids=[eve_src_effect_id], defeff_id=eve_src_effect_id)
    eve_fighter_id = client.mk_eve_fighter(eff_ids=[eve_tgt_effect_id], defeff_id=eve_tgt_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_item = api_src_fit.add_module(type_id=eve_src_item_id, state=consts.ApiModuleState.active)
    api_fit = api_sol.create_fit()
    api_fighter1 = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fighter2 = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_src_item.change_module(add_proj_item_ids=[api_fighter1.id, api_fighter2.id])
    # Verification - fighters are tried in bay, and nothing is projected onto them
    api_fit.check_try_fit_items(
        val_options=ValOptions(effect_stopper=[api_fighter1.id]),
        passing=[],
        failing=[eve_fighter_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(effect_stopper=[api_fighter2.id]),
        passing=[],
        failing=[eve_fighter_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(effect_stopper=[api_fighter1.id, api_fighter2.id]),
        passing=[eve_fighter_id],
        failing=[])
