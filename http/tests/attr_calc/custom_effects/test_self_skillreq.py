from pytest import approx


def test_missile_launcher_rof(client, consts):
    # All missile specialization skills use the same effect, which has no modifiers defined in EVE
    # data files. In EVE, they affect missile launchers which have skill which carries the effect
    # as their skill requirement
    eve_src_attr = client.mk_eve_attr(id_=consts.Attr.rof_bonus)
    eve_tgt_attr = client.mk_eve_attr(id_=consts.Attr.speed)
    eve_effect = client.mk_eve_effect(id_=consts.Effect.self_rof)
    eve_skill1 = client.mk_eve_item(attrs={eve_src_attr.id: -20}, eff_ids=[eve_effect.id])
    eve_skill2 = client.mk_eve_item()
    eve_launcher1 = client.mk_eve_item(attrs={eve_tgt_attr.id: 5}, srqs={eve_skill1.id: 5})
    eve_launcher2 = client.mk_eve_item(attrs={eve_tgt_attr.id: 5}, srqs={eve_skill2.id: 5})
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.set_ship(type_id=eve_ship.id)
    api_fit.add_skill(type_id=eve_skill1.id, level=5)
    api_fit.add_skill(type_id=eve_skill2.id, level=5)
    api_launcher1 = api_fit.add_mod(type_id=eve_launcher1.id)
    api_launcher2 = api_fit.add_mod(type_id=eve_launcher2.id)
    assert api_launcher1.update().attrs[eve_tgt_attr.id].dogma == approx(4)
    assert api_launcher2.update().attrs[eve_tgt_attr.id].dogma == approx(5)


def test_missile_damage_em(client, consts):
    # All basic missile skill give damage to all missiles which need this skill
    eve_src_attr = client.mk_eve_attr(id_=consts.Attr.dmg_mult_bonus)
    eve_tgt_attr = client.mk_eve_attr(id_=consts.Attr.em_dmg)
    eve_effect = client.mk_eve_effect(id_=consts.Effect.missile_em_dmg_bonus)
    eve_skill1 = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_skill2 = client.mk_eve_item()
    eve_launcher = client.mk_eve_item()
    eve_charge1 = client.mk_eve_item(attrs={eve_tgt_attr.id: 5}, srqs={eve_skill1.id: 5})
    eve_charge2 = client.mk_eve_item(attrs={eve_tgt_attr.id: 5}, srqs={eve_skill2.id: 5})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.add_skill(type_id=eve_skill1.id, level=5)
    api_fit.add_skill(type_id=eve_skill2.id, level=5)
    api_launcher1 = api_fit.add_mod(type_id=eve_launcher.id, charge_type_id=eve_charge1.id)
    api_launcher2 = api_fit.add_mod(type_id=eve_launcher.id, charge_type_id=eve_charge2.id)
    assert api_launcher1.update().charge.attrs[eve_tgt_attr.id].dogma == approx(6)
    assert api_launcher2.update().charge.attrs[eve_tgt_attr.id].dogma == approx(5)


def test_missile_damage_therm(client, consts):
    # All basic missile skill give damage to all missiles which need this skill
    eve_src_attr = client.mk_eve_attr(id_=consts.Attr.dmg_mult_bonus)
    eve_tgt_attr = client.mk_eve_attr(id_=consts.Attr.therm_dmg)
    eve_effect = client.mk_eve_effect(id_=consts.Effect.missile_therm_dmg_bonus)
    eve_skill1 = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_skill2 = client.mk_eve_item()
    eve_launcher = client.mk_eve_item()
    eve_charge1 = client.mk_eve_item(attrs={eve_tgt_attr.id: 5}, srqs={eve_skill1.id: 5})
    eve_charge2 = client.mk_eve_item(attrs={eve_tgt_attr.id: 5}, srqs={eve_skill2.id: 5})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.add_skill(type_id=eve_skill1.id, level=5)
    api_fit.add_skill(type_id=eve_skill2.id, level=5)
    api_launcher1 = api_fit.add_mod(type_id=eve_launcher.id, charge_type_id=eve_charge1.id)
    api_launcher2 = api_fit.add_mod(type_id=eve_launcher.id, charge_type_id=eve_charge2.id)
    assert api_launcher1.update().charge.attrs[eve_tgt_attr.id].dogma == approx(6)
    assert api_launcher2.update().charge.attrs[eve_tgt_attr.id].dogma == approx(5)


