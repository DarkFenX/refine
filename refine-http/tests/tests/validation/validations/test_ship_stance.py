"""
This validator takes role similar to "max subsystem slots" for tech 3 cruisers - raises an error
when non-t3d ship has a stance set.
"""

from fw import check_no_field
from fw.api import ValOptions


def test_switch_ship(client, consts):
    eve_stance_id = client.mk_eve_item()
    eve_confessor_id = client.mk_eve_ship(id_=consts.EveItem.confessor)
    eve_hecate_id = client.mk_eve_ship(id_=consts.EveItem.hecate)
    eve_jackdaw_id = client.mk_eve_ship(id_=consts.EveItem.jackdaw)
    eve_svipul_id = client.mk_eve_ship(id_=consts.EveItem.svipul)
    eve_skua_id = client.mk_eve_ship(id_=consts.EveItem.skua)
    eve_anhinga_id = client.mk_eve_ship(id_=consts.EveItem.anhinga)
    eve_other_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_other_id)
    api_stance = api_fit.set_stance(type_id=eve_stance_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_stance=True))
    assert api_val.passed is False
    assert api_val.details.ship_stance.item_id == api_stance.id
    # Action
    api_fit.set_ship(type_id=eve_confessor_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_stance=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_hecate_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_stance=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_jackdaw_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_stance=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_svipul_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_stance=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_skua_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_stance=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_anhinga_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_stance=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_switch_ship_type_id(client, consts):
    eve_stance_id = client.mk_eve_item()
    eve_confessor_id = client.mk_eve_ship(id_=consts.EveItem.confessor)
    eve_hecate_id = client.mk_eve_ship(id_=consts.EveItem.hecate)
    eve_jackdaw_id = client.mk_eve_ship(id_=consts.EveItem.jackdaw)
    eve_svipul_id = client.mk_eve_ship(id_=consts.EveItem.svipul)
    eve_skua_id = client.mk_eve_ship(id_=consts.EveItem.skua)
    eve_anhinga_id = client.mk_eve_ship(id_=consts.EveItem.anhinga)
    eve_other_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_other_id)
    api_stance = api_fit.set_stance(type_id=eve_stance_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_stance=True))
    assert api_val.passed is False
    assert api_val.details.ship_stance.item_id == api_stance.id
    # Action
    api_ship.change_ship(type_id=eve_confessor_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_stance=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_ship.change_ship(type_id=eve_hecate_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_stance=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_ship.change_ship(type_id=eve_jackdaw_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_stance=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_ship.change_ship(type_id=eve_svipul_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_stance=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_ship.change_ship(type_id=eve_skua_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_stance=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_ship.change_ship(type_id=eve_anhinga_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_stance=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_known_failures(client):
    eve_stance_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship()
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_implant(type_id=eve_other_id)
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_stance = api_fit.set_stance(type_id=eve_stance_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_stance=(True, [api_ship.id])))
    assert api_val.passed is False
    assert api_val.details.ship_stance.item_id == api_stance.id
    api_val = api_fit.validate(options=ValOptions(ship_stance=(True, [api_stance.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(ship_stance=(True, [api_other.id, api_stance.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_no_ship(client):
    eve_stance_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_stance = api_fit.set_stance(type_id=eve_stance_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_stance=True))
    assert api_val.passed is False
    assert api_val.details.ship_stance.item_id == api_stance.id


def test_no_stance(client, consts):
    eve_t3d_id = client.mk_eve_ship(id_=consts.EveItem.confessor)
    eve_other_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_t3d_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_stance=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_other_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_stance=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_not_loaded_ship(client, consts):
    eve_t3d_id = consts.EveItem.confessor
    eve_other_id = client.alloc_item_id()
    eve_stance_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_t3d_id)
    api_stance = api_fit.set_stance(type_id=eve_stance_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_stance=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_other_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_stance=True))
    assert api_val.passed is False
    assert api_val.details.ship_stance.item_id == api_stance.id


def test_not_loaded_stance(client, consts):
    eve_t3d_id = client.mk_eve_ship(id_=consts.EveItem.confessor)
    eve_other_id = client.mk_eve_ship()
    eve_stance_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_t3d_id)
    api_stance = api_fit.set_stance(type_id=eve_stance_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_stance=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_other_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_stance=True))
    assert api_val.passed is False
    assert api_val.details.ship_stance.item_id == api_stance.id
