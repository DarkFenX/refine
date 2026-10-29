from fw.api import ValOptions


def test_known_failures(client, consts):
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.ftr_abil_launch_bomb, cat_id=consts.EveEffCat.active)
    eve_abil_id = client.mk_eve_abil(id_=consts.EveAbil.launch_bomb, banned_hisec=True, banned_lowsec=True)
    eve_fighter_id = client.mk_eve_fighter(eff_ids=[eve_effect_id], abils=[client.mk_eve_item_abil(id_=eve_abil_id)])
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_fighter1 = api_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_abil_id: True})
    api_fighter2 = api_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_abil_id: True})
    # Verification - fighters are tried in bay
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_effect=[api_fighter1.id]),
        passing=[],
        failing=[eve_fighter_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_effect=[api_fighter2.id]),
        passing=[],
        failing=[eve_fighter_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_effect=[api_fighter1.id, api_fighter2.id]),
        passing=[eve_fighter_id],
        failing=[])
