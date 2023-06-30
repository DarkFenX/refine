from pytest import approx


def test_missile_launcher_rof(client, consts):
    eve_src_attr = client.mk_eve_attr(id_=consts.Attr.rof_bonus)
    eve_tgt_attr = client.mk_eve_attr(id_=consts.Attr.speed)
    eve_effect = client.mk_eve_effect(id_=consts.Effect.self_rof)
    eve_skill1 = client.mk_eve_item(attrs={eve_src_attr.id: -20}, eff_ids=[eve_effect.id])
    eve_skill2 = client.mk_eve_item()
    eve_launcher1 = client.mk_eve_item(attrs={eve_tgt_attr.id: 5}, srqs={eve_skill1.id: 5})
    eve_launcher2 = client.mk_eve_item(attrs={eve_tgt_attr.id: 5}, srqs={eve_skill2.id: 5})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.add_skill(type_id=eve_skill1.id, level=5)
    api_fit.add_skill(type_id=eve_skill2.id, level=5)
    api_launcher1 = api_fit.add_mod(type_id=eve_launcher1.id, state=consts.State.offline)
    api_launcher2 = api_fit.add_mod(type_id=eve_launcher2.id, state=consts.State.offline)
    assert api_launcher1.update().attrs[eve_tgt_attr.id].dogma == approx(4)
    assert api_launcher2.update().attrs[eve_tgt_attr.id].dogma == approx(5)
