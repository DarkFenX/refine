from pytest import approx


def test_ab(client, consts):
    eve_speed_attr = client.mk_eve_attr(id_=consts.Attr.max_velocity)
    eve_thrust_attr = client.mk_eve_attr(id_=consts.Attr.speed_boost_factor)
    eve_speed_boost_attr = client.mk_eve_attr(id_=consts.Attr.speed_factor)
    eve_mass_attr = client.mk_eve_attr(id_=consts.Attr.mass)
    eve_mass_add_attr = client.mk_eve_attr(id_=consts.Attr.mass_addition)
    eve_sig_src_attr = client.mk_eve_attr(id_=consts.Attr.sig_radius_bonus)
    eve_sig_tgt_attr = client.mk_eve_attr(id_=consts.Attr.sig_radius)
    eve_prop_effect = client.mk_eve_effect(id_=consts.Effect.mod_bonus_afterburner, cat_id=consts.EffCat.active)
    eve_ship_item = client.mk_eve_item(
        attrs={eve_speed_attr.id: 455, eve_mass_attr.id: 1050000, eve_sig_tgt_attr.id: 32})
    eve_prop_item = client.mk_eve_item(
        attrs={
            eve_speed_boost_attr.id: 135, eve_thrust_attr.id: 1500000,
            eve_mass_add_attr.id: 500000, eve_sig_src_attr.id: 200},
        eff_ids=[eve_prop_effect.id],
        defeff_id=eve_prop_effect.id)
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_ship_item = api_fit.set_ship(type_id=eve_ship_item.id)
    api_fit.add_mod(type_id=eve_prop_item.id, rack=consts.Rack.mid, state=consts.State.active)
    api_ship_item.update()
    assert api_ship_item.attrs[eve_mass_attr.id].dogma == approx(1550000)
    assert api_ship_item.attrs[eve_sig_tgt_attr.id].dogma == approx(32)  # Not affected by sig blow
    assert api_ship_item.attrs[eve_speed_attr.id].dogma == approx(1049.43548)


def test_mwd(client, consts):
    eve_speed_attr = client.mk_eve_attr(id_=consts.Attr.max_velocity)
    eve_thrust_attr = client.mk_eve_attr(id_=consts.Attr.speed_boost_factor)
    eve_speed_boost_attr = client.mk_eve_attr(id_=consts.Attr.speed_factor)
    eve_mass_attr = client.mk_eve_attr(id_=consts.Attr.mass)
    eve_mass_add_attr = client.mk_eve_attr(id_=consts.Attr.mass_addition)
    eve_sig_src_attr = client.mk_eve_attr(id_=consts.Attr.sig_radius_bonus)
    eve_sig_tgt_attr = client.mk_eve_attr(id_=consts.Attr.sig_radius)
    eve_prop_effect = client.mk_eve_effect(id_=consts.Effect.mod_bonus_microwarpdrive, cat_id=consts.EffCat.active)
    eve_ship_item = client.mk_eve_item(
        attrs={eve_speed_attr.id: 455, eve_mass_attr.id: 1050000, eve_sig_tgt_attr.id: 32})
    eve_prop_item = client.mk_eve_item(
        attrs={
            eve_speed_boost_attr.id: 505, eve_thrust_attr.id: 1500000,
            eve_mass_add_attr.id: 500000, eve_sig_src_attr.id: 450},
        eff_ids=[eve_prop_effect.id],
        defeff_id=eve_prop_effect.id)
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_ship_item = api_fit.set_ship(type_id=eve_ship_item.id)
    api_fit.add_mod(type_id=eve_prop_item.id, rack=consts.Rack.mid, state=consts.State.active)
    api_ship_item.update()
    assert api_ship_item.attrs[eve_mass_attr.id].dogma == approx(1550000)
    assert api_ship_item.attrs[eve_sig_tgt_attr.id].dogma == approx(176)
    assert api_ship_item.attrs[eve_speed_attr.id].dogma == approx(2678.62903)


