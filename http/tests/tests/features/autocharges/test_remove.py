from tests import Effect


def test_remove_item(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_charge_id = client.mk_eve_item()
    eve_fighter_id = client.mk_eve_item(attrs={eve_attr_id: eve_charge_id}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_effect_id = Effect.dogma_to_api(dogma_effect_id=eve_effect_id)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[api_effect_id]
    api_autocharge_id = api_autocharge.id
    # Cannot remove autocharges, they are handled automatically
    api_autocharge.remove(
        status_code=403,
        json_predicate={'code': 'ACH-001', 'message': 'Autocharge cannot be manually removed'})
    # And after attempt of removal, all the info is still there
    api_fighter.update()
    assert api_fighter.autocharges[api_effect_id].id == api_autocharge_id
    # Remove it with fighter for the sake of consistency check
    api_fighter.remove()
    # Try removing autocharge again
    api_autocharge.remove(status_code=404)


def test_remove_fit(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_charge_id = client.mk_eve_item()
    eve_fighter_id = client.mk_eve_item(attrs={eve_attr_id: eve_charge_id}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_fighter(type_id=eve_fighter_id)
    # Check via consistency check if item with autocharge is properly removed when fit is removed
    api_fit.remove()
