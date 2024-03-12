from pytest import approx


def test_slots(client, consts):
    eve_src_attr_hi = client.mk_eve_attr(id_=consts.EveAttr.hi_slot_modifier)
    eve_src_attr_mid = client.mk_eve_attr(id_=consts.EveAttr.med_slot_modifier)
    eve_src_attr_low = client.mk_eve_attr(id_=consts.EveAttr.low_slot_modifier)
    eve_tgt_attr_hi = client.mk_eve_attr(id_=consts.EveAttr.hi_slots)
    eve_tgt_attr_mid = client.mk_eve_attr(id_=consts.EveAttr.med_slots)
    eve_tgt_attr_low = client.mk_eve_attr(id_=consts.EveAttr.low_slots)
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.slot_modifier)
    eve_subsystem = client.mk_eve_item(
        attrs={eve_src_attr_hi.id: 3, eve_src_attr_mid.id: 4, eve_src_attr_low.id: 1},
        eff_ids=[eve_effect.id])
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr_hi.id: 0, eve_tgt_attr_mid.id: 2, eve_tgt_attr_low.id: 1})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_fit.add_subsystem(type_id=eve_subsystem.id)
    api_ship.update()
    assert api_ship.attrs[eve_tgt_attr_hi.id].dogma == approx(3)
    assert api_ship.attrs[eve_tgt_attr_mid.id].dogma == approx(6)
    assert api_ship.attrs[eve_tgt_attr_low.id].dogma == approx(2)


def test_hardpoints(client, consts):
    eve_src_attr_turret = client.mk_eve_attr(id_=consts.EveAttr.turret_hardpoint_modifier)
    eve_src_attr_launcher = client.mk_eve_attr(id_=consts.EveAttr.launcher_hardpoint_modifier)
    eve_tgt_attr_turret = client.mk_eve_attr(id_=consts.EveAttr.turret_slots_left)
    eve_tgt_attr_launcher = client.mk_eve_attr(id_=consts.EveAttr.launcher_slots_left)
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.hardpoint_modifier_effect)
    eve_subsystem = client.mk_eve_item(
        attrs={eve_src_attr_turret.id: 4, eve_src_attr_launcher.id: 6},
        eff_ids=[eve_effect.id])
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr_turret.id: 0, eve_tgt_attr_launcher.id: 2})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_fit.add_subsystem(type_id=eve_subsystem.id)
    api_ship.update()
    assert api_ship.attrs[eve_tgt_attr_turret.id].dogma == approx(4)
    assert api_ship.attrs[eve_tgt_attr_launcher.id].dogma == approx(8)
