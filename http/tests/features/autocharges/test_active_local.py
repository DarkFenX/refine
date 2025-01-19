"""
There are no autocharges with active effects which affect fit-local items, but it is a possibility
with the lib, so we check it nevertheless (just hope it doesn't crash or doesn't make solar system
inconsistent).
"""

from tests import approx


def test_remove(client, consts):
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=[eve_mod])
    eve_charge_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_fighter_id = client.mk_eve_item(
        attrs={eve_autocharge_attr_id: eve_charge_id},
        eff_ids=[eve_autocharge_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiState.active)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1200)
    # Action
    api_fighter.remove()
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1000)


def test_states(client, consts):
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=[eve_mod])
    eve_charge_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_fighter_id = client.mk_eve_item(
        attrs={eve_autocharge_attr_id: eve_charge_id},
        eff_ids=[eve_autocharge_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiState.online)
    api_autocharge = api_fighter.autocharges[eve_autocharge_effect_id]
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1000)
    # Action
    api_fighter.change_fighter(state=consts.ApiState.active)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1200)
    # Action
    api_fighter.change_fighter(state=consts.ApiState.online)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1000)
    # Action
    api_autocharge.change_autocharge(state=True)
    # Verification - active charge state does not override too low module state
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1000)
    # Action
    api_fighter.change_fighter(state=consts.ApiState.active)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1200)
    # Action
    api_autocharge.change_autocharge(state=False)
    # Verification - disabled charge state stops effects, even if parent module is in high enough
    # state
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1000)
    # Action
    api_fighter.change_fighter(state=consts.ApiState.online)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1000)
    # Action
    api_fighter.change_fighter(state=consts.ApiState.active)
    # Verification - re-enabling module does not enable charge, since it was not enabled after
    # getting disabled
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1000)


def test_src_switch(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    # The same autocharge attr ID
    eve_autocharge_attr_id = client.mk_eve_attr(
        datas=[eve_d1, eve_d2],
        id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    # Different affector attrs IDs
    eve_d1_affector_attr_id = client.alloc_attr_id(datas=[eve_d1, eve_d2])
    client.mk_eve_attr(datas=[eve_d1], id_=eve_d1_affector_attr_id)
    eve_d2_affector_attr_id = client.alloc_attr_id(datas=[eve_d1, eve_d2])
    client.mk_eve_attr(datas=[eve_d2], id_=eve_d2_affector_attr_id)
    # The same affectee attr ID
    eve_affectee_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    # The same on-fighter autocharge effect ID
    eve_autocharge_effect_id = client.mk_eve_effect(
        datas=[eve_d1, eve_d2],
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    # Different on-autocharge effect IDs
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_d1_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_d1_effect_id = client.alloc_effect_id(datas=[eve_d1, eve_d2])
    client.mk_eve_effect(datas=[eve_d1], id_=eve_d1_effect_id, cat_id=consts.EveEffCat.active, mod_info=[eve_mod1])
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_d2_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_d2_effect_id = client.alloc_effect_id(datas=[eve_d1, eve_d2])
    client.mk_eve_effect(datas=[eve_d2], id_=eve_d2_effect_id, cat_id=consts.EveEffCat.active, mod_info=[eve_mod2])
    # Different autocharge IDs
    eve_d1_autocharge_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(
        datas=[eve_d1],
        id_=eve_d1_autocharge_id,
        attrs={eve_d1_affector_attr_id: 20},
        eff_ids=[eve_d1_effect_id],
        defeff_id=eve_d1_effect_id)
    eve_d2_autocharge_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(
        datas=[eve_d2],
        id_=eve_d2_autocharge_id,
        attrs={eve_d2_affector_attr_id: 1.5},
        eff_ids=[eve_d2_effect_id],
        defeff_id=eve_d2_effect_id)
    # The same fighter ID
    eve_fighter_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(
        datas=[eve_d1],
        id_=eve_fighter_id,
        attrs={eve_autocharge_attr_id: eve_d1_autocharge_id},
        eff_ids=[eve_autocharge_effect_id])
    client.mk_eve_item(
        datas=[eve_d2],
        id_=eve_fighter_id,
        attrs={eve_autocharge_attr_id: eve_d2_autocharge_id},
        eff_ids=[eve_autocharge_effect_id])
    # The same ship ID
    eve_ship_id = client.mk_eve_ship(datas=[eve_d1, eve_d2], attrs={eve_affectee_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiState.active)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1200)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1500)
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1200)
