from pytest import approx


def test_affected(client, consts):
    eve_grp = client.mk_eve_item_group()
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.ship,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_rig = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_tgt_attr.id: 100})
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_rig = api_fit.add_rig(type_id=eve_rig.id)
    api_proj_effect = api_ss.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_tgts=[api_ship.id])
    assert api_rig.update().attrs[eve_tgt_attr.id].dogma == approx(120)
    api_proj_effect.remove()
    assert api_rig.update().attrs[eve_tgt_attr.id].dogma == approx(100)


def test_unaffected_other_domain(client, consts):
    # Check that entities from other domains are not affected
    eve_grp = client.mk_eve_item_group()
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.ship,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_implant = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_tgt_attr.id: 100})
    eve_ship = client.mk_eve_item()
    eve_char = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.set_char(type_id=eve_char.id)
    api_implant = api_fit.add_implant(type_id=eve_implant.id)
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_proj_effect = api_ss.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_tgts=[api_ship.id])
    assert api_implant.update().attrs[eve_tgt_attr.id].dogma == approx(100)


def test_unaffected_other_group(client, consts):
    # Check that entities belonging to other item groups are not affected
    eve_grp1 = client.mk_eve_item_group()
    eve_grp2 = client.mk_eve_item_group()
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.ship,
        grp=eve_grp1.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_rig = client.mk_eve_item(grp_id=eve_grp2.id, attrs={eve_tgt_attr.id: 100})
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_rig = api_fit.add_rig(type_id=eve_rig.id)
    api_proj_effect = api_ss.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_tgts=[api_ship.id])
    assert api_rig.update().attrs[eve_tgt_attr.id].dogma == approx(100)


def test_unaffected_other_fit(client, consts):
    # Check that projected modifications are not carried over to another fit
    eve_grp = client.mk_eve_item_group()
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.ship,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_rig = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_tgt_attr.id: 100})
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit1 = api_ss.create_fit()
    api_fit2 = api_ss.create_fit()
    api_ship1 = api_fit1.set_ship(type_id=eve_ship.id)
    api_fit2.set_ship(type_id=eve_ship.id)
    api_rig = api_fit2.add_rig(type_id=eve_rig.id)
    api_proj_effect = api_ss.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_tgts=[api_ship1.id])
    assert api_rig.update().attrs[eve_tgt_attr.id].dogma == approx(100)


def test_replace_parent(client, consts):
    # Modifiers which target items on ship location shouldn't apply when ship isn't set
    eve_grp = client.mk_eve_item_group()
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.ship,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_src_item = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_tgt_item = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_tgt_attr.id: 100})
    eve_ship_item = client.mk_eve_item()
    eve_struct_item = client.mk_eve_item()
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    # Structure shouldn't interfere with this logic, despite rig being able to receive modifications
    # via ship or structure domains
    api_fit.set_struct(type_id=eve_struct_item.id)
    api_ship_item1 = api_fit.set_ship(type_id=eve_ship_item.id)
    api_tgt_item = api_fit.add_rig(type_id=eve_tgt_item.id)
    api_proj_effect = api_ss.add_proj_effect(type_id=eve_src_item.id)
    api_proj_effect.change_proj_effect(add_tgts=[api_ship_item1.id])
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(120)
    api_ship_item1.remove()
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(100)
    api_ship_item2 = api_fit.set_ship(type_id=eve_ship_item.id)
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(100)
    api_proj_effect.change_proj_effect(add_tgts=[api_ship_item2.id])
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(120)
