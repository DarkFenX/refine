from tests import approx, check_no_field


def test_sec_status_switch(client, consts):
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.pilot_security_status)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(sec_status=4)
    api_item = api_fit.set_ship(type_id=eve_item_id)
    # Verification
    assert api_fit.update().sec_status == 4
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(400)
    api_item.update()
    assert api_item.attrs[eve_affector_attr_id].base == approx(0)
    assert api_item.attrs[eve_affector_attr_id].dogma == approx(4)
    assert api_item.attrs[eve_affector_attr_id].extra == approx(4)
    api_mod = api_item.mods.find_by_affector_item(
        affectee_attr_id=eve_affectee_attr_id,
        affector_item_id=api_item.id).one()
    assert api_mod.op == consts.ApiModOp.post_mul
    assert api_mod.initial_val == approx(4)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(4)
    # Action
    api_fit.change(sec_status=-9.9)
    # Verification - have to update item 2nd time, since on 1st pass affector attribute is just
    # getting calculated and is not exposed
    assert api_fit.update().sec_status == -9.9
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(-990)
    api_item.update()
    assert api_item.attrs[eve_affector_attr_id].base == approx(0)
    assert api_item.attrs[eve_affector_attr_id].dogma == approx(-9.9)
    assert api_item.attrs[eve_affector_attr_id].extra == approx(-9.9)
    api_mod = api_item.mods.find_by_affector_item(
        affectee_attr_id=eve_affectee_attr_id,
        affector_item_id=api_item.id).one()
    assert api_mod.op == consts.ApiModOp.post_mul
    assert api_mod.initial_val == approx(-9.9)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(-9.9)


def test_sec_status_modification(client, consts):
    # Check that modifications of security status attribute are completely ignored
    eve_affectee_attr_id = client.mk_eve_attr(id_=consts.EveAttr.pilot_security_status)
    eve_affector_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: -7, eve_affectee_attr_id: 0}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(sec_status=2)
    api_item = api_fit.set_ship(type_id=eve_item_id)
    # Verification
    assert api_fit.update().sec_status == 2
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].base == approx(0)
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(2)
    assert api_item.attrs[eve_affectee_attr_id].extra == approx(2)
    with check_no_field():
        api_item.mods  # noqa: B018
    # Action
    api_fit.change(sec_status=3)
    # Verification
    assert api_fit.update().sec_status == 3
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].base == approx(0)
    assert api_item.attrs[eve_affectee_attr_id].dogma == approx(3)
    assert api_item.attrs[eve_affectee_attr_id].extra == approx(3)
    with check_no_field():
        api_item.mods  # noqa: B018
