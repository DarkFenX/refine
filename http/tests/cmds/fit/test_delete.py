

def test_charge(client, consts):
    eve_module = client.mk_eve_item()
    eve_charge = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_mod(type_id=eve_module.id, charge_type_id=eve_charge.id)
    # Check via consistency check if item with charge is properly removed when fit is removed
    api_fit.remove()


def test_autocharge(client, consts):
    eve_attr = client.mk_eve_attr(id_=consts.EveAttr.fighter_ability_launch_bomb_type)
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.fighter_ability_launch_bomb, cat_id=consts.EveEffCat.active)
    eve_charge = client.mk_eve_item()
    eve_fighter = client.mk_eve_item(attrs={eve_attr.id: eve_charge.id}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_fighter(type_id=eve_fighter.id)
    # Check via consistency check if item with autocharge is properly removed when fit is removed
    api_fit.remove()
