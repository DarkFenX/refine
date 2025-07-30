from tests import check_no_field


def test_no_effect(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_effect_id = client.mk_eve_effect(
        datas=[eve_d2],
        id_=consts.EveEffect.ftr_abil_attack_m,
        cat_id=consts.EveEffCat.target)
    eve_abil_id = client.mk_eve_abil(datas=[eve_d1, eve_d2], id_=consts.EveAbil.pulse_cannon)
    eve_fighter_id = client.mk_eve_item(
        datas=[eve_d1, eve_d2],
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id,
        abils=[client.mk_eve_item_abil(id_=eve_abil_id)])
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_fighter.update()
    with check_no_field():
        api_fighter.abilities  # noqa: B018
    api_fighter.change_fighter(abilities={eve_abil_id: False})
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification - status hasn't been changed, since ability wasn't valid on 1st source
    assert api_fighter.update().abilities[eve_abil_id].state is True


def test_no_ability(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_effect_id = client.mk_eve_effect(
        datas=[eve_d1, eve_d2],
        id_=consts.EveEffect.ftr_abil_attack_m,
        cat_id=consts.EveEffCat.target)
    eve_abil_id = client.mk_eve_abil(datas=[eve_d2], id_=consts.EveAbil.pulse_cannon)
    eve_fighter_id = client.mk_eve_item(
        datas=[eve_d1, eve_d2],
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id,
        abils=[client.mk_eve_item_abil(id_=eve_abil_id)])
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_fighter.update()
    with check_no_field():
        api_fighter.abilities  # noqa: B018
    api_fighter.change_fighter(abilities={eve_abil_id: False})
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification - status hasn't been changed, since ability wasn't valid on 1st source
    assert api_fighter.update().abilities[eve_abil_id].state is True


def test_no_item_ability_data(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_effect_id = client.mk_eve_effect(
        datas=[eve_d1, eve_d2],
        id_=consts.EveEffect.ftr_abil_attack_m,
        cat_id=consts.EveEffCat.target)
    eve_abil_id = client.mk_eve_abil(datas=[eve_d1, eve_d2], id_=consts.EveAbil.pulse_cannon)
    eve_fighter_id = client.alloc_item_id()
    client.mk_eve_item(
        datas=[eve_d1],
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.mk_eve_item(
        datas=[eve_d2],
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id,
        abils=[client.mk_eve_item_abil(id_=eve_abil_id)])
    # An item just to keep ability data alive during cleanup on 1st source
    client.mk_eve_item(
        datas=[eve_d1],
        eff_ids=[eve_effect_id],
        abils=[client.mk_eve_item_abil(id_=eve_abil_id)])
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_fighter.update()
    with check_no_field():
        api_fighter.abilities  # noqa: B018
    api_fighter.change_fighter(abilities={eve_abil_id: False})
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification - status hasn't been changed, since ability wasn't valid on 1st source
    assert api_fighter.update().abilities[eve_abil_id].state is True