def test_missile_damage_kin(client, consts):
    # All basic missile skill give damage to all missiles which need this skill
    eve_src_attr = client.mk_eve_attr(id_=consts.Attr.dmg_mult_bonus)
    eve_tgt_attr = client.mk_eve_attr(id_=consts.Attr.kin_dmg)
    eve_effect = client.mk_eve_effect(id_=consts.Effect.missile_kin_dmg_bonus)
    eve_skill1 = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_skill2 = client.mk_eve_item()
    eve_launcher = client.mk_eve_item()
    eve_charge1 = client.mk_eve_item(attrs={eve_tgt_attr.id: 5}, srqs={eve_skill1.id: 5})
    eve_charge2 = client.mk_eve_item(attrs={eve_tgt_attr.id: 5}, srqs={eve_skill2.id: 5})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.add_skill(type_id=eve_skill1.id, level=5)
    api_fit.add_skill(type_id=eve_skill2.id, level=5)
    api_launcher1 = api_fit.add_mod(type_id=eve_launcher.id, charge_type_id=eve_charge1.id)
    api_launcher2 = api_fit.add_mod(type_id=eve_launcher.id, charge_type_id=eve_charge2.id)
    assert api_launcher1.update().charge.attrs[eve_tgt_attr.id].dogma == approx(6)
    assert api_launcher2.update().charge.attrs[eve_tgt_attr.id].dogma == approx(5)


def test_missile_damage_expl(client, consts):
    # All basic missile skill give damage to all missiles which need this skill
    eve_src_attr = client.mk_eve_attr(id_=consts.Attr.dmg_mult_bonus)
    eve_tgt_attr = client.mk_eve_attr(id_=consts.Attr.expl_dmg)
    eve_effect = client.mk_eve_effect(id_=consts.Effect.missile_expl_dmg_bonus)
    eve_skill1 = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_skill2 = client.mk_eve_item()
    eve_launcher = client.mk_eve_item()
    eve_charge1 = client.mk_eve_item(attrs={eve_tgt_attr.id: 5}, srqs={eve_skill1.id: 5})
    eve_charge2 = client.mk_eve_item(attrs={eve_tgt_attr.id: 5}, srqs={eve_skill2.id: 5})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.add_skill(type_id=eve_skill1.id, level=5)
    api_fit.add_skill(type_id=eve_skill2.id, level=5)
    api_launcher1 = api_fit.add_mod(type_id=eve_launcher.id, charge_type_id=eve_charge1.id)
    api_launcher2 = api_fit.add_mod(type_id=eve_launcher.id, charge_type_id=eve_charge2.id)
    assert api_launcher1.update().charge.attrs[eve_tgt_attr.id].dogma == approx(6)
    assert api_launcher2.update().charge.attrs[eve_tgt_attr.id].dogma == approx(5)


def test_drone_dmg(client, consts):
    # Almost all the drone skills have the same effect as well
    eve_src_attr = client.mk_eve_attr(id_=consts.Attr.dmg_mult_bonus)
    eve_tgt_attr = client.mk_eve_attr(id_=consts.Attr.dmg_mult)
    eve_effect = client.mk_eve_effect(id_=consts.Effect.drone_dmg_bonus)
    eve_skill1 = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_skill2 = client.mk_eve_item()
    eve_drone1 = client.mk_eve_item(attrs={eve_tgt_attr.id: 5}, srqs={eve_skill1.id: 5})
    eve_drone2 = client.mk_eve_item(attrs={eve_tgt_attr.id: 5}, srqs={eve_skill2.id: 5})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.add_skill(type_id=eve_skill1.id, level=5)
    api_fit.add_skill(type_id=eve_skill2.id, level=5)
    api_drone1 = api_fit.add_drone(type_id=eve_drone1.id)
    api_drone2 = api_fit.add_drone(type_id=eve_drone2.id)
    assert api_drone1.update().attrs[eve_tgt_attr.id].dogma == approx(6)
    assert api_drone2.update().attrs[eve_tgt_attr.id].dogma == approx(5)
