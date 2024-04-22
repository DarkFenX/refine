from pytest import approx


def test_affected_via_ship(client, consts):
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.char,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_implant = client.mk_eve_item(attrs={eve_tgt_attr.id: 100})
    eve_char = client.mk_eve_item()
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.set_char(type_id=eve_char.id)
    api_implant = api_fit.add_implant(type_id=eve_implant.id)
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_proj_effect = api_ss.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_tgts=[api_ship.id])
    assert api_implant.update().attrs[eve_tgt_attr.id].dogma == approx(120)
    api_proj_effect.remove()
    assert api_implant.update().attrs[eve_tgt_attr.id].dogma == approx(100)


def test_affected_via_struct(client, consts):
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.char,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_implant = client.mk_eve_item(attrs={eve_tgt_attr.id: 100})
    eve_char = client.mk_eve_item()
    eve_struct = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.set_char(type_id=eve_char.id)
    api_implant = api_fit.add_implant(type_id=eve_implant.id)
    api_struct = api_fit.set_struct(type_id=eve_struct.id)
    api_proj_effect = api_ss.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_tgts=[api_struct.id])
    assert api_implant.update().attrs[eve_tgt_attr.id].dogma == approx(120)
    api_proj_effect.remove()
    assert api_implant.update().attrs[eve_tgt_attr.id].dogma == approx(100)


def test_unaffected_other_domain(client, consts):
    # Check that entities from other domains are not affected
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.char,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_rig = client.mk_eve_item(attrs={eve_tgt_attr.id: 100})
    eve_char = client.mk_eve_item()
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.set_char(type_id=eve_char.id)
    api_rig = api_fit.add_rig(type_id=eve_rig.id)
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_proj_effect = api_ss.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_tgts=[api_ship.id])
    assert api_rig.update().attrs[eve_tgt_attr.id].dogma == approx(100)


def test_unaffected_other_fit(client, consts):
    # Check that projected modifications are not carried over to another fit
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.char,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_implant = client.mk_eve_item(attrs={eve_tgt_attr.id: 100})
    eve_char = client.mk_eve_item()
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit1 = api_ss.create_fit()
    api_fit2 = api_ss.create_fit()
    api_fit1.set_char(type_id=eve_char.id)
    api_fit2.set_char(type_id=eve_char.id)
    api_implant = api_fit2.add_implant(type_id=eve_implant.id)
    api_ship = api_fit1.set_ship(type_id=eve_ship.id)
    api_proj_effect = api_ss.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_tgts=[api_ship.id])
    assert api_implant.update().attrs[eve_tgt_attr.id].dogma == approx(100)


def test_replace_parent(client, consts):
    # Modifiers which target items on character location shouldn't apply when character isn't set
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.char,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_implant = client.mk_eve_item(attrs={eve_tgt_attr.id: 100})
    eve_char = client.mk_eve_item()
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_char = api_fit.set_char(type_id=eve_char.id)
    api_implant = api_fit.add_implant(type_id=eve_implant.id)
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_proj_effect = api_ss.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_tgts=[api_ship.id])
    assert api_implant.update().attrs[eve_tgt_attr.id].dogma == approx(120)
    api_char.remove()
    assert api_implant.update().attrs[eve_tgt_attr.id].dogma == approx(100)
    api_fit.set_char(type_id=eve_char.id)
    assert api_implant.update().attrs[eve_tgt_attr.id].dogma == approx(120)
