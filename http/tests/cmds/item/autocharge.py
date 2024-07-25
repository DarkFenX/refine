
def test_remove(client, consts):
    eve_attr = client.mk_eve_attr(id_=consts.EveAttr.fighter_ability_launch_bomb_type)
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.fighter_ability_launch_bomb, cat_id=consts.EveEffCat.active)
    eve_charge = client.mk_eve_item()
    eve_fighter = client.mk_eve_item(attrs={eve_attr.id: eve_charge.id}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter.id)
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[eve_effect.id]
    api_autocharge_id = api_autocharge.id
    # Cannot remove autocharges, they are handled automatically
    api_autocharge.remove(
        status_code=403,
        json_predicate={'code': 'COR-019', 'message': 'core library error: SolAutoCharge cannot be manually removed'})
    # And after attempt of removal, all the info is still there
    api_fighter.update()
    assert api_fighter.autocharges[eve_effect.id].id == api_autocharge_id
    # Remove it with fighter for the sake of consistency check
    api_fighter.remove()
    # Try removing autocharge again
    api_autocharge.remove(status_code=404)
