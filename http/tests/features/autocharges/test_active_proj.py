"""
Unlike regular charges, no known autocharges have active effects which apply modifiers to targets.
But autocharges inherit that ability from charges, so test it here nevertheless.
"""

from tests import approx


def test_proj_unproj(client, consts):
    eve_autocharge_attr = client.mk_eve_attr(id_=consts.EveAttr.fighter_ability_launch_bomb_type)
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_autocharge_effect = client.mk_eve_effect(
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_charge = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_fighter = client.mk_eve_item(attrs={eve_autocharge_attr.id: eve_charge.id}, eff_ids=[eve_autocharge_effect.id])
    eve_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    api_affector_fit = api_sol.create_fit()
    api_affector_fighter = api_affector_fit.add_fighter(type_id=eve_fighter.id, state=consts.ApiState.active)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)
    # Action
    api_affector_fighter.change_fighter(add_projs=[api_affectee_ship.id])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1200)
    # Action
    api_affector_fighter.change_fighter(rm_projs=[api_affectee_ship.id])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)


def test_remove(client, consts):
    eve_autocharge_attr = client.mk_eve_attr(id_=consts.EveAttr.fighter_ability_launch_bomb_type)
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_autocharge_effect = client.mk_eve_effect(
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_charge = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_fighter = client.mk_eve_item(attrs={eve_autocharge_attr.id: eve_charge.id}, eff_ids=[eve_autocharge_effect.id])
    eve_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    api_affector_fit = api_sol.create_fit()
    api_affector_fighter = api_affector_fit.add_fighter(type_id=eve_fighter.id, state=consts.ApiState.active)
    api_affector_fighter.change_fighter(add_projs=[api_affectee_ship.id])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1200)
    # Action
    api_affector_fighter.remove()
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)


def test_states(client, consts):
    eve_autocharge_attr = client.mk_eve_attr(id_=consts.EveAttr.fighter_ability_launch_bomb_type)
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_autocharge_effect = client.mk_eve_effect(
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_charge = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_fighter = client.mk_eve_item(attrs={eve_autocharge_attr.id: eve_charge.id}, eff_ids=[eve_autocharge_effect.id])
    eve_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    api_affector_fit = api_sol.create_fit()
    api_affector_fighter = api_affector_fit.add_fighter(type_id=eve_fighter.id, state=consts.ApiState.online)
    api_affector_fighter.change_fighter(add_projs=[api_affectee_ship.id])
    api_autocharge = api_affector_fighter.autocharges[eve_autocharge_effect.id]
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)
    # Action
    api_affector_fighter.change_fighter(state=consts.ApiState.active)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1200)
    # Action
    api_affector_fighter.change_fighter(state=consts.ApiState.online)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)
    # Action
    api_autocharge.change_autocharge(state=True)
    # Verification - active charge state does not override too low module state
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)
    # Action
    api_affector_fighter.change_fighter(state=consts.ApiState.active)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1200)
    # Action
    api_autocharge.change_autocharge(state=False)
    # Verification - disabled charge state stops effects, even if parent module is in high enough
    # state
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)
    # Action
    api_affector_fighter.change_fighter(state=consts.ApiState.online)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)
    # Action
    api_affector_fighter.change_fighter(state=consts.ApiState.active)
    # Verification - re-enabling module does not enable charge, since it was not enabled after
    # getting disabled
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)


