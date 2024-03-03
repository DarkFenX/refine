from pytest import approx


def test_filter(client, consts):
    # Some missile damage modules affect missiles via on-character attribute. Here we make sure it's
    # applied
    eve_attr_bcs = client.mk_eve_attr()
    eve_attr_char = client.mk_eve_attr(id_=consts.Attr.missile_dmg_mult)
    eve_attr_missile_em = client.mk_eve_attr(id_=consts.Attr.em_dmg)
    eve_attr_missile_therm = client.mk_eve_attr(id_=consts.Attr.therm_dmg)
    eve_attr_missile_kin = client.mk_eve_attr(id_=consts.Attr.kin_dmg)
    eve_attr_missile_expl = client.mk_eve_attr(id_=consts.Attr.expl_dmg)
    eve_effect_online = client.mk_eve_online_effect()
    eve_mod_bcs = client.mk_eve_effect_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.char,
        op=consts.ModOp.pre_mul,
        src_attr_id=eve_attr_bcs.id,
        tgt_attr_id=eve_attr_char.id)
    eve_effect_bcs = client.mk_eve_effect(cat_id=consts.EffCat.online, mod_info=[eve_mod_bcs])
    eve_item_skill1 = client.mk_eve_item(id_=consts.Item.missile_launcher_operation)
    eve_item_skill2 = client.mk_eve_item()
    eve_item_bcs = client.mk_eve_item(
        cat_id=consts.ItemCat.module,
        attrs={eve_attr_bcs.id: 1.1},
        eff_ids=[eve_effect_online.id, eve_effect_bcs.id])
    eve_item_char = client.mk_eve_item(grp_id=consts.ItemGrp.character, attrs={eve_attr_char.id: 1})
    eve_item_launcher = client.mk_eve_item()
    eve_item_missile = client.mk_eve_item(
        attrs={
            eve_attr_missile_em.id: 50, eve_attr_missile_therm.id: 70,
            eve_attr_missile_kin.id: 80, eve_attr_missile_expl.id: 100},
        srqs={eve_item_skill1.id: 1})
    eve_item_nonmissile = client.mk_eve_item(
        attrs={
            eve_attr_missile_em.id: 50, eve_attr_missile_therm.id: 70,
            eve_attr_missile_kin.id: 80, eve_attr_missile_expl.id: 100},
        srqs={eve_item_skill2.id: 1})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.set_char(type_id=eve_item_char.id)
    api_fit.add_mod(type_id=eve_item_bcs.id, rack=consts.Rack.low, state=consts.State.online)
    api_launcher1 = api_fit.add_mod(
        type_id=eve_item_launcher.id,
        rack=consts.Rack.high,
        charge_type_id=eve_item_missile.id)
    api_launcher2 = api_fit.add_mod(
        type_id=eve_item_launcher.id,
        rack=consts.Rack.high,
        charge_type_id=eve_item_nonmissile.id)
    api_launcher1.update()
    api_launcher2.update()
    assert api_launcher1.charge.attrs[eve_attr_missile_em.id].dogma == approx(55)
    assert api_launcher1.charge.attrs[eve_attr_missile_therm.id].dogma == approx(77)
    assert api_launcher1.charge.attrs[eve_attr_missile_kin.id].dogma == approx(88)
    assert api_launcher1.charge.attrs[eve_attr_missile_expl.id].dogma == approx(110)
    assert api_launcher2.charge.attrs[eve_attr_missile_em.id].dogma == approx(50)
    assert api_launcher2.charge.attrs[eve_attr_missile_therm.id].dogma == approx(70)
    assert api_launcher2.charge.attrs[eve_attr_missile_kin.id].dogma == approx(80)
    assert api_launcher2.charge.attrs[eve_attr_missile_expl.id].dogma == approx(100)


