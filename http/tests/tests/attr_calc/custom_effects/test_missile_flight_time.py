from tests import approx


def test_missile_kinds(client, consts):
    eve_flight_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.explosion_delay)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_regular_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.missile_launching,
        cat_id=consts.EveEffCat.target)
    eve_defender_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.defender_missile_launching,
        cat_id=consts.EveEffCat.active)
    eve_fof_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fof_missile_launching,
        cat_id=consts.EveEffCat.active)
    eve_dot_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.dot_missile_launching,
        cat_id=consts.EveEffCat.target)
    eve_regular_missile_id = client.mk_eve_item(
        attrs={eve_speed_attr_id: 500, eve_flight_time_attr_id: 10000},
        eff_ids=[eve_regular_effect_id],
        defeff_id=eve_regular_effect_id)
    eve_defender_missile_id = client.mk_eve_item(
        attrs={eve_speed_attr_id: 500, eve_flight_time_attr_id: 10000},
        eff_ids=[eve_defender_effect_id],
        defeff_id=eve_defender_effect_id)
    eve_fof_missile_id = client.mk_eve_item(
        attrs={eve_speed_attr_id: 500, eve_flight_time_attr_id: 10000},
        eff_ids=[eve_fof_effect_id],
        defeff_id=eve_fof_effect_id)
    eve_dot_missile_id = client.mk_eve_item(
        attrs={eve_speed_attr_id: 500, eve_flight_time_attr_id: 10000},
        eff_ids=[eve_dot_effect_id],
        defeff_id=eve_dot_effect_id)
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 2000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_regular_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_regular_missile_id)
    api_defender_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_defender_missile_id)
    api_fof_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_fof_missile_id)
    api_dot_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_dot_missile_id)
    # Verification
    api_regular_module.update()
    assert api_regular_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_regular_module.charge.attrs[eve_flight_time_attr_id].extra == approx(14000)
    api_defender_module.update()
    assert api_defender_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_defender_module.charge.attrs[eve_flight_time_attr_id].extra == approx(14000)
    api_fof_module.update()
    assert api_fof_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_fof_module.charge.attrs[eve_flight_time_attr_id].extra == approx(14000)
    api_dot_module.update()
    assert api_dot_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_dot_module.charge.attrs[eve_flight_time_attr_id].extra == approx(14000)


def test_mod_info(client, consts):
    eve_flight_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.explosion_delay)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.missile_launching,
        cat_id=consts.EveEffCat.target)
    eve_missile_id = client.mk_eve_item(
        attrs={eve_speed_attr_id: 500, eve_flight_time_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 2000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_missile_id)
    # Verification
    api_module.update()
    assert api_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_module.charge.attrs[eve_flight_time_attr_id].extra == approx(14000)
    api_mod = api_module.charge.mods[eve_flight_time_attr_id].one()
    assert api_mod.op == consts.ApiModOp.extra_add
    assert api_mod.initial_val == approx(4000)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(4000)
    assert len(api_mod.affectors) == 2
    assert api_mod.affectors.find_by_item(item_id=api_module.charge.id).one().attr_id == eve_speed_attr_id
    assert api_mod.affectors.find_by_item(item_id=api_ship.id).one().attr_id == eve_radius_attr_id


def test_isolation(client, consts):
    # Each change is isolated to a charge itself
    eve_flight_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.explosion_delay)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.missile_launching,
        cat_id=consts.EveEffCat.target)
    eve_missile_id = client.mk_eve_item(
        attrs={eve_speed_attr_id: 500, eve_flight_time_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 2000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_missile_id)
    api_module2 = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_missile_id)
    # Verification
    api_module1.update()
    assert api_module1.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_module1.charge.attrs[eve_flight_time_attr_id].extra == approx(14000)
    api_module2.update()
    assert api_module2.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_module2.charge.attrs[eve_flight_time_attr_id].extra == approx(14000)


