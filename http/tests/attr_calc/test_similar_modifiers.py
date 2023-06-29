from pytest import approx


def test_same_item_different_effects_attrs(client, consts):
    # Reflects currently real EVE scenario: 2 different skills affect 2 separate
    # attributes on capital ships, which, in turn, affect ship agility via 2
    # different on-ship effects
    eve_src_attr1 = client.mk_eve_attr()
    eve_src_attr2 = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.item,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_src_attr1.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect1 = client.mk_eve_effect(mod_info=[eve_mod1])
    eve_mod2 = client.mk_eve_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.item,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_src_attr1.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect2 = client.mk_eve_effect(mod_info=[eve_mod2])
    eve_item = client.mk_eve_item(
        attrs={eve_src_attr1.id: 20, eve_src_attr2.id: 20, eve_tgt_attr.id: 100},
        eff_ids=[eve_effect1.id, eve_effect2.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.set_char(type_id=eve_item.id)
    value = api_item.update().attrs[eve_tgt_attr.id].dogma
    assert value == approx(144)


def test_same_item_attr_different_effects(client, consts):
    # Reflects currently real EVE scenario: capital hull repair systems have
    # both capital and sub-capital repair systems skills in their direct
    # requirements. There are some items which affect both (e.g. nanobot
    # accelerator rigs). Despite having two different effects, modification is
    # applied only once in game
    eve_skill1 = client.mk_eve_item()
    eve_skill2 = client.mk_eve_item()
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr(stackable=True)
    eve_mod1 = client.mk_eve_mod(
        func=consts.ModFunc.loc_srq,
        dom=consts.ModDom.ship,
        srq=eve_skill1.id,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect1 = client.mk_eve_effect(mod_info=[eve_mod1])
    eve_mod2 = client.mk_eve_mod(
        func=consts.ModFunc.loc_srq,
        dom=consts.ModDom.ship,
        srq=eve_skill2.id,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect2 = client.mk_eve_effect(mod_info=[eve_mod2])
    eve_src_item = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect1.id, eve_effect2.id])
    eve_tgt_item = client.mk_eve_item(attrs={eve_tgt_attr.id: 100}, srqs={eve_skill1.id: 1, eve_skill2.id: 1})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.add_rig(type_id=eve_src_item.id)
    api_item = api_fit.add_mod(type_id=eve_tgt_item.id, rack=consts.Rack.mid)
    value = api_item.update().attrs[eve_tgt_attr.id].dogma
    assert value == approx(120)