def test_range(client, consts):
    # Check that module range change affects charge as well
    eve_autocharge_attr = client.mk_eve_attr(id_=consts.EveAttr.fighter_ability_launch_bomb_type)
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_optimal_attr = client.mk_eve_attr()
    eve_falloff_attr = client.mk_eve_attr()
    eve_autocharge_effect = client.mk_eve_effect(
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr.id,
        falloff_attr_id=eve_falloff_attr.id,
        mod_info=[eve_mod])
    eve_charge = client.mk_eve_item(
        attrs={eve_affector_attr.id: 20, eve_optimal_attr.id: 10000, eve_falloff_attr.id: 5000},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_fighter = client.mk_eve_item(attrs={eve_autocharge_attr.id: eve_charge.id}, eff_ids=[eve_autocharge_effect.id])
    eve_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    api_affector_fit = api_sol.create_fit()
    api_affector_fighter = api_affector_fit.add_fighter(type_id=eve_fighter.id, state=consts.ApiState.active)
    api_affector_fighter.change_fighter(add_projs=[(api_affectee_ship.id, 10000)])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1200)
    # Action
    api_affector_fighter.change_fighter(change_projs=[(api_affectee_ship.id, 15000)])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1100)
    # Action
    api_affector_fighter.change_fighter(change_projs=[(api_affectee_ship.id, None)])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1200)


def test_src_switch(client, consts):
    used_attrs = set()
    used_effects = set()
    used_items = set()
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    # The same autocharge attr ID
    eve_autocharge_attr_id = eve_d1.mk_attr(id_=consts.EveAttr.fighter_ability_launch_bomb_type).id
    eve_d2.mk_attr(id_=eve_autocharge_attr_id)
    used_attrs.add(eve_autocharge_attr_id)
    # Different affector attrs IDs
    eve_d1_affector_attr = eve_d1.mk_attr(avoid_ids=used_attrs)
    used_attrs.add(eve_d1_affector_attr.id)
    eve_d2_affector_attr = eve_d2.mk_attr(avoid_ids=used_attrs)
    used_attrs.add(eve_d2_affector_attr.id)
    # The same affectee attr ID
    eve_affectee_attr_id = eve_d1.mk_attr(avoid_ids=used_attrs).id
    eve_d2.mk_attr(id_=eve_affectee_attr_id)
    # The same on-fighter autocharge effect ID
    eve_autocharge_effect_id = eve_d1.mk_effect(
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active).id
    eve_d2.mk_effect(id_=eve_autocharge_effect_id, cat_id=consts.EveEffCat.active)
    used_effects.add(eve_autocharge_effect_id)
    # Different on-autocharge effect IDs
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_d1_affector_attr.id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_d1_effect = eve_d1.mk_effect(avoid_ids=used_effects, cat_id=consts.EveEffCat.target, mod_info=[eve_mod1])
    used_effects.add(eve_d1_effect.id)
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_d2_affector_attr.id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_d2_effect = eve_d2.mk_effect(avoid_ids=used_effects, cat_id=consts.EveEffCat.target, mod_info=[eve_mod2])
    # Different autocharge IDs
    eve_d1_autocharge = eve_d1.mk_item(
        attrs={eve_d1_affector_attr.id: 20},
        eff_ids=[eve_d1_effect.id],
        defeff_id=eve_d1_effect.id)
    used_items.add(eve_d1_autocharge.id)
    eve_d2_autocharge = eve_d2.mk_item(
        attrs={eve_d2_affector_attr.id: 1.5},
        eff_ids=[eve_d2_effect.id],
        defeff_id=eve_d2_effect.id)
    used_items.add(eve_d2_autocharge.id)
    # The same fighter ID
    eve_fighter_id = eve_d1.mk_item(
        avoid_ids=used_items,
        attrs={eve_autocharge_attr_id: eve_d1_autocharge.id},
        eff_ids=[eve_autocharge_effect_id]).id
    eve_d2.mk_item(
        id_=eve_fighter_id,
        attrs={eve_autocharge_attr_id: eve_d2_autocharge.id},
        eff_ids=[eve_autocharge_effect_id])
    used_items.add(eve_fighter_id)
    # The same ship ID
    eve_ship_id = eve_d1.mk_ship(avoid_ids=used_items, attrs={eve_affectee_attr_id: 1000}).id
    eve_d2.mk_ship(id_=eve_ship_id, attrs={eve_affectee_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_affector_fit = api_sol.create_fit()
    api_affector_fighter = api_affector_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiState.active)
    api_affector_fighter.change_fighter(add_projs=[api_affectee_ship.id])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1200)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1500)
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1200)
