from tests import approx, check_no_field
from tests.fw.api import ValOptions


def test_add_remove(client, consts):
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter_id = client.mk_eve_item(attrs={eve_count_attr_id: 9})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_val = api_fit.validate(options=ValOptions(fighter_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, count=10)
    # Verification
    api_val = api_fit.validate(options=ValOptions(fighter_count=True))
    assert api_val.passed is False
    assert api_val.details.fighter_count == {api_fighter.id: (10, 9)}
    # Action
    api_fighter.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(fighter_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_change(client, consts):
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter_id = client.mk_eve_item(attrs={eve_count_attr_id: 9})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(fighter_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fighter.change_fighter(count=10)
    # Verification
    api_val = api_fit.validate(options=ValOptions(fighter_count=True))
    assert api_val.passed is False
    assert api_val.details.fighter_count == {api_fighter.id: (10, 9)}
    # Action
    api_fighter.change_fighter(count=5)
    # Verification
    api_val = api_fit.validate(options=ValOptions(fighter_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fighter.change_fighter(count=11)
    # Verification
    api_val = api_fit.validate(options=ValOptions(fighter_count=True))
    assert api_val.passed is False
    assert api_val.details.fighter_count == {api_fighter.id: (11, 9)}
    # Action
    api_fighter.change_fighter(count=None)
    # Verification
    api_val = api_fit.validate(options=ValOptions(fighter_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_equal(client, consts):
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter_id = client.mk_eve_item(attrs={eve_count_attr_id: 9})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(fighter_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fighter.change_fighter(count=9)
    # Verification
    api_val = api_fit.validate(options=ValOptions(fighter_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fighter.change_fighter(count=None)
    # Verification
    api_val = api_fit.validate(options=ValOptions(fighter_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_known_failures(client, consts):
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter_id = client.mk_eve_item(attrs={eve_count_attr_id: 9})
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_implant(type_id=eve_other_id)
    api_fighter1 = api_fit.add_fighter(type_id=eve_fighter_id, count=10)
    api_fighter2 = api_fit.add_fighter(type_id=eve_fighter_id, count=11)
    # Verification
    api_val = api_fit.validate(options=ValOptions(fighter_count=(True, [api_fighter1.id])))
    assert api_val.passed is False
    assert api_val.details.fighter_count == {api_fighter2.id: (11, 9)}
    api_val = api_fit.validate(options=ValOptions(fighter_count=(True, [api_fighter2.id])))
    assert api_val.passed is False
    assert api_val.details.fighter_count == {api_fighter1.id: (10, 9)}
    api_val = api_fit.validate(options=ValOptions(fighter_count=(True, [api_fighter1.id, api_fighter2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(
        fighter_count=(True, [api_fighter1.id, api_other.id, api_fighter2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_modified_count(client, consts):
    # Max fighter squad size is never modified, so the lib just uses unmodified attributes for
    # faster access to the attr value
    eve_skill_id = client.mk_eve_item()
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_count_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_fighter_id = client.mk_eve_item(attrs={eve_count_attr_id: 9}, srqs={eve_skill_id: 1})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 6}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    assert api_fighter.update().attrs[eve_count_attr_id].extra == approx(6)
    api_val = api_fit.validate(options=ValOptions(fighter_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fighter.change_fighter(count=10)
    # Verification
    assert api_fighter.update().attrs[eve_count_attr_id].extra == approx(6)
    api_val = api_fit.validate(options=ValOptions(fighter_count=True))
    assert api_val.passed is False
    assert api_val.details.fighter_count == {api_fighter.id: (10, 9)}
    # Action
    api_implant.remove()
    # Verification
    assert api_fighter.update().attrs[eve_count_attr_id].extra == approx(9)
    api_val = api_fit.validate(options=ValOptions(fighter_count=True))
    assert api_val.passed is False
    assert api_val.details.fighter_count == {api_fighter.id: (10, 9)}
    # Action
    api_fighter.change_fighter(count=None)
    # Verification
    assert api_fighter.update().attrs[eve_count_attr_id].extra == approx(9)
    api_val = api_fit.validate(options=ValOptions(fighter_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_rounding(client, consts):
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter1_id = client.mk_eve_item(attrs={eve_count_attr_id: 8.6})
    eve_fighter2_id = client.mk_eve_item(attrs={eve_count_attr_id: 8.4})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_fighter(type_id=eve_fighter1_id, count=9)
    api_fighter2 = api_fit.add_fighter(type_id=eve_fighter2_id, count=9)
    # Verification
    api_val = api_fit.validate(options=ValOptions(fighter_count=True))
    assert api_val.passed is False
    assert api_val.details.fighter_count == {api_fighter2.id: (9, 8)}


def test_not_loaded(client, consts):
    eve_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter_id = client.alloc_item_id()
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_count_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_fighter(type_id=eve_fighter_id, count=10)
    # Verification
    api_val = api_fit.validate(options=ValOptions(fighter_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_no_attr(client, consts):
    eve_count_attr_id = consts.EveAttr.ftr_sq_max_size
    eve_fighter_id = client.mk_eve_item(attrs={eve_count_attr_id: 9})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_val = api_fit.validate(options=ValOptions(fighter_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, count=10)
    # Verification
    api_val = api_fit.validate(options=ValOptions(fighter_count=True))
    assert api_val.passed is False
    assert api_val.details.fighter_count == {api_fighter.id: (10, 9)}
