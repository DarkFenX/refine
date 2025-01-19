from tests import approx, check_no_field


def test_ab(client, consts):
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_thrust_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_boost_factor)
    eve_speed_boost_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_mass_add_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass_addition)
    eve_sig_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius_bonus)
    eve_sig_affectee_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_prop_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_afterburner,
        cat_id=consts.EveEffCat.active)
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_speed_attr_id: 455, eve_mass_attr_id: 1050000, eve_sig_affectee_attr_id: 32})
    eve_prop_item_id = client.mk_eve_item(
        attrs={
            eve_speed_boost_attr_id: 135, eve_thrust_attr_id: 1500000,
            eve_mass_add_attr_id: 500000, eve_sig_affector_attr_id: 200},
        eff_ids=[eve_prop_effect_id],
        defeff_id=eve_prop_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_prop_item = api_fit.add_mod(type_id=eve_prop_item_id, rack=consts.ApiRack.mid, state=consts.ApiState.active)
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_mass_attr_id].dogma == approx(1550000)
    assert api_ship.attrs[eve_sig_affectee_attr_id].dogma == approx(32)  # Not affected by sig blow
    assert api_ship.attrs[eve_speed_attr_id].dogma == approx(1049.435484)
    assert len(api_ship.mods) == 2
    # Mass modification
    api_mod_mass = api_ship.mods.find_by_affector_attr(
        affectee_attr_id=eve_mass_attr_id, affector_attr_id=eve_mass_add_attr_id).one()
    assert api_mod_mass.op == consts.ApiModOp.mod_add
    assert api_mod_mass.initial_val == approx(500000)
    assert api_mod_mass.stacking_mult is None
    assert api_mod_mass.applied_val == approx(500000)
    assert api_mod_mass.affectors.one().item_id == api_prop_item.id
    assert api_mod_mass.affectors.one().attr_id == eve_mass_add_attr_id
    # Speed modification
    api_mod_prop = api_ship.mods.find_by_affector_attr(
        affectee_attr_id=eve_speed_attr_id, affector_attr_id=eve_speed_boost_attr_id).one()
    assert api_mod_prop.op == consts.ApiModOp.post_mul
    assert api_mod_prop.initial_val == approx(2.306452)
    assert api_mod_prop.stacking_mult is None
    assert api_mod_prop.applied_val == approx(2.306452)
    assert len(api_mod_prop.affectors) == 3
    assert api_mod_prop.affectors.find_by_attr(attr_id=eve_speed_boost_attr_id).one().item_id == api_prop_item.id
    assert api_mod_prop.affectors.find_by_attr(attr_id=eve_thrust_attr_id).one().item_id == api_prop_item.id
    assert api_mod_prop.affectors.find_by_attr(attr_id=eve_mass_attr_id).one().item_id == api_ship.id


