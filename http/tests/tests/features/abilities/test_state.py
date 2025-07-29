

def test_switch_state(client, consts):
    eve_main_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_attack_m,
        cat_id=consts.EveEffCat.target)
    eve_secondary_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_missiles,
        cat_id=consts.EveEffCat.target)
    eve_main_abil_id = client.mk_eve_abil(id_=consts.EveAbil.pulse_cannon)
    eve_secondary_abil_id = client.mk_eve_abil(id_=consts.EveAbil.heavy_rocket_salvo)
    eve_fighter_id = client.mk_eve_item(
        attrs={},
        eff_ids=[eve_main_effect_id, eve_secondary_effect_id],
        defeff_id=eve_main_effect_id,
        abils=[client.mk_eve_item_abil(id_=eve_main_abil_id), client.mk_eve_item_abil(id_=eve_secondary_abil_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_fighter.update()
    assert len(api_fighter.abilities) == 2
    assert api_fighter.abilities[eve_main_abil_id].status is True
    assert api_fighter.abilities[eve_secondary_abil_id].status is False