def test_state(client, consts):
    # Since hidden bonus is implemented as effect, it stops working when module has Ghost state
    eve_flight_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.explosion_delay)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.missile_launching,
        cat_id=consts.EveEffCat.target)
    eve_missile_id = client.mk_eve_item(
        attrs={eve_speed_attr_id: 500, eve_flight_time_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 2000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.ghost,
        charge_type_id=eve_missile_id)
    # Verification
    api_module.update()
    assert api_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_module.charge.attrs[eve_flight_time_attr_id].extra == approx(10000)
    # Action
    api_module.change_module(state=consts.ApiModuleState.offline)
    # Verification
    api_module.update()
    assert api_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_module.charge.attrs[eve_flight_time_attr_id].extra == approx(14000)
    # Action
    api_module.change_module(state=consts.ApiModuleState.ghost)
    # Verification
    api_module.update()
    assert api_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_module.charge.attrs[eve_flight_time_attr_id].extra == approx(10000)
    # Action - just check that solar system is in consistent state when charge is removed with the
    # effect disabled
    api_module.change_module(charge_type_id=None)


def test_dogma_interaction(client, consts):
    # Modification is applied after all normal dogma operations, so should work even with
    # post-assignment
    eve_skill_id = client.mk_eve_item()
    eve_flight_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.explosion_delay)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_missile_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.missile_launching,
        cat_id=consts.EveEffCat.target)
    eve_missile_id = client.mk_eve_item(
        attrs={eve_speed_attr_id: 500, eve_flight_time_attr_id: 10000},
        eff_ids=[eve_missile_effect_id],
        defeff_id=eve_missile_effect_id,
        srqs={eve_skill_id: 2})
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 2000})
    eve_affector_attr_id = client.mk_eve_attr()
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_flight_time_attr_id)
    eve_rig_effect_id = client.mk_eve_effect(mod_info=[eve_rig_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20000}, eff_ids=[eve_rig_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_missile_id)
    # Verification
    api_module.update()
    assert api_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_module.charge.attrs[eve_flight_time_attr_id].extra == approx(14000)
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_module.update()
    assert api_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(20000)
    assert api_module.charge.attrs[eve_flight_time_attr_id].extra == approx(24000)
    # Action
    api_rig.remove()
    # Verification
    api_module.update()
    assert api_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_module.charge.attrs[eve_flight_time_attr_id].extra == approx(14000)


def test_speed_zero(client, consts):
    eve_flight_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.explosion_delay)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.missile_launching,
        cat_id=consts.EveEffCat.target)
    eve_missile_id = client.mk_eve_item(
        attrs={eve_speed_attr_id: 0, eve_flight_time_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 2000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_missile_id)
    # Verification - bonus is not applied when missile velocity is 0
    api_module.update()
    assert api_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_module.charge.attrs[eve_flight_time_attr_id].extra == approx(10000)


def test_modifier_ship_change(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_flight_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.explosion_delay)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_missile_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.missile_launching,
        cat_id=consts.EveEffCat.target)
    eve_missile_id = client.mk_eve_item(
        attrs={eve_speed_attr_id: 500, eve_flight_time_attr_id: 10000},
        eff_ids=[eve_missile_effect_id],
        defeff_id=eve_missile_effect_id,
        srqs={eve_skill_id: 2})
    eve_module_id = client.mk_eve_item()
    eve_ship1_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 2000})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 1000})
    eve_unloaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship1_id)
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_missile_id)
    # Verification
    api_module.update()
    assert api_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_module.charge.attrs[eve_flight_time_attr_id].extra == approx(14000)
    # Action
    api_fit.set_ship(type_id=eve_ship2_id)
    # Verification
    api_module.update()
    assert api_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_module.charge.attrs[eve_flight_time_attr_id].extra == approx(12000)
    # Action
    api_fit.set_ship(type_id=eve_unloaded_id)
    # Verification
    api_module.update()
    assert api_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_module.charge.attrs[eve_flight_time_attr_id].extra == approx(10000)
    # Action
    api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_module.update()
    assert api_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_module.charge.attrs[eve_flight_time_attr_id].extra == approx(14000)
    # Action
    api_fit.remove_ship()
    # Verification
    api_module.update()
    assert api_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_module.charge.attrs[eve_flight_time_attr_id].extra == approx(10000)
    # Action
    api_fit.set_ship(type_id=eve_ship2_id)
    # Verification
    api_module.update()
    assert api_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_module.charge.attrs[eve_flight_time_attr_id].extra == approx(12000)