def test_mwd(client, consts):
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_thrust_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_boost_factor)
    eve_speed_boost_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_mass_add_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass_addition)
    eve_sig_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius_bonus)
    eve_sig_affectee_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_prop_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_microwarpdrive,
        cat_id=consts.EveEffCat.active)
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_speed_attr_id: 455, eve_mass_attr_id: 1050000, eve_sig_affectee_attr_id: 32})
    eve_prop_item_id = client.mk_eve_item(
        attrs={
            eve_speed_boost_attr_id: 505, eve_thrust_attr_id: 1500000,
            eve_mass_add_attr_id: 500000, eve_sig_affector_attr_id: 450},
        eff_ids=[eve_prop_effect_id],
        defeff_id=eve_prop_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_prop_item = api_fit.add_mod(type_id=eve_prop_item_id, rack=consts.ApiRack.mid, state=consts.ApiState.active)
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_mass_attr_id].dogma == approx(1550000)
    assert api_ship.attrs[eve_sig_affectee_attr_id].dogma == approx(176)
    assert api_ship.attrs[eve_speed_attr_id].dogma == approx(2678.629032)
    assert len(api_ship.mods) == 3
    # Mass modification
    api_mod_mass = api_ship.mods.find_by_affector_attr(
        affectee_attr_id=eve_mass_attr_id, affector_attr_id=eve_mass_add_attr_id).one()
    assert api_mod_mass.op == consts.ApiModOp.mod_add
    assert api_mod_mass.initial_val == approx(500000)
    assert api_mod_mass.stacking_mult is None
    assert api_mod_mass.applied_val == approx(500000)
    assert api_mod_mass.affectors.one().item_id == api_prop_item.id
    assert api_mod_mass.affectors.one().attr_id == eve_mass_add_attr_id
    # Sig modification
    api_mod_sig = api_ship.mods.find_by_affector_attr(
        affectee_attr_id=eve_sig_affectee_attr_id, affector_attr_id=eve_sig_affector_attr_id).one()
    assert api_mod_sig.op == consts.ApiModOp.post_percent
    assert api_mod_sig.initial_val == approx(450)
    assert api_mod_sig.stacking_mult is None
    assert api_mod_sig.applied_val == approx(450)
    assert api_mod_sig.affectors.one().item_id == api_prop_item.id
    assert api_mod_sig.affectors.one().attr_id == eve_sig_affector_attr_id
    # Speed modification
    api_mod_prop = api_ship.mods.find_by_affector_attr(
        affectee_attr_id=eve_speed_attr_id, affector_attr_id=eve_speed_boost_attr_id).one()
    assert api_mod_prop.op == consts.ApiModOp.post_mul
    assert api_mod_prop.initial_val == approx(5.887097)
    assert api_mod_prop.stacking_mult is None
    assert api_mod_prop.applied_val == approx(5.887097)
    assert len(api_mod_prop.affectors) == 3
    assert api_mod_prop.affectors.find_by_attr(attr_id=eve_speed_boost_attr_id).one().item_id == api_prop_item.id
    assert api_mod_prop.affectors.find_by_attr(attr_id=eve_thrust_attr_id).one().item_id == api_prop_item.id
    assert api_mod_prop.affectors.find_by_attr(attr_id=eve_mass_attr_id).one().item_id == api_ship.id


def test_state(client, consts):
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_thrust_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_boost_factor)
    eve_speed_boost_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_mass_add_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass_addition)
    eve_sig_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius_bonus)
    eve_sig_affectee_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_prop_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_microwarpdrive,
        cat_id=consts.EveEffCat.active)
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_speed_attr_id: 455, eve_mass_attr_id: 1050000, eve_sig_affectee_attr_id: 32})
    eve_prop_item_id = client.mk_eve_item(
        attrs={
            eve_speed_boost_attr_id: 505, eve_thrust_attr_id: 1500000,
            eve_mass_add_attr_id: 500000, eve_sig_affector_attr_id: 450},
        eff_ids=[eve_prop_effect_id],
        defeff_id=eve_prop_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_prop = api_fit.add_mod(type_id=eve_prop_item_id, rack=consts.ApiRack.mid, state=consts.ApiState.active)
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_mass_attr_id].dogma == approx(1550000)
    assert api_ship.attrs[eve_sig_affectee_attr_id].dogma == approx(176)
    assert api_ship.attrs[eve_speed_attr_id].dogma == approx(2678.629032)
    assert len(api_ship.mods) == 3
    # Action
    api_prop.change_mod(state=consts.ApiState.online)
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_mass_attr_id].dogma == approx(1050000)
    assert api_ship.attrs[eve_sig_affectee_attr_id].dogma == approx(32)
    assert api_ship.attrs[eve_speed_attr_id].dogma == approx(455)
    with check_no_field():
        api_ship.mods  # pylint: disable=W0104


