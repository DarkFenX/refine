from pytest import approx


def test_calculation(client, consts):
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_attr3 = client.mk_eve_attr()
    eve_attr4 = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.item,
        op=consts.ModOp.post_mul,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect1 = client.mk_eve_effect(mod_info=[eve_mod1])
    eve_mod2 = client.mk_eve_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.ship,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_attr2.id,
        tgt_attr_id=eve_attr3.id)
    eve_effect2 = client.mk_eve_effect(mod_info=[eve_mod2])
    eve_mod3 = client.mk_eve_mod(
        func=consts.ModFunc.loc,
        dom=consts.ModDom.ship,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_attr3.id,
        tgt_attr_id=eve_attr4.id)
    eve_effect3 = client.mk_eve_effect(mod_info=[eve_mod3])
    eve_implant = client.mk_eve_item(
        attrs={eve_attr1.id: 5, eve_attr2.id: 20},
        eff_ids=[eve_effect1.id, eve_effect2.id])
    eve_ship = client.mk_eve_item(attrs={eve_attr3.id: 150}, eff_ids=[eve_effect3.id])
    eve_rig = client.mk_eve_item(attrs={eve_attr4.id: 12.5})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.set_ship(type_id=eve_ship.id)
    api_rig = api_fit.add_rig(type_id=eve_rig.id)
    api_fit.add_implant(type_id=eve_implant.id)
    value = api_rig.update().attr_vals[eve_attr4.id].dogma
    # If everything is processed properly, item1 will multiply attr2 by
    # attr1 on self, resulting in 20 * 5 = 100, then apply it as percentage
    # modifier on ship's (item2) attr3, resulting in 150 + 100% = 300, then
    # it is applied to all entities assigned to ship, including item3, to
    # theirs attr4 as percentage modifier again - so final result is 12.5 +
    # 300% = 50
    assert value == approx(50)
