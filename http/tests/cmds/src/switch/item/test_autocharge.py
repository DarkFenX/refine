from tests import approx


def test_specified_different(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_d1.mk_attr(id_=consts.EveAttr.fighter_ability_launch_bomb_type)
    eve_d2.mk_attr(id_=consts.EveAttr.fighter_ability_launch_bomb_type)
    eve_d1_attr = eve_d1.mk_attr()
    eve_d2_attr = eve_d2.mk_attr()
    eve_d1.mk_effect(id_=consts.EveEffect.fighter_ability_launch_bomb, cat_id=consts.EveEffCat.active)
    eve_d2.mk_effect(id_=consts.EveEffect.fighter_ability_launch_bomb, cat_id=consts.EveEffCat.active)
    eve_d1_charge = eve_d1.mk_item(attrs={eve_d1_attr.id: 50})
    eve_d2_charge_id = eve_d2.alloc_item_id(avoid_ids=[eve_d1_charge.id])
    eve_d2.mk_item(id_=eve_d2_charge_id, attrs={eve_d2_attr.id: 70})
    eve_fighter_id = eve_d1.alloc_attr_id(avoid_ids=[eve_d1_charge.id, eve_d2_charge_id])
    eve_d1.mk_item(
        id_=eve_fighter_id,
        attrs={consts.EveAttr.fighter_ability_launch_bomb_type: eve_d1_charge.id},
        eff_ids=[consts.EveEffect.fighter_ability_launch_bomb])
    eve_d2.mk_item(
        id_=eve_fighter_id,
        attrs={consts.EveAttr.fighter_ability_launch_bomb_type: eve_d2_charge_id},
        eff_ids=[consts.EveEffect.fighter_ability_launch_bomb])
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_fighter.update()
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[consts.EveEffect.fighter_ability_launch_bomb]
    assert api_autocharge.type_id == eve_d1_charge.id
    assert api_autocharge.attrs[eve_d1_attr.id].dogma == approx(50)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    api_fighter.update()
    assert len(api_fighter.autocharges) == 1
    api_autocharge = api_fighter.autocharges[consts.EveEffect.fighter_ability_launch_bomb]
    assert api_autocharge.type_id == eve_d2_charge_id
    assert api_autocharge.attrs[eve_d1_attr.id].dogma == approx(70)