def test_speed_mod_stacking(client, consts):
    # Actual EVE scenario, AB speed boost + black hole speed boost
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity, stackable=False)
    eve_thrust_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_boost_factor)
    eve_speed_boost_attr_prop_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_mass_add_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass_addition)
    eve_prop_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_afterburner,
        cat_id=consts.EveEffCat.active)
    eve_ship_id = client.mk_eve_ship(attrs={eve_speed_attr_id: 455, eve_mass_attr_id: 1050000})
    eve_prop_item_id = client.mk_eve_item(
        attrs={eve_speed_boost_attr_prop_id: 135, eve_thrust_attr_id: 1500000, eve_mass_add_attr_id: 500000},
        eff_ids=[eve_prop_effect_id],
        defeff_id=eve_prop_effect_id)
    eve_speed_boost_attr_sw_id = client.mk_eve_attr()
    eve_sw_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_speed_boost_attr_sw_id,
        affectee_attr_id=eve_speed_attr_id)
    eve_sw_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_sw_mod])
    eve_sw_item_id = client.mk_eve_item(attrs={eve_speed_boost_attr_sw_id: 1.86}, eff_ids=[eve_sw_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_mod(type_id=eve_prop_item_id, rack=consts.ApiRack.mid, state=consts.ApiState.active)
    api_sol.add_sw_effect(type_id=eve_sw_item_id)
    # Verification - if prop speed boost wasn't penalized against BH speed boost, speed would be
    # 1951.95
    api_ship.update()
    assert api_ship.attrs[eve_speed_attr_id].dogma == approx(1833.828883)
    api_ship_mod_prop = api_ship.mods.find_by_affector_attr(
        affectee_attr_id=eve_speed_attr_id, affector_attr_id=eve_speed_boost_attr_prop_id).one()
    assert api_ship_mod_prop.op == consts.ApiModOp.post_mul
    assert api_ship_mod_prop.initial_val == approx(2.306452)
    assert api_ship_mod_prop.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_ship_mod_prop.applied_val == approx(2.306452)
    api_ship_mod_sw = api_ship.mods.find_by_affector_attr(
        affectee_attr_id=eve_speed_attr_id, affector_attr_id=eve_speed_boost_attr_sw_id).one()
    assert api_ship_mod_sw.op == consts.ApiModOp.post_mul
    assert api_ship_mod_sw.initial_val == approx(1.86)
    assert api_ship_mod_sw.stacking_mult == approx(consts.PenaltyStr.p2)
    assert api_ship_mod_sw.applied_val == approx(1.747443)


def test_sig_mod_stacking(client, consts):
    # Actual EVE scenario, MWD sig bloom + shield rigs
    eve_sig_affector_attr_prop_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius_bonus)
    eve_sig_affectee_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius, stackable=False)
    eve_prop_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_microwarpdrive,
        cat_id=consts.EveEffCat.active)
    eve_ship_id = client.mk_eve_ship(attrs={eve_sig_affectee_attr_id: 32})
    eve_prop_item_id = client.mk_eve_item(
        attrs={eve_sig_affector_attr_prop_id: 450},
        eff_ids=[eve_prop_effect_id],
        defeff_id=eve_prop_effect_id)
    eve_sig_affector_attr_rig_id = client.mk_eve_attr()
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_sig_affector_attr_rig_id,
        affectee_attr_id=eve_sig_affectee_attr_id)
    eve_rig_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_rig_mod])
    eve_rig_item_id = client.mk_eve_item(attrs={eve_sig_affector_attr_rig_id: 10}, eff_ids=[eve_rig_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_mod(type_id=eve_prop_item_id, rack=consts.ApiRack.mid, state=consts.ApiState.active)
    api_fit.add_rig(type_id=eve_rig_item_id)
    # Verification - if MWD sig bloom wasn't stacking penalized against rig sig penalty, it'd be
    # 193.6
    api_ship.update()
    assert api_ship.attrs[eve_sig_affectee_attr_id].dogma == approx(191.296512)
    api_ship_mod_prop = api_ship.mods.find_by_affector_attr(
        affectee_attr_id=eve_sig_affectee_attr_id, affector_attr_id=eve_sig_affector_attr_prop_id).one()
    assert api_ship_mod_prop.op == consts.ApiModOp.post_percent
    assert api_ship_mod_prop.initial_val == approx(450)
    assert api_ship_mod_prop.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_ship_mod_prop.applied_val == approx(450)
    api_ship_mod_rig = api_ship.mods.find_by_affector_attr(
        affectee_attr_id=eve_sig_affectee_attr_id, affector_attr_id=eve_sig_affector_attr_rig_id).one()
    assert api_ship_mod_rig.op == consts.ApiModOp.post_percent
    assert api_ship_mod_rig.initial_val == approx(10)
    assert api_ship_mod_rig.stacking_mult == approx(consts.PenaltyStr.p2)
    assert api_ship_mod_rig.applied_val == approx(8.6912)


def test_speed_mod_mass_zero(client, consts):
    # Part of speed boost calculation is division by mass, just check what happens if it's 0
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_thrust_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_boost_factor)
    eve_speed_boost_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_prop_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_afterburner,
        cat_id=consts.EveEffCat.active)
    eve_ship_id = client.mk_eve_ship(attrs={eve_speed_attr_id: 455, eve_mass_attr_id: 0})
    eve_prop_item_id = client.mk_eve_item(
        attrs={eve_speed_boost_attr_id: 135, eve_thrust_attr_id: 1500000},
        eff_ids=[eve_prop_effect_id],
        defeff_id=eve_prop_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_mod(type_id=eve_prop_item_id, rack=consts.ApiRack.mid, state=consts.ApiState.active)
    # Verification
    api_ship.update()
    assert api_ship.attrs[eve_mass_attr_id].dogma == approx(0)
    assert api_ship.attrs[eve_speed_attr_id].dogma == approx(455)
    with check_no_field():
        api_ship.mods  # pylint: disable=W0104


def test_speed_mod_mass_changed(client, consts):
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_thrust_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_boost_factor)
    eve_speed_boost_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_mass_add_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass_addition)
    eve_prop_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_afterburner,
        cat_id=consts.EveEffCat.active)
    eve_ship1_item_id = client.mk_eve_ship(attrs={eve_speed_attr_id: 455, eve_mass_attr_id: 1050000})
    eve_ship2_item_id = client.mk_eve_ship(attrs={eve_speed_attr_id: 420, eve_mass_attr_id: 1060000})
    eve_prop_item_id = client.mk_eve_item(
        attrs={eve_speed_boost_attr_id: 135, eve_thrust_attr_id: 1500000, eve_mass_add_attr_id: 500000},
        eff_ids=[eve_prop_effect_id],
        defeff_id=eve_prop_effect_id)
    eve_mass_boost_attr_id = client.mk_eve_attr()
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mass_boost_attr_id,
        affectee_attr_id=eve_mass_attr_id)
    eve_rig_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_rig_mod])
    eve_rig_item_id = client.mk_eve_item(attrs={eve_mass_boost_attr_id: 100}, eff_ids=[eve_rig_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship1_item = api_fit.set_ship(type_id=eve_ship1_item_id)
    api_prop_item = api_fit.add_mod(type_id=eve_prop_item_id, rack=consts.ApiRack.mid, state=consts.ApiState.active)
    # Verification
    api_ship1_item.update()
    assert api_ship1_item.attrs[eve_mass_attr_id].dogma == approx(1550000)
    assert api_ship1_item.attrs[eve_speed_attr_id].dogma == approx(1049.435484)
    assert len(api_ship1_item.mods[eve_speed_attr_id].find_by_affector_item(affector_item_id=api_prop_item.id)) == 1
    # Action
    api_rig_item = api_fit.add_rig(type_id=eve_rig_item_id)
    # Verification
    api_ship1_item.update()
    assert api_ship1_item.attrs[eve_mass_attr_id].dogma == approx(3100000)
    assert api_ship1_item.attrs[eve_speed_attr_id].dogma == approx(752.217742)
    assert len(api_ship1_item.mods[eve_speed_attr_id].find_by_affector_item(affector_item_id=api_prop_item.id)) == 1
    # Action
    api_rig_item.remove()
    # Verification
    api_ship1_item.update()
    assert api_ship1_item.attrs[eve_mass_attr_id].dogma == approx(1550000)
    assert api_ship1_item.attrs[eve_speed_attr_id].dogma == approx(1049.435484)
    assert len(api_ship1_item.mods[eve_speed_attr_id].find_by_affector_item(affector_item_id=api_prop_item.id)) == 1
    # Action - make sure it works even after we switch ship
    api_ship2_item = api_fit.set_ship(type_id=eve_ship2_item_id)
    # Verification
    api_ship2_item.update()
    assert api_ship2_item.attrs[eve_mass_attr_id].dogma == approx(1560000)
    assert api_ship2_item.attrs[eve_speed_attr_id].dogma == approx(965.192308)
    assert len(api_ship2_item.mods[eve_speed_attr_id].find_by_affector_item(affector_item_id=api_prop_item.id)) == 1
    # Action
    api_rig_item = api_fit.add_rig(type_id=eve_rig_item_id)
    # Verification
    api_ship2_item.update()
    assert api_ship2_item.attrs[eve_mass_attr_id].dogma == approx(3120000)
    assert api_ship2_item.attrs[eve_speed_attr_id].dogma == approx(692.596154)
    assert len(api_ship2_item.mods[eve_speed_attr_id].find_by_affector_item(affector_item_id=api_prop_item.id)) == 1
    # Action
    api_rig_item.remove()
    # Verification
    api_ship2_item.update()
    assert api_ship2_item.attrs[eve_mass_attr_id].dogma == approx(1560000)
    assert api_ship2_item.attrs[eve_speed_attr_id].dogma == approx(965.192308)
    assert len(api_ship2_item.mods[eve_speed_attr_id].find_by_affector_item(affector_item_id=api_prop_item.id)) == 1


def test_speed_mod_boost_changed(client, consts):
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_thrust_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_boost_factor)
    eve_speed_boost_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_mass_add_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass_addition)
    eve_prop_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_afterburner,
        cat_id=consts.EveEffCat.active)
    eve_ship1_item_id = client.mk_eve_ship(attrs={eve_speed_attr_id: 455, eve_mass_attr_id: 1050000})
    eve_ship2_item_id = client.mk_eve_ship(attrs={eve_speed_attr_id: 420, eve_mass_attr_id: 1060000})
    eve_prop_item_id = client.mk_eve_item(
        attrs={eve_speed_boost_attr_id: 135, eve_thrust_attr_id: 1500000, eve_mass_add_attr_id: 500000},
        eff_ids=[eve_prop_effect_id],
        defeff_id=eve_prop_effect_id)
    eve_boost_booster_attr_id = client.mk_eve_attr()
    eve_implant_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_boost_booster_attr_id,
        affectee_attr_id=eve_speed_boost_attr_id)
    eve_implant_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_implant_mod])
    eve_implant_item_id = client.mk_eve_item(attrs={eve_boost_booster_attr_id: 10}, eff_ids=[eve_implant_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship1_item = api_fit.set_ship(type_id=eve_ship1_item_id)
    api_prop_item = api_fit.add_mod(type_id=eve_prop_item_id, rack=consts.ApiRack.mid, state=consts.ApiState.active)
    # Verification
    api_ship1_item.update()
    assert api_ship1_item.attrs[eve_speed_attr_id].dogma == approx(1049.435484)
    assert len(api_ship1_item.mods[eve_speed_attr_id].find_by_affector_item(affector_item_id=api_prop_item.id)) == 1
    assert api_prop_item.update().attrs[eve_speed_boost_attr_id].dogma == approx(135)
    # Action
    api_implant_item = api_fit.add_implant(type_id=eve_implant_item_id)
    # Verification
    api_ship1_item.update()
    assert api_ship1_item.attrs[eve_speed_attr_id].dogma == approx(1108.879032)
    assert len(api_ship1_item.mods[eve_speed_attr_id].find_by_affector_item(affector_item_id=api_prop_item.id)) == 1
    assert api_prop_item.update().attrs[eve_speed_boost_attr_id].dogma == approx(148.5)
    # Action
    api_implant_item.remove()
    # Verification
    api_ship1_item.update()
    assert api_ship1_item.attrs[eve_speed_attr_id].dogma == approx(1049.435484)
    assert len(api_ship1_item.mods[eve_speed_attr_id].find_by_affector_item(affector_item_id=api_prop_item.id)) == 1
    assert api_prop_item.update().attrs[eve_speed_boost_attr_id].dogma == approx(135)
    # Action - make sure it works even after we switch ship
    api_ship2_item = api_fit.set_ship(type_id=eve_ship2_item_id)
    # Verification
    api_ship2_item.update()
    assert api_ship2_item.attrs[eve_speed_attr_id].dogma == approx(965.192308)
    assert len(api_ship2_item.mods[eve_speed_attr_id].find_by_affector_item(affector_item_id=api_prop_item.id)) == 1
    assert api_prop_item.update().attrs[eve_speed_boost_attr_id].dogma == approx(135)
    # Action
    api_implant_item = api_fit.add_implant(type_id=eve_implant_item_id)
    # Verification
    api_ship2_item.update()
    assert api_ship2_item.attrs[eve_speed_attr_id].dogma == approx(1019.711538)
    assert len(api_ship2_item.mods[eve_speed_attr_id].find_by_affector_item(affector_item_id=api_prop_item.id)) == 1
    assert api_prop_item.update().attrs[eve_speed_boost_attr_id].dogma == approx(148.5)
    # Action
    api_implant_item.remove()
    # Verification
    api_ship2_item.update()
    assert api_ship2_item.attrs[eve_speed_attr_id].dogma == approx(965.192308)
    assert len(api_ship2_item.mods[eve_speed_attr_id].find_by_affector_item(affector_item_id=api_prop_item.id)) == 1
    assert api_prop_item.update().attrs[eve_speed_boost_attr_id].dogma == approx(135)


def test_speed_mod_thrust_changed(client, consts):
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_thrust_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_boost_factor)
    eve_speed_boost_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_mass_add_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass_addition)
    eve_prop_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_afterburner,
        cat_id=consts.EveEffCat.active)
    eve_ship1_item_id = client.mk_eve_ship(attrs={eve_speed_attr_id: 455, eve_mass_attr_id: 1050000})
    eve_ship2_item_id = client.mk_eve_ship(attrs={eve_speed_attr_id: 420, eve_mass_attr_id: 1060000})
    eve_prop_item_id = client.mk_eve_item(
        attrs={eve_speed_boost_attr_id: 135, eve_thrust_attr_id: 1500000, eve_mass_add_attr_id: 500000},
        eff_ids=[eve_prop_effect_id],
        defeff_id=eve_prop_effect_id)
    eve_thrust_booster_attr_id = client.mk_eve_attr()
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_thrust_booster_attr_id,
        affectee_attr_id=eve_thrust_attr_id)
    eve_rig_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_rig_mod])
    eve_rig_item_id = client.mk_eve_item(attrs={eve_thrust_booster_attr_id: 30}, eff_ids=[eve_rig_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship1_item = api_fit.set_ship(type_id=eve_ship1_item_id)
    api_prop_item = api_fit.add_mod(type_id=eve_prop_item_id, rack=consts.ApiRack.mid, state=consts.ApiState.active)
    # Verification
    api_ship1_item.update()
    assert api_ship1_item.attrs[eve_speed_attr_id].dogma == approx(1049.435484)
    assert len(api_ship1_item.mods[eve_speed_attr_id].find_by_affector_item(affector_item_id=api_prop_item.id)) == 1
    assert api_prop_item.update().attrs[eve_thrust_attr_id].dogma == approx(1500000)
    # Action
    api_rig_item = api_fit.add_implant(type_id=eve_rig_item_id)
    # Verification
    api_ship1_item.update()
    assert api_ship1_item.attrs[eve_speed_attr_id].dogma == approx(1227.76613)
    assert len(api_ship1_item.mods[eve_speed_attr_id].find_by_affector_item(affector_item_id=api_prop_item.id)) == 1
    assert api_prop_item.update().attrs[eve_thrust_attr_id].dogma == approx(1950000)
    # Action
    api_rig_item.remove()
    # Verification
    api_ship1_item.update()
    assert api_ship1_item.attrs[eve_speed_attr_id].dogma == approx(1049.435484)
    assert len(api_ship1_item.mods[eve_speed_attr_id].find_by_affector_item(affector_item_id=api_prop_item.id)) == 1
    assert api_prop_item.update().attrs[eve_thrust_attr_id].dogma == approx(1500000)
    # Action - make sure it works even after we switch ship
    api_ship2_item = api_fit.set_ship(type_id=eve_ship2_item_id)
    # Verification
    api_ship2_item.update()
    assert api_ship2_item.attrs[eve_speed_attr_id].dogma == approx(965.192308)
    assert len(api_ship2_item.mods[eve_speed_attr_id].find_by_affector_item(affector_item_id=api_prop_item.id)) == 1
    assert api_prop_item.update().attrs[eve_thrust_attr_id].dogma == approx(1500000)
    # Action
    api_rig_item = api_fit.add_implant(type_id=eve_rig_item_id)
    # Verification
    api_ship2_item.update()
    assert api_ship2_item.attrs[eve_speed_attr_id].dogma == approx(1128.75)
    assert len(api_ship2_item.mods[eve_speed_attr_id].find_by_affector_item(affector_item_id=api_prop_item.id)) == 1
    assert api_prop_item.update().attrs[eve_thrust_attr_id].dogma == approx(1950000)
    # Action
    api_rig_item.remove()
    # Verification
    api_ship2_item.update()
    assert api_ship2_item.attrs[eve_speed_attr_id].dogma == approx(965.192308)
    assert len(api_ship2_item.mods[eve_speed_attr_id].find_by_affector_item(affector_item_id=api_prop_item.id)) == 1
    assert api_prop_item.update().attrs[eve_thrust_attr_id].dogma == approx(1500000)
