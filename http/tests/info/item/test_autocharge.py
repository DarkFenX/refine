
def test_autocharge(client, consts):
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.fighter_ability_launch_bomb, cat_id=consts.EveEffCat.active)
    eve_charge = client.mk_eve_item()
    eve_fighter = client.mk_eve_item(
        attrs={consts.EveAttr.fighter_ability_launch_bomb_type: eve_charge.id},
        eff_ids=[eve_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Check default upon addition
    api_fighter = api_fit.add_fighter(type_id=eve_fighter.id)
    assert len(api_fighter.autocharges) == 1
    assert isinstance(api_fighter.autocharges[eve_effect.id].id, str)
    autocharge_id = api_fighter.autocharges[eve_effect.id].id
    # ID only
    api_fighter.update(item_info_mode=consts.ApiItemInfoMode.id)
    assert len(api_fighter.autocharges) == 1
    assert api_fighter.autocharges[eve_effect.id].id == autocharge_id
    # Partial
    api_fighter.update(item_info_mode=consts.ApiItemInfoMode.partial)
    assert len(api_fighter.autocharges) == 1
    assert api_fighter.autocharges[eve_effect.id].id == autocharge_id
    # Full
    api_fighter.update(item_info_mode=consts.ApiItemInfoMode.full)
    assert len(api_fighter.autocharges) == 1
    assert api_fighter.autocharges[eve_effect.id].id == autocharge_id