def test_penalization(client, consts):
    # There are different things which affect missile damage. Some of them are immune to stacking
    # penalties thanks to their carriers being in immune categories, but some are not - like
    # magnetar, wolf-rayet, and plasma storm effect. Here, we check that character modification is
    # not stacking penalized against those.
    eve_item_skill = client.mk_eve_item(id_=consts.Item.missile_launcher_operation)
    eve_attr_magnetar = client.mk_eve_attr()
    eve_attr_char = client.mk_eve_attr(id_=consts.Attr.missile_dmg_mult)
    eve_attr_missile_em = client.mk_eve_attr(id_=consts.Attr.em_dmg, stackable=False)
    eve_attr_missile_therm = client.mk_eve_attr(id_=consts.Attr.therm_dmg, stackable=False)
    eve_attr_missile_kin = client.mk_eve_attr(id_=consts.Attr.kin_dmg, stackable=False)
    eve_attr_missile_expl = client.mk_eve_attr(id_=consts.Attr.expl_dmg, stackable=False)
    # Magnetar, wolf-rayet and plasma storm use post multiplication
    eve_mod_magnetar_em = client.mk_eve_effect_mod(
        func=consts.ModFunc.own_srq,
        dom=consts.ModDom.char,
        srq=eve_item_skill.id,
        op=consts.ModOp.post_mul,
        src_attr_id=eve_attr_magnetar.id,
        tgt_attr_id=eve_attr_missile_em.id)
    eve_mod_magnetar_therm = client.mk_eve_effect_mod(
        func=consts.ModFunc.own_srq,
        dom=consts.ModDom.char,
        srq=eve_item_skill.id,
        op=consts.ModOp.post_mul,
        src_attr_id=eve_attr_magnetar.id,
        tgt_attr_id=eve_attr_missile_therm.id)
    eve_mod_magnetar_kin = client.mk_eve_effect_mod(
        func=consts.ModFunc.own_srq,
        dom=consts.ModDom.char,
        srq=eve_item_skill.id,
        op=consts.ModOp.post_mul,
        src_attr_id=eve_attr_magnetar.id,
        tgt_attr_id=eve_attr_missile_kin.id)
    eve_mod_magnetar_expl = client.mk_eve_effect_mod(
        func=consts.ModFunc.own_srq,
        dom=consts.ModDom.char,
        srq=eve_item_skill.id,
        op=consts.ModOp.post_mul,
        src_attr_id=eve_attr_magnetar.id,
        tgt_attr_id=eve_attr_missile_expl.id)
    eve_effect_magnetar = client.mk_eve_effect(
        cat_id=consts.EffCat.system,
        mod_info=[eve_mod_magnetar_em, eve_mod_magnetar_therm, eve_mod_magnetar_kin, eve_mod_magnetar_expl])
    eve_item_magnetar = client.mk_eve_item(
        grp_id=consts.ItemGrp.effect_beacon,
        cat_id=consts.ItemCat.celestial,
        attrs={eve_attr_magnetar.id: 1.44},
        eff_ids=[eve_effect_magnetar.id])
    eve_item_char = client.mk_eve_item(grp_id=consts.ItemGrp.character, attrs={eve_attr_char.id: 1.3})
    eve_item_launcher = client.mk_eve_item()
    eve_item_missile = client.mk_eve_item(
        attrs={
            eve_attr_missile_em.id: 50, eve_attr_missile_therm.id: 70,
            eve_attr_missile_kin.id: 80, eve_attr_missile_expl.id: 100},
        srqs={eve_item_skill.id: 1})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.set_char(type_id=eve_item_char.id)
    api_ss.add_sw_effect(type_id=eve_item_magnetar.id)
    api_launcher = api_fit.add_mod(
        type_id=eve_item_launcher.id,
        rack=consts.Rack.high,
        charge_type_id=eve_item_missile.id)
    api_launcher.update()
    assert api_launcher.charge.attrs[eve_attr_missile_em.id].dogma == approx(93.6)
    assert api_launcher.charge.attrs[eve_attr_missile_therm.id].dogma == approx(131.04)
    assert api_launcher.charge.attrs[eve_attr_missile_kin.id].dogma == approx(149.76)
    assert api_launcher.charge.attrs[eve_attr_missile_expl.id].dogma == approx(187.2)
