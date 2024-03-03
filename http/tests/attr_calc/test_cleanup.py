from pytest import approx


def test_item_addition_removal(client, consts):
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_attr3 = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.ship,
        op=consts.ModOp.post_mul,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect1 = client.mk_eve_effect(mod_info=[eve_mod1])
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.ModFunc.loc,
        dom=consts.ModDom.ship,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_attr2.id,
        tgt_attr_id=eve_attr3.id)
    eve_effect2 = client.mk_eve_effect(mod_info=[eve_mod2])
    eve_implant = client.mk_eve_item(attrs={eve_attr1.id: 5}, eff_ids=[eve_effect1.id])
    eve_ship = client.mk_eve_item(attrs={eve_attr2.id: 7.5}, eff_ids=[eve_effect2.id])
    eve_rig = client.mk_eve_item(attrs={eve_attr3.id: 0.5})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.set_ship(type_id=eve_ship.id)
    api_rig = api_fit.add_rig(type_id=eve_rig.id)
    assert api_rig.update().attrs[eve_attr3.id].dogma == approx(0.5375)
    eve_implant = api_fit.add_implant(type_id=eve_implant.id)
    # Added item must clean all already calculated attributes which are now affected by it, to allow
    # recalculation
    assert api_rig.update().attrs[eve_attr3.id].dogma == approx(0.6875)
    eve_implant.remove()
    # Removed item should've triggered cleanup too
    assert api_rig.update().attrs[eve_attr3.id].dogma == approx(0.5375)