def test_modifier_switch_type_id_ship(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_flight_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.explosion_delay)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_missile_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.missile_launching,
        cat_id=consts.EveEffCat.target)
    eve_missile_id = client.mk_eve_item(
        attrs={eve_speed_attr_id: 500, eve_flight_time_attr_id: 10000},
        eff_ids=[eve_missile_effect_id],
        defeff_id=eve_missile_effect_id,
        srqs={eve_skill_id: 2})
    eve_module_id = client.mk_eve_item()
    eve_ship1_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 2000})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 1000})
    eve_unloaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship1_id)
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_missile_id)
    # Verification
    api_module.update()
    assert api_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_module.charge.attrs[eve_flight_time_attr_id].extra == approx(14000)
    # Action
    api_ship.change_ship(type_id=eve_ship2_id)
    # Verification
    api_module.update()
    assert api_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_module.charge.attrs[eve_flight_time_attr_id].extra == approx(12000)
    # Action
    api_ship.change_ship(type_id=eve_unloaded_id)
    # Verification
    api_module.update()
    assert api_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_module.charge.attrs[eve_flight_time_attr_id].extra == approx(10000)
    # Action
    api_ship.change_ship(type_id=eve_ship1_id)
    # Verification
    api_module.update()
    assert api_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_module.charge.attrs[eve_flight_time_attr_id].extra == approx(14000)


def test_modifier_missile_velocity_changed(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_flight_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.explosion_delay)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_missile_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.missile_launching,
        cat_id=consts.EveEffCat.target)
    eve_missile_id = client.mk_eve_item(
        attrs={eve_speed_attr_id: 500, eve_flight_time_attr_id: 10000},
        eff_ids=[eve_missile_effect_id],
        defeff_id=eve_missile_effect_id,
        srqs={eve_skill_id: 2})
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 2000})
    eve_affector_attr_id = client.mk_eve_attr()
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_speed_attr_id)
    eve_rig_effect_id = client.mk_eve_effect(mod_info=[eve_rig_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_affector_attr_id: 60}, eff_ids=[eve_rig_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_missile_id)
    # Verification
    api_module.update()
    assert api_module.charge.attrs[eve_speed_attr_id].dogma == approx(500)
    assert api_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_module.charge.attrs[eve_flight_time_attr_id].extra == approx(14000)
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_module.update()
    assert api_module.charge.attrs[eve_speed_attr_id].dogma == approx(800)
    assert api_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_module.charge.attrs[eve_flight_time_attr_id].extra == approx(12500)
    # Action
    api_rig.remove()
    # Verification
    api_module.update()
    assert api_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_module.charge.attrs[eve_flight_time_attr_id].extra == approx(14000)


def test_modifier_ship_radius_changed(client, consts):
    eve_flight_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.explosion_delay)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_missile_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.missile_launching,
        cat_id=consts.EveEffCat.target)
    eve_missile_id = client.mk_eve_item(
        attrs={eve_speed_attr_id: 500, eve_flight_time_attr_id: 10000},
        eff_ids=[eve_missile_effect_id],
        defeff_id=eve_missile_effect_id)
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 2000})
    eve_affector_attr_id = client.mk_eve_attr()
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_radius_attr_id)
    eve_rig_effect_id = client.mk_eve_effect(mod_info=[eve_rig_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_affector_attr_id: -20}, eff_ids=[eve_rig_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_missile_id)
    # Verification
    assert api_ship.update().attrs[eve_radius_attr_id].dogma == approx(2000)
    api_module.update()
    assert api_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_module.charge.attrs[eve_flight_time_attr_id].extra == approx(14000)
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification - lib uses unmodified ship radius for calculations, so result does not change
    assert api_ship.update().attrs[eve_radius_attr_id].dogma == approx(1600)
    api_module.update()
    assert api_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_module.charge.attrs[eve_flight_time_attr_id].extra == approx(14000)
    # Action
    api_rig.remove()
    # Verification
    assert api_ship.update().attrs[eve_radius_attr_id].dogma == approx(2000)
    api_module.update()
    assert api_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_module.charge.attrs[eve_flight_time_attr_id].extra == approx(14000)


def test_modifier_ship_not_loaded(client, consts):
    eve_flight_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.explosion_delay)
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.missile_launching,
        cat_id=consts.EveEffCat.target)
    eve_missile_id = client.mk_eve_item(
        attrs={eve_speed_attr_id: 500, eve_flight_time_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_missile_id)
    # Verification - failure to calculate dependencies means bonus is not applied
    api_module.update()
    assert api_module.charge.attrs[eve_flight_time_attr_id].dogma == approx(10000)
    assert api_module.charge.attrs[eve_flight_time_attr_id].extra == approx(10000)