def test_speed_mod_stacking(client, consts):
    # Actual EVE scenario, AB speed boost + black hole speed boost
    eve_speed_attr = client.mk_eve_attr(id_=consts.Attr.max_velocity, stackable=False)
    eve_thrust_attr = client.mk_eve_attr(id_=consts.Attr.speed_boost_factor)
    eve_speed_boost_attr_prop = client.mk_eve_attr(id_=consts.Attr.speed_factor)
    eve_mass_attr = client.mk_eve_attr(id_=consts.Attr.mass)
    eve_mass_add_attr = client.mk_eve_attr(id_=consts.Attr.mass_addition)
    eve_prop_effect = client.mk_eve_effect(id_=consts.Effect.mod_bonus_afterburner, cat_id=consts.EffCat.active)
    eve_ship_item = client.mk_eve_item(attrs={eve_speed_attr.id: 455, eve_mass_attr.id: 1050000})
    eve_prop_item = client.mk_eve_item(
        attrs={eve_speed_boost_attr_prop.id: 135, eve_thrust_attr.id: 1500000, eve_mass_add_attr.id: 500000},
        eff_ids=[eve_prop_effect.id],
        defeff_id=eve_prop_effect.id)
    eve_speed_boost_attr_sw = client.mk_eve_attr()
    eve_sw_mod = client.mk_eve_effect_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.ship,
        op=consts.ModOp.post_mul,
        src_attr_id=eve_speed_boost_attr_sw.id,
        tgt_attr_id=eve_speed_attr.id)
    eve_sw_effect = client.mk_eve_effect(cat_id=consts.EffCat.system, mod_info=[eve_sw_mod])
    eve_sw_item = client.mk_eve_item(attrs={eve_speed_boost_attr_sw.id: 1.86}, eff_ids=[eve_sw_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_ship_item = api_fit.set_ship(type_id=eve_ship_item.id)
    api_fit.add_mod(type_id=eve_prop_item.id, rack=consts.Rack.mid, state=consts.State.active)
    api_ss.add_sw_effect(type_id=eve_sw_item.id)
    # If prop speed boost wasn't penalized against BH speed boost, speed would be 1951.95
    assert api_ship_item.update().attrs[eve_speed_attr.id].dogma == approx(1833.82888)


def test_sig_mod_stacking(client, consts):
    # Actual EVE scenario, MWD sig bloom + shield rigs
    eve_sig_src_attr_prop = client.mk_eve_attr(id_=consts.Attr.sig_radius_bonus)
    eve_sig_tgt_attr = client.mk_eve_attr(id_=consts.Attr.sig_radius, stackable=False)
    eve_prop_effect = client.mk_eve_effect(id_=consts.Effect.mod_bonus_microwarpdrive, cat_id=consts.EffCat.active)
    eve_ship_item = client.mk_eve_item(attrs={eve_sig_tgt_attr.id: 32})
    eve_prop_item = client.mk_eve_item(
        attrs={eve_sig_src_attr_prop.id: 450},
        eff_ids=[eve_prop_effect.id],
        defeff_id=eve_prop_effect.id)
    eve_sig_src_attr_rig = client.mk_eve_attr()
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.ship,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_sig_src_attr_rig.id,
        tgt_attr_id=eve_sig_tgt_attr.id)
    eve_rig_effect = client.mk_eve_effect(cat_id=consts.EffCat.passive, mod_info=[eve_rig_mod])
    eve_rig_item = client.mk_eve_item(attrs={eve_sig_src_attr_rig.id: 10}, eff_ids=[eve_rig_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_ship_item = api_fit.set_ship(type_id=eve_ship_item.id)
    api_fit.add_mod(type_id=eve_prop_item.id, rack=consts.Rack.mid, state=consts.State.active)
    api_fit.add_rig(type_id=eve_rig_item.id)
    # If MWD sig bloom wasn't stacking penalized against rig sig penalty, it'd be 193.6
    assert api_ship_item.update().attrs[eve_sig_tgt_attr.id].dogma == approx(191.29651)


def test_speed_mod_mass_zero(client, consts):
    # Part of speed boost calculation is division by mass, just check what happens if it's 0
    eve_speed_attr = client.mk_eve_attr(id_=consts.Attr.max_velocity)
    eve_thrust_attr = client.mk_eve_attr(id_=consts.Attr.speed_boost_factor)
    eve_speed_boost_attr = client.mk_eve_attr(id_=consts.Attr.speed_factor)
    eve_mass_attr = client.mk_eve_attr(id_=consts.Attr.mass)
    eve_prop_effect = client.mk_eve_effect(id_=consts.Effect.mod_bonus_afterburner, cat_id=consts.EffCat.active)
    eve_ship_item = client.mk_eve_item(attrs={eve_speed_attr.id: 455, eve_mass_attr.id: 0})
    eve_prop_item = client.mk_eve_item(
        attrs={eve_speed_boost_attr.id: 135, eve_thrust_attr.id: 1500000},
        eff_ids=[eve_prop_effect.id],
        defeff_id=eve_prop_effect.id)
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_ship_item = api_fit.set_ship(type_id=eve_ship_item.id)
    api_fit.add_mod(type_id=eve_prop_item.id, rack=consts.Rack.mid, state=consts.State.active)
    api_ship_item.update()
    assert api_ship_item.attrs[eve_mass_attr.id].dogma == approx(0)
    assert api_ship_item.attrs[eve_speed_attr.id].dogma == approx(455)


def test_speed_mod_mass_changed(client, consts):
    eve_speed_attr = client.mk_eve_attr(id_=consts.Attr.max_velocity)
    eve_thrust_attr = client.mk_eve_attr(id_=consts.Attr.speed_boost_factor)
    eve_speed_boost_attr = client.mk_eve_attr(id_=consts.Attr.speed_factor)
    eve_mass_attr = client.mk_eve_attr(id_=consts.Attr.mass)
    eve_mass_add_attr = client.mk_eve_attr(id_=consts.Attr.mass_addition)
    eve_prop_effect = client.mk_eve_effect(id_=consts.Effect.mod_bonus_afterburner, cat_id=consts.EffCat.active)
    eve_ship_item = client.mk_eve_item(attrs={eve_speed_attr.id: 455, eve_mass_attr.id: 1050000})
    eve_prop_item = client.mk_eve_item(
        attrs={eve_speed_boost_attr.id: 135, eve_thrust_attr.id: 1500000, eve_mass_add_attr.id: 500000},
        eff_ids=[eve_prop_effect.id],
        defeff_id=eve_prop_effect.id)
    eve_mass_boost_attr = client.mk_eve_attr()
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.ship,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_mass_boost_attr.id,
        tgt_attr_id=eve_mass_attr.id)
    eve_rig_effect = client.mk_eve_effect(cat_id=consts.EffCat.passive, mod_info=[eve_rig_mod])
    eve_rig_item = client.mk_eve_item(attrs={eve_mass_boost_attr.id: 100}, eff_ids=[eve_rig_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_ship_item = api_fit.set_ship(type_id=eve_ship_item.id)
    api_fit.add_mod(type_id=eve_prop_item.id, rack=consts.Rack.mid, state=consts.State.active)
    api_ship_item.update()
    assert api_ship_item.attrs[eve_mass_attr.id].dogma == approx(1550000)
    assert api_ship_item.attrs[eve_speed_attr.id].dogma == approx(1049.43548)
    api_rig_item = api_fit.add_rig(type_id=eve_rig_item.id)
    api_ship_item.update()
    assert api_ship_item.attrs[eve_mass_attr.id].dogma == approx(3100000)
    assert api_ship_item.attrs[eve_speed_attr.id].dogma == approx(752.21774)
    api_rig_item.remove()
    api_ship_item.update()
    assert api_ship_item.attrs[eve_mass_attr.id].dogma == approx(1550000)
    assert api_ship_item.attrs[eve_speed_attr.id].dogma == approx(1049.43548)


def test_speed_mod_boost_changed(client, consts):
    eve_speed_attr = client.mk_eve_attr(id_=consts.Attr.max_velocity)
    eve_thrust_attr = client.mk_eve_attr(id_=consts.Attr.speed_boost_factor)
    eve_speed_boost_attr = client.mk_eve_attr(id_=consts.Attr.speed_factor)
    eve_mass_attr = client.mk_eve_attr(id_=consts.Attr.mass)
    eve_mass_add_attr = client.mk_eve_attr(id_=consts.Attr.mass_addition)
    eve_prop_effect = client.mk_eve_effect(id_=consts.Effect.mod_bonus_afterburner, cat_id=consts.EffCat.active)
    eve_ship_item = client.mk_eve_item(attrs={eve_speed_attr.id: 455, eve_mass_attr.id: 1050000})
    eve_prop_item = client.mk_eve_item(
        attrs={eve_speed_boost_attr.id: 135, eve_thrust_attr.id: 1500000, eve_mass_add_attr.id: 500000},
        eff_ids=[eve_prop_effect.id],
        defeff_id=eve_prop_effect.id)
    eve_boost_booster_attr = client.mk_eve_attr()
    eve_implant_mod = client.mk_eve_effect_mod(
        func=consts.ModFunc.loc,
        dom=consts.ModDom.ship,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_boost_booster_attr.id,
        tgt_attr_id=eve_speed_boost_attr.id)
    eve_implant_effect = client.mk_eve_effect(cat_id=consts.EffCat.passive, mod_info=[eve_implant_mod])
    eve_implant_item = client.mk_eve_item(attrs={eve_boost_booster_attr.id: 10}, eff_ids=[eve_implant_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_ship_item = api_fit.set_ship(type_id=eve_ship_item.id)
    api_prop_item = api_fit.add_mod(type_id=eve_prop_item.id, rack=consts.Rack.mid, state=consts.State.active)
    assert api_prop_item.update().attrs[eve_speed_boost_attr.id].dogma == approx(135)
    assert api_ship_item.update().attrs[eve_speed_attr.id].dogma == approx(1049.43548)
    api_implant_item = api_fit.add_implant(type_id=eve_implant_item.id)
    assert api_prop_item.update().attrs[eve_speed_boost_attr.id].dogma == approx(148.5)
    assert api_ship_item.update().attrs[eve_speed_attr.id].dogma == approx(1108.87903)
    api_implant_item.remove()
    assert api_prop_item.update().attrs[eve_speed_boost_attr.id].dogma == approx(135)
    assert api_ship_item.update().attrs[eve_speed_attr.id].dogma == approx(1049.43548)


def test_speed_mod_thrust_changed(client, consts):
    eve_speed_attr = client.mk_eve_attr(id_=consts.Attr.max_velocity)
    eve_thrust_attr = client.mk_eve_attr(id_=consts.Attr.speed_boost_factor)
    eve_speed_boost_attr = client.mk_eve_attr(id_=consts.Attr.speed_factor)
    eve_mass_attr = client.mk_eve_attr(id_=consts.Attr.mass)
    eve_mass_add_attr = client.mk_eve_attr(id_=consts.Attr.mass_addition)
    eve_prop_effect = client.mk_eve_effect(id_=consts.Effect.mod_bonus_afterburner, cat_id=consts.EffCat.active)
    eve_ship_item = client.mk_eve_item(attrs={eve_speed_attr.id: 455, eve_mass_attr.id: 1050000})
    eve_prop_item = client.mk_eve_item(
        attrs={eve_speed_boost_attr.id: 135, eve_thrust_attr.id: 1500000, eve_mass_add_attr.id: 500000},
        eff_ids=[eve_prop_effect.id],
        defeff_id=eve_prop_effect.id)
    eve_thrust_booster_attr = client.mk_eve_attr()
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.ModFunc.loc,
        dom=consts.ModDom.ship,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_thrust_booster_attr.id,
        tgt_attr_id=eve_thrust_attr.id)
    eve_rig_effect = client.mk_eve_effect(cat_id=consts.EffCat.passive, mod_info=[eve_rig_mod])
    eve_rig_item = client.mk_eve_item(attrs={eve_thrust_booster_attr.id: 30}, eff_ids=[eve_rig_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_ship_item = api_fit.set_ship(type_id=eve_ship_item.id)
    api_prop_item = api_fit.add_mod(type_id=eve_prop_item.id, rack=consts.Rack.mid, state=consts.State.active)
    assert api_prop_item.update().attrs[eve_thrust_attr.id].dogma == approx(1500000)
    assert api_ship_item.update().attrs[eve_speed_attr.id].dogma == approx(1049.43548)
    api_rig_item = api_fit.add_implant(type_id=eve_rig_item.id)
    assert api_prop_item.update().attrs[eve_thrust_attr.id].dogma == approx(1950000)
    assert api_ship_item.update().attrs[eve_speed_attr.id].dogma == approx(1227.76613)
    api_rig_item.remove()
    assert api_prop_item.update().attrs[eve_thrust_attr.id].dogma == approx(1500000)
    assert api_ship_item.update().attrs[eve_speed_attr.id].dogma == approx(1049.43548)
