from tests import approx


def test_modification(client, consts):
    # Check that mutated value is used as base for source and target of modifications
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_base_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20, eve_affectee_attr.id: 200})
    eve_mutated_item = client.mk_eve_item(eff_ids=[eve_effect.id])
    eve_mutator = client.mk_eve_mutator(
        items=[([eve_base_item.id], eve_mutated_item.id)],
        attributes={eve_affector_attr.id: (0.8, 1.2), eve_affectee_attr.id: (0.8, 1.2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_base_item.id, mutation=(eve_mutator.id, {
        eve_affector_attr.id: {consts.ApiAttrMutation.roll: 0.2},
        eve_affectee_attr.id: {consts.ApiAttrMutation.roll: 0.8}}))
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affector_attr.id].base == approx(17.6)
    assert api_item.attrs[eve_affector_attr.id].dogma == approx(17.6)
    assert api_item.attrs[eve_affectee_attr.id].base == approx(224)
    assert api_item.attrs[eve_affectee_attr.id].dogma == approx(263.424)
