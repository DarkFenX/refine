from pytest import approx


def test_local_aar(client, consts):
    # Check that paste boost works on local ancillary repairer
    eve_src_attr = client.mk_eve_attr(id_=consts.EveAttr.charged_armor_dmg_mult)
    eve_tgt_attr = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.fueled_armor_repair)
    eve_aar_item = client.mk_eve_item(attrs={eve_src_attr.id: 3, eve_tgt_attr.id: 100}, eff_ids=[eve_effect.id])
    eve_paste_item = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_aar_item = api_fit.add_mod(type_id=eve_aar_item.id, rack=consts.ApiRack.low, charge_type_id=eve_paste_item.id)
    # Verification
    api_aar_item.update()
    assert api_aar_item.attrs[eve_tgt_attr.id].dogma == approx(100)
    assert api_aar_item.attrs[eve_tgt_attr.id].extra == approx(300)
    api_mod = api_aar_item.mods[eve_tgt_attr.id].one()
    assert api_mod.val == approx(3)
    assert api_mod.op == consts.ApiModOp.extra_mul
    assert api_mod.src.one().item_id == api_aar_item.id
    assert api_mod.src.one().attr_id == eve_src_attr.id


def test_remote_aar(client, consts):
    # Check that paste boost works on remote ancillary repairer
    eve_src_attr = client.mk_eve_attr(id_=consts.EveAttr.charged_armor_dmg_mult)
    eve_tgt_attr = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.ship_module_arar)
    eve_aar_item = client.mk_eve_item(attrs={eve_src_attr.id: 3, eve_tgt_attr.id: 100}, eff_ids=[eve_effect.id])
    eve_paste_item = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_aar_item = api_fit.add_mod(type_id=eve_aar_item.id, rack=consts.ApiRack.high, charge_type_id=eve_paste_item.id)
    # Verification
    api_aar_item.update()
    assert api_aar_item.attrs[eve_tgt_attr.id].dogma == approx(100)
    assert api_aar_item.attrs[eve_tgt_attr.id].extra == approx(300)
    api_mod = api_aar_item.mods[eve_tgt_attr.id].one()
    assert api_mod.val == approx(3)
    assert api_mod.op == consts.ApiModOp.extra_mul
    assert api_mod.src.one().item_id == api_aar_item.id
    assert api_mod.src.one().attr_id == eve_src_attr.id


def test_charge_switch(client, consts):
    eve_src_attr = client.mk_eve_attr(id_=consts.EveAttr.charged_armor_dmg_mult)
    eve_tgt_attr = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.fueled_armor_repair)
    eve_aar_item = client.mk_eve_item(attrs={eve_src_attr.id: 3, eve_tgt_attr.id: 100}, eff_ids=[eve_effect.id])
    eve_paste_item = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_aar_item = api_fit.add_mod(type_id=eve_aar_item.id, rack=consts.ApiRack.low)
    # Verification
    api_aar_item.update()
    assert api_aar_item.attrs[eve_tgt_attr.id].dogma == approx(100)
    assert api_aar_item.attrs[eve_tgt_attr.id].extra == approx(100)
    assert len(api_aar_item.mods) == 0
    # Action
    api_aar_item.change_mod(charge=eve_paste_item.id)
    # Verification
    api_aar_item.update()
    assert api_aar_item.attrs[eve_tgt_attr.id].dogma == approx(100)
    assert api_aar_item.attrs[eve_tgt_attr.id].extra == approx(300)
    assert len(api_aar_item.mods[eve_tgt_attr.id]) == 1
    # Action
    api_aar_item.change_mod(charge=None)
    # Verification
    api_aar_item.update()
    assert api_aar_item.attrs[eve_tgt_attr.id].dogma == approx(100)
    assert api_aar_item.attrs[eve_tgt_attr.id].extra == approx(100)
    assert len(api_aar_item.mods) == 0


def test_mult_change(client, consts):
    eve_ship_item = client.mk_eve_item()
    eve_aar_src_attr = client.mk_eve_attr(id_=consts.EveAttr.charged_armor_dmg_mult)
    eve_aar_tgt_attr = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_mod_src_attr = client.mk_eve_attr()
    eve_aar_effect = client.mk_eve_effect(id_=consts.EveEffect.fueled_armor_repair)
    eve_aar_item = client.mk_eve_item(
        attrs={eve_aar_src_attr.id: 3, eve_aar_tgt_attr.id: 100},
        eff_ids=[eve_aar_effect.id])
    eve_paste_item = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_mod_src_attr.id,
        tgt_attr_id=eve_aar_src_attr.id)
    eve_mod_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_mod_item = client.mk_eve_item(attrs={eve_mod_src_attr.id: 25}, eff_ids=[eve_mod_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(eve_ship_item.id)
    api_aar_item = api_fit.add_mod(type_id=eve_aar_item.id, rack=consts.ApiRack.low, charge_type_id=eve_paste_item.id)
    # Verification
    api_aar_item.update()
    assert api_aar_item.attrs[eve_aar_tgt_attr.id].dogma == approx(100)
    assert api_aar_item.attrs[eve_aar_tgt_attr.id].extra == approx(300)
    # Action
    api_mod_item = api_fit.add_rig(type_id=eve_mod_item.id)
    # Verification
    api_aar_item.update()
    assert api_aar_item.attrs[eve_aar_tgt_attr.id].dogma == approx(100)
    assert api_aar_item.attrs[eve_aar_tgt_attr.id].extra == approx(375)
    # Action
    api_mod_item.remove()
    # Verification
    api_aar_item.update()
    assert api_aar_item.attrs[eve_aar_tgt_attr.id].dogma == approx(100)
    assert api_aar_item.attrs[eve_aar_tgt_attr.id].extra == approx(300)


def test_penalties(client, consts):
    # Check that paste multiplier is not stacking penalized against other multiplications
    eve_ship_item = client.mk_eve_item()
    eve_aar_src_attr = client.mk_eve_attr(id_=consts.EveAttr.charged_armor_dmg_mult)
    eve_aar_tgt_attr = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount, stackable=False)
    eve_mod_src_attr = client.mk_eve_attr()
    eve_aar_effect = client.mk_eve_effect(id_=consts.EveEffect.fueled_armor_repair)
    eve_aar_item = client.mk_eve_item(
        attrs={eve_aar_src_attr.id: 3, eve_aar_tgt_attr.id: 100},
        eff_ids=[eve_aar_effect.id])
    eve_paste_item = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_mul,
        src_attr_id=eve_mod_src_attr.id,
        tgt_attr_id=eve_aar_tgt_attr.id)
    eve_mod_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_mod_item = client.mk_eve_item(attrs={eve_mod_src_attr.id: 1.5}, eff_ids=[eve_mod_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(eve_ship_item.id)
    api_aar_item = api_fit.add_mod(type_id=eve_aar_item.id, rack=consts.ApiRack.low, charge_type_id=eve_paste_item.id)
    api_fit.add_rig(type_id=eve_mod_item.id)
    # Verification
    api_aar_item.update()
    assert api_aar_item.attrs[eve_aar_tgt_attr.id].dogma == approx(150)
    assert api_aar_item.attrs[eve_aar_tgt_attr.id].extra == approx(450)
