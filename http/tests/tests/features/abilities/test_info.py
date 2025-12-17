from fw import check_no_field


def test_charge_count(client, consts):
    eve_primary_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_attack_m,
        cat_id=consts.EveEffCat.target)
    eve_secondary_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_missiles,
        cat_id=consts.EveEffCat.target)
    eve_primary_abil_id = client.mk_eve_abil(id_=consts.EveAbil.pulse_cannon)
    eve_secondary_abil_id = client.mk_eve_abil(id_=consts.EveAbil.heavy_rocket_salvo)
    eve_fighter1_id = client.mk_eve_fighter(
        eff_ids=[eve_primary_effect_id, eve_secondary_effect_id],
        defeff_id=eve_primary_effect_id,
        abils=[
            client.mk_eve_item_abil(id_=eve_primary_abil_id),
            client.mk_eve_item_abil(id_=eve_secondary_abil_id, charge_count=12, charge_rearm_time=5)])
    eve_fighter2_id = client.mk_eve_fighter(
        eff_ids=[eve_primary_effect_id, eve_secondary_effect_id],
        defeff_id=eve_primary_effect_id,
        abils=[
            client.mk_eve_item_abil(id_=eve_primary_abil_id),
            client.mk_eve_item_abil(id_=eve_secondary_abil_id, charge_count=15, charge_rearm_time=5)])
    eve_fighter3_id = client.mk_eve_fighter(
        eff_ids=[eve_primary_effect_id, eve_secondary_effect_id],
        defeff_id=eve_primary_effect_id,
        abils=[
            client.mk_eve_item_abil(id_=eve_primary_abil_id),
            client.mk_eve_item_abil(id_=eve_secondary_abil_id, charge_count=0, charge_rearm_time=5)])
    eve_fighter4_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter1_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_fighter.update()
    assert api_fighter.abilities[eve_primary_abil_id].charge_count is None
    assert api_fighter.abilities[eve_secondary_abil_id].charge_count == 12
    # Action
    api_fighter.change_fighter(type_id=eve_fighter2_id)
    # Verification
    api_fighter.update()
    assert api_fighter.abilities[eve_primary_abil_id].charge_count is None
    assert api_fighter.abilities[eve_secondary_abil_id].charge_count == 15
    # Action
    api_fighter.change_fighter(type_id=eve_fighter3_id)
    # Verification
    api_fighter.update()
    assert api_fighter.abilities[eve_primary_abil_id].charge_count is None
    assert api_fighter.abilities[eve_secondary_abil_id].charge_count == 0
    # Action
    api_fighter.change_fighter(type_id=eve_fighter4_id)
    # Verification
    api_fighter.update()
    with check_no_field():
        api_fighter.abilities  # noqa: B018
