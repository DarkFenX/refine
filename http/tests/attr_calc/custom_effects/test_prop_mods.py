from pytest import approx


def test_ab_state(client, consts):
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
    api_prop_item = api_fit.add_mod(type_id=eve_prop_item.id, rack=consts.Rack.mid, state=consts.State.active)
    api_ship_item.update()
    assert api_ship_item.attrs[eve_mass_attr.id].dogma == approx(1550000)
    assert api_ship_item.attrs[eve_sig_tgt_attr.id].dogma == approx(32)
    assert api_ship_item.attrs[eve_speed_attr.id].dogma == approx(455)
    api_prop_item.change_mod(state=consts.State.online)
    api_ship_item.update()
    assert api_ship_item.attrs[eve_mass_attr.id].dogma == approx(1050000)
    assert api_ship_item.attrs[eve_sig_tgt_attr.id].dogma == approx(32)
    assert api_ship_item.attrs[eve_speed_attr.id].dogma == approx(455)


def test_mwd_state(client, consts):
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
    api_prop_item = api_fit.add_mod(type_id=eve_prop_item.id, rack=consts.Rack.mid, state=consts.State.active)
    api_ship_item.update()
    assert api_ship_item.attrs[eve_mass_attr.id].dogma == approx(1550000)
    assert api_ship_item.attrs[eve_sig_tgt_attr.id].dogma == approx(176)
    assert api_ship_item.attrs[eve_speed_attr.id].dogma == approx(455)
    api_prop_item.change_mod(state=consts.State.online)
    api_ship_item.update()
    assert api_ship_item.attrs[eve_mass_attr.id].dogma == approx(1050000)
    assert api_ship_item.attrs[eve_sig_tgt_attr.id].dogma == approx(32)
    assert api_ship_item.attrs[eve_speed_attr.id].dogma == approx(455)
