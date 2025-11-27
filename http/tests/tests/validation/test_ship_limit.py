from tests import approx, check_no_field
from tests.fw.api import ValOptions


def test_type_single(client, consts):
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type1, unit_id=consts.EveAttrUnit.item_id)
    eve_allowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_module_id = client.mk_eve_item(attrs={eve_type_attr_id: eve_allowed_ship_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_disallowed_ship_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.items == {api_module.id: ([eve_allowed_ship_id], [])}
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_type_multiple_different(client, consts):
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_type_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type1, unit_id=consts.EveAttrUnit.item_id)
    eve_type_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type2, unit_id=consts.EveAttrUnit.item_id)
    eve_allowed_ship1_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_allowed_ship2_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_type_attr1_id: eve_allowed_ship1_id, eve_type_attr2_id: eve_allowed_ship2_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_disallowed_ship_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.items == {
        api_module.id: (sorted([eve_allowed_ship1_id, eve_allowed_ship2_id]), [])}
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_type_multiple_same_rounding(client, consts):
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_type_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type1, unit_id=consts.EveAttrUnit.item_id)
    eve_type_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type2, unit_id=consts.EveAttrUnit.item_id)
    eve_type_attr3_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type3, unit_id=consts.EveAttrUnit.item_id)
    eve_allowed_ship1_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_allowed_ship2_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_module_id = client.mk_eve_item(attrs={
        eve_type_attr1_id: eve_allowed_ship1_id - 0.4,
        eve_type_attr2_id: eve_allowed_ship2_id,
        eve_type_attr3_id: eve_allowed_ship1_id + 0.4})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_disallowed_ship_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.items == {
        api_module.id: (sorted([eve_allowed_ship1_id, eve_allowed_ship2_id]), [])}
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship1_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_group_single(client, consts):
    eve_allowed_grp_id = client.mk_eve_ship_group()
    eve_disallowed_grp_id = client.mk_eve_ship_group()
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_allowed_ship_id = client.mk_eve_ship(grp_id=eve_allowed_grp_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_disallowed_grp_id)
    eve_module_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_allowed_grp_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_disallowed_ship_id
    assert api_val.details.ship_limit.ship_group_id == eve_disallowed_grp_id
    assert api_val.details.ship_limit.items == {api_module.id: ([], [eve_allowed_grp_id])}
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_group_multiple_different(client, consts):
    eve_allowed_grp1_id = client.mk_eve_ship_group()
    eve_allowed_grp2_id = client.mk_eve_ship_group()
    eve_disallowed_grp_id = client.mk_eve_ship_group()
    eve_group_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_group_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group2, unit_id=consts.EveAttrUnit.group_id)
    eve_allowed_ship_id = client.mk_eve_ship(grp_id=eve_allowed_grp2_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_disallowed_grp_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_group_attr1_id: eve_allowed_grp1_id, eve_group_attr2_id: eve_allowed_grp2_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_disallowed_ship_id
    assert api_val.details.ship_limit.ship_group_id == eve_disallowed_grp_id
    assert api_val.details.ship_limit.items == {
        api_module.id: ([], sorted([eve_allowed_grp1_id, eve_allowed_grp2_id]))}
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_group_multiple_same_rounding(client, consts):
    eve_allowed_grp1_id = client.mk_eve_ship_group()
    eve_allowed_grp2_id = client.mk_eve_ship_group()
    eve_disallowed_grp_id = client.mk_eve_ship_group()
    eve_group_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_group_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group2, unit_id=consts.EveAttrUnit.group_id)
    eve_group_attr3_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group3, unit_id=consts.EveAttrUnit.group_id)
    eve_allowed_ship_id = client.mk_eve_ship(grp_id=eve_allowed_grp1_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_disallowed_grp_id)
    eve_module_id = client.mk_eve_item(attrs={
        eve_group_attr1_id: eve_allowed_grp1_id - 0.4,
        eve_group_attr2_id: eve_allowed_grp2_id,
        eve_group_attr3_id: eve_allowed_grp1_id + 0.4})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_disallowed_ship_id
    assert api_val.details.ship_limit.ship_group_id == eve_disallowed_grp_id
    assert api_val.details.ship_limit.items == {
        api_module.id: ([], sorted([eve_allowed_grp1_id, eve_allowed_grp2_id]))}
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_combined(client, consts):
    eve_allowed_grp1_id = client.mk_eve_ship_group()
    eve_allowed_grp2_id = client.mk_eve_ship_group()
    eve_disallowed_grp_id = client.mk_eve_ship_group()
    eve_type_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type8, unit_id=consts.EveAttrUnit.item_id)
    eve_type_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type10, unit_id=consts.EveAttrUnit.item_id)
    eve_group_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group4, unit_id=consts.EveAttrUnit.group_id)
    eve_group_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group9, unit_id=consts.EveAttrUnit.group_id)
    eve_allowed_type_ship_id = client.mk_eve_ship(grp_id=eve_disallowed_grp_id)
    eve_allowed_group_ship_id = client.mk_eve_ship(grp_id=eve_allowed_grp2_id)
    eve_allowed_both_ship_id = client.mk_eve_ship(grp_id=eve_allowed_grp1_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_disallowed_grp_id)
    eve_module_id = client.mk_eve_item(attrs={
        eve_type_attr1_id: eve_allowed_type_ship_id,
        eve_type_attr2_id: eve_allowed_both_ship_id,
        eve_group_attr1_id: eve_allowed_grp1_id,
        eve_group_attr2_id: eve_allowed_grp2_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_disallowed_ship_id
    assert api_val.details.ship_limit.ship_group_id == eve_disallowed_grp_id
    assert api_val.details.ship_limit.items == {api_module.id: (
        sorted([eve_allowed_type_ship_id, eve_allowed_both_ship_id]),
        sorted([eve_allowed_grp1_id, eve_allowed_grp2_id]))}
    # Action
    api_fit.set_ship(type_id=eve_allowed_type_ship_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_allowed_group_ship_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_allowed_both_ship_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_struct(client, consts):
    eve_struct_grp_id = client.mk_eve_struct_group()
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type1, unit_id=consts.EveAttrUnit.item_id)
    eve_allowed_struct_id = client.mk_eve_struct(grp_id=eve_struct_grp_id)
    eve_disallowed_struct_id = client.mk_eve_struct(grp_id=eve_struct_grp_id)
    eve_module_id = client.mk_eve_item(attrs={eve_type_attr_id: eve_allowed_struct_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_struct_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_disallowed_struct_id
    assert api_val.details.ship_limit.ship_group_id == eve_struct_grp_id
    assert api_val.details.ship_limit.items == {api_module.id: ([eve_allowed_struct_id], [])}
    # Action
    api_fit.set_ship(type_id=eve_allowed_struct_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_rig(client, consts):
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type12, unit_id=consts.EveAttrUnit.item_id)
    eve_allowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_rig_id = client.mk_eve_item(attrs={eve_type_attr_id: eve_allowed_ship_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_disallowed_ship_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.items == {api_rig.id: ([eve_allowed_ship_id], [])}
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_service(client, consts):
    eve_struct_grp_id = client.mk_eve_struct_group()
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type11, unit_id=consts.EveAttrUnit.item_id)
    eve_allowed_struct_id = client.mk_eve_struct(grp_id=eve_struct_grp_id)
    eve_disallowed_struct_id = client.mk_eve_struct(grp_id=eve_struct_grp_id)
    eve_service_id = client.mk_eve_item(attrs={eve_type_attr_id: eve_allowed_struct_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_struct_id)
    api_service = api_fit.add_service(type_id=eve_service_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_disallowed_struct_id
    assert api_val.details.ship_limit.ship_group_id == eve_struct_grp_id
    assert api_val.details.ship_limit.items == {api_service.id: ([eve_allowed_struct_id], [])}
    # Action
    api_fit.set_ship(type_id=eve_allowed_struct_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_subsystem(client, consts):
    # Subsystems have their special limit attribute
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.fits_to_ship_type, unit_id=consts.EveAttrUnit.item_id)
    eve_allowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_subsystem_id = client.mk_eve_item(attrs={eve_type_attr_id: eve_allowed_ship_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    api_subsystem = api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_disallowed_ship_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.items == {api_subsystem.id: ([eve_allowed_ship_id], [])}
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_t3d_confessor(client, consts):
    # Tactical destroyer on-stance limits are hardcoded, we check it here
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_defensive_id = client.mk_eve_item(id_=consts.EveItem.confessor_defense_mode)
    eve_propulsion_id = client.mk_eve_item(id_=consts.EveItem.confessor_propulsion_mode)
    eve_sharpshooter_id = client.mk_eve_item(id_=consts.EveItem.confessor_sharpshooter_mode)
    eve_t3d_id = client.mk_eve_ship(id_=consts.EveItem.confessor, grp_id=eve_ship_grp_id)
    eve_other_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_other_id)
    api_defensive = api_fit.set_stance(type_id=eve_defensive_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_other_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.items == {api_defensive.id: ([eve_t3d_id], [])}
    # Action
    api_propulsion = api_fit.set_stance(type_id=eve_propulsion_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_other_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.items == {api_propulsion.id: ([eve_t3d_id], [])}
    # Action
    api_sharpshooter = api_fit.set_stance(type_id=eve_sharpshooter_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_other_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.items == {api_sharpshooter.id: ([eve_t3d_id], [])}
    # Action
    api_fit.set_ship(type_id=eve_t3d_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_stance(type_id=eve_propulsion_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_stance(type_id=eve_defensive_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_t3d_hecate(client, consts):
    # Tactical destroyer on-stance limits are hardcoded, we check it here
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_defensive_id = client.mk_eve_item(id_=consts.EveItem.hecate_defense_mode)
    eve_propulsion_id = client.mk_eve_item(id_=consts.EveItem.hecate_propulsion_mode)
    eve_sharpshooter_id = client.mk_eve_item(id_=consts.EveItem.hecate_sharpshooter_mode)
    eve_t3d_id = client.mk_eve_ship(id_=consts.EveItem.hecate, grp_id=eve_ship_grp_id)
    eve_other_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_other_id)
    api_defensive = api_fit.set_stance(type_id=eve_defensive_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_other_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.items == {api_defensive.id: ([eve_t3d_id], [])}
    # Action
    api_propulsion = api_fit.set_stance(type_id=eve_propulsion_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_other_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.items == {api_propulsion.id: ([eve_t3d_id], [])}
    # Action
    api_sharpshooter = api_fit.set_stance(type_id=eve_sharpshooter_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_other_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.items == {api_sharpshooter.id: ([eve_t3d_id], [])}
    # Action
    api_fit.set_ship(type_id=eve_t3d_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_stance(type_id=eve_propulsion_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_stance(type_id=eve_defensive_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_t3d_jackdaw(client, consts):
    # Tactical destroyer on-stance limits are hardcoded, we check it here
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_defensive_id = client.mk_eve_item(id_=consts.EveItem.jackdaw_defense_mode)
    eve_propulsion_id = client.mk_eve_item(id_=consts.EveItem.jackdaw_propulsion_mode)
    eve_sharpshooter_id = client.mk_eve_item(id_=consts.EveItem.jackdaw_sharpshooter_mode)
    eve_t3d_id = client.mk_eve_ship(id_=consts.EveItem.jackdaw, grp_id=eve_ship_grp_id)
    eve_other_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_other_id)
    api_defensive = api_fit.set_stance(type_id=eve_defensive_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_other_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.items == {api_defensive.id: ([eve_t3d_id], [])}
    # Action
    api_propulsion = api_fit.set_stance(type_id=eve_propulsion_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_other_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.items == {api_propulsion.id: ([eve_t3d_id], [])}
    # Action
    api_sharpshooter = api_fit.set_stance(type_id=eve_sharpshooter_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_other_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.items == {api_sharpshooter.id: ([eve_t3d_id], [])}
    # Action
    api_fit.set_ship(type_id=eve_t3d_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_stance(type_id=eve_propulsion_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_stance(type_id=eve_defensive_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_t3d_svipul(client, consts):
    # Tactical destroyer on-stance limits are hardcoded, we check it here
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_defensive_id = client.mk_eve_item(id_=consts.EveItem.svipul_defense_mode)
    eve_propulsion_id = client.mk_eve_item(id_=consts.EveItem.svipul_propulsion_mode)
    eve_sharpshooter_id = client.mk_eve_item(id_=consts.EveItem.svipul_sharpshooter_mode)
    eve_t3d_id = client.mk_eve_ship(id_=consts.EveItem.svipul, grp_id=eve_ship_grp_id)
    eve_other_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_other_id)
    api_defensive = api_fit.set_stance(type_id=eve_defensive_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_other_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.items == {api_defensive.id: ([eve_t3d_id], [])}
    # Action
    api_propulsion = api_fit.set_stance(type_id=eve_propulsion_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_other_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.items == {api_propulsion.id: ([eve_t3d_id], [])}
    # Action
    api_sharpshooter = api_fit.set_stance(type_id=eve_sharpshooter_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_other_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.items == {api_sharpshooter.id: ([eve_t3d_id], [])}
    # Action
    api_fit.set_ship(type_id=eve_t3d_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_stance(type_id=eve_propulsion_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_stance(type_id=eve_defensive_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_known_failures(client, consts):
    eve_grp1_id = client.mk_eve_ship_group()
    eve_grp2_id = client.mk_eve_ship_group()
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type9, unit_id=consts.EveAttrUnit.item_id)
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group18, unit_id=consts.EveAttrUnit.group_id)
    eve_ship_id = client.mk_eve_ship(grp_id=eve_grp1_id)
    eve_other_id = client.mk_eve_item()
    eve_module_id = client.mk_eve_item(attrs={eve_type_attr_id: eve_other_id, eve_group_attr_id: eve_grp2_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_implant(type_id=eve_other_id)
    api_module1 = api_fit.add_module(type_id=eve_module_id)
    api_module2 = api_fit.add_module(type_id=eve_module_id)
    # Verification - no ship case has to be checked as well, since there is no-ship logic
    api_val = api_fit.validate(options=ValOptions(ship_limit=(True, [api_module1.id])))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id is None
    assert api_val.details.ship_limit.ship_group_id is None
    assert api_val.details.ship_limit.items == {api_module2.id: ([eve_other_id], [eve_grp2_id])}
    api_val = api_fit.validate(options=ValOptions(ship_limit=(True, [api_module2.id])))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id is None
    assert api_val.details.ship_limit.ship_group_id is None
    assert api_val.details.ship_limit.items == {api_module1.id: ([eve_other_id], [eve_grp2_id])}
    api_val = api_fit.validate(options=ValOptions(ship_limit=(True, [api_module1.id, api_module2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(ship_limit=(True, [api_module1.id, api_other.id, api_module2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=(True, [api_module1.id])))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_ship_id
    assert api_val.details.ship_limit.ship_group_id == eve_grp1_id
    assert api_val.details.ship_limit.items == {api_module2.id: ([eve_other_id], [eve_grp2_id])}
    api_val = api_fit.validate(options=ValOptions(ship_limit=(True, [api_module2.id])))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_ship_id
    assert api_val.details.ship_limit.ship_group_id == eve_grp1_id
    assert api_val.details.ship_limit.items == {api_module1.id: ([eve_other_id], [eve_grp2_id])}
    api_val = api_fit.validate(options=ValOptions(ship_limit=(True, [api_module1.id, api_module2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(ship_limit=(True, [api_module1.id, api_other.id, api_module2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_modified_type(client, consts):
    # Verification - unrealistic scenario, but testing here detail of implementation: raw values of
    # attributes are taken, their modification does not affect validation
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type1, unit_id=consts.EveAttrUnit.item_id)
    eve_allowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_module_id = client.mk_eve_item(attrs={eve_type_attr_id: eve_allowed_ship_id})
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_type_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: eve_allowed_ship_id}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification
    assert api_module.update().attrs[eve_type_attr_id].extra == approx(eve_allowed_ship_id)
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_disallowed_ship_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.items == {api_module.id: ([eve_allowed_ship_id], [])}
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship_id)
    # Verification
    assert api_module.update().attrs[eve_type_attr_id].extra == approx(eve_allowed_ship_id)
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_modified_group(client, consts):
    # Verification - unrealistic scenario, but testing here detail of implementation: raw values of
    # attributes are taken, their modification does not affect validation
    eve_allowed_grp_id = client.mk_eve_ship_group()
    eve_disallowed_grp_id = client.mk_eve_ship_group()
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_allowed_ship_id = client.mk_eve_ship(grp_id=eve_allowed_grp_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_disallowed_grp_id)
    eve_module_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_allowed_grp_id})
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_group_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: eve_allowed_grp_id}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification
    assert api_module.update().attrs[eve_group_attr_id].extra == approx(eve_allowed_grp_id)
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_disallowed_ship_id
    assert api_val.details.ship_limit.ship_group_id == eve_disallowed_grp_id
    assert api_val.details.ship_limit.items == {api_module.id: ([], [eve_allowed_grp_id])}
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship_id)
    # Verification
    assert api_module.update().attrs[eve_group_attr_id].extra == approx(eve_allowed_grp_id)
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_mutation_type(client, consts):
    # Unrealistic scenario, but we still check what happens if restrictions on base and mutated item
    # are not the same
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_type1_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type1, unit_id=consts.EveAttrUnit.item_id)
    eve_type2_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type2, unit_id=consts.EveAttrUnit.item_id)
    eve_ship1_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_ship2_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_ship3_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_base_module_id = client.mk_eve_item(attrs={eve_type1_attr_id: eve_ship1_id, eve_type2_attr_id: eve_ship2_id})
    eve_mutated_module_id = client.mk_eve_item(attrs={eve_type1_attr_id: eve_ship3_id})
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_module_id], eve_mutated_module_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship3_id)
    api_module = api_fit.add_module(type_id=eve_base_module_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_ship3_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.items == {api_module.id: (sorted([eve_ship1_id, eve_ship2_id]), [])}
    # Action
    api_module.change_module(mutation=eve_mutator_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_ship2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_ship1_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.items == {api_module.id: (sorted([eve_ship2_id, eve_ship3_id]), [])}
    # Action
    api_module.change_module(mutation=None)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_ship3_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_ship3_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.items == {api_module.id: (sorted([eve_ship1_id, eve_ship2_id]), [])}


def test_mutation_group(client, consts):
    # Unrealistic scenario, but we still check what happens if restrictions on base and mutated item
    # are not the same
    eve_ship_grp1_id = client.mk_eve_ship_group()
    eve_ship_grp2_id = client.mk_eve_ship_group()
    eve_ship_grp3_id = client.mk_eve_ship_group()
    eve_group1_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_group2_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group2, unit_id=consts.EveAttrUnit.group_id)
    eve_ship1_id = client.mk_eve_ship(grp_id=eve_ship_grp1_id)
    eve_ship2_id = client.mk_eve_ship(grp_id=eve_ship_grp2_id)
    eve_ship3_id = client.mk_eve_ship(grp_id=eve_ship_grp3_id)
    eve_base_module_id = client.mk_eve_item(
        attrs={eve_group1_attr_id: eve_ship_grp1_id, eve_group2_attr_id: eve_ship_grp2_id})
    eve_mutated_module_id = client.mk_eve_item(attrs={eve_group1_attr_id: eve_ship_grp3_id})
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_module_id], eve_mutated_module_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship3_id)
    api_module = api_fit.add_module(type_id=eve_base_module_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_ship3_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp3_id
    assert api_val.details.ship_limit.items == {api_module.id: ([], sorted([eve_ship_grp1_id, eve_ship_grp2_id]))}
    # Action
    api_module.change_module(mutation=eve_mutator_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_ship2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_ship1_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp1_id
    assert api_val.details.ship_limit.items == {api_module.id: ([], sorted([eve_ship_grp2_id, eve_ship_grp3_id]))}
    # Action
    api_module.change_module(mutation=None)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_ship3_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_ship3_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp3_id
    assert api_val.details.ship_limit.items == {api_module.id: ([], sorted([eve_ship_grp1_id, eve_ship_grp2_id]))}


def test_no_ship(client, consts):
    eve_ship_grp1_id = client.mk_eve_ship_group()
    eve_ship_grp2_id = client.mk_eve_ship_group()
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type1, unit_id=consts.EveAttrUnit.item_id)
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp2_id)
    eve_module_id = client.mk_eve_item(attrs={eve_type_attr_id: eve_ship_id, eve_group_attr_id: eve_ship_grp1_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id is None
    assert api_val.details.ship_limit.ship_group_id is None
    assert api_val.details.ship_limit.items == {api_module.id: ([eve_ship_id], [eve_ship_grp1_id])}


def test_not_loaded_ship(client, consts):
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type1, unit_id=consts.EveAttrUnit.item_id)
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_ship1_id = client.alloc_item_id()
    eve_ship2_id = client.alloc_item_id()
    eve_module1_id = client.mk_eve_item(attrs={eve_type_attr_id: eve_ship1_id})
    eve_module2_id = client.mk_eve_item(attrs={eve_type_attr_id: eve_ship2_id})
    eve_module3_id = client.mk_eve_item(attrs={eve_type_attr_id: eve_ship1_id, eve_group_attr_id: eve_ship_grp_id})
    eve_module4_id = client.mk_eve_item(attrs={eve_type_attr_id: eve_ship2_id, eve_group_attr_id: eve_ship_grp_id})
    eve_module5_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_ship_grp_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship2_id)
    api_module1 = api_fit.add_module(type_id=eve_module1_id)
    api_fit.add_module(type_id=eve_module2_id)
    api_module3 = api_fit.add_module(type_id=eve_module3_id)
    api_fit.add_module(type_id=eve_module4_id)
    api_module5 = api_fit.add_module(type_id=eve_module5_id)
    # Verification - when ship is not loaded, we fail validation only we 100% know it will fail with
    # the info we have.
    # - Module 1 fails because item type is mismatched
    # - Module 2 passes because item type is matched
    # - Module 3 fails because type is mismatched, and ship group is not available
    # - Module 4 passes because item type is matched, even if ship group is not available
    # - Module 5 fails because ship group is not available
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_ship2_id
    assert api_val.details.ship_limit.ship_group_id is None
    assert api_val.details.ship_limit.items == {
        api_module1.id: ([eve_ship1_id], []),
        api_module3.id: ([eve_ship1_id], [eve_ship_grp_id]),
        api_module5.id: ([], [eve_ship_grp_id])}
    # Action
    api_module1.remove()
    api_module3.remove()
    api_module5.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_criterion_item_state(client, consts):
    # Restriction applies even to disabled mods and disabled rigs
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type1, unit_id=consts.EveAttrUnit.item_id)
    eve_allowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_module_id = client.mk_eve_item(attrs={eve_type_attr_id: eve_allowed_ship_id})
    eve_rig_id = client.mk_eve_item(attrs={eve_type_attr_id: eve_allowed_ship_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.disabled)
    api_rig = api_fit.add_rig(type_id=eve_rig_id, state=False)
    api_subsystem = api_fit.add_subsystem(type_id=eve_rig_id, state=False)
    # Verification
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_disallowed_ship_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.items == {
        api_module.id: ([eve_allowed_ship_id], []),
        api_rig.id: ([eve_allowed_ship_id], []),
        api_subsystem.id: ([eve_allowed_ship_id], [])}


def test_criterion_item_kind(client, consts):
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type1, unit_id=consts.EveAttrUnit.item_id)
    eve_ship_id = client.mk_eve_ship()
    eve_item_id = client.mk_eve_ship(attrs={eve_type_attr_id: eve_ship_id})
    eve_module_id = client.mk_eve_ship()
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_fighter_id = client.mk_eve_item(
        attrs={eve_autocharge_attr_id: eve_item_id, eve_type_attr_id: eve_ship_id},
        eff_ids=[eve_autocharge_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_booster(type_id=eve_item_id)
    api_fit.set_character(type_id=eve_item_id)
    api_fit.add_drone(type_id=eve_item_id, state=consts.ApiMinionState.engaging)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fit.add_fw_effect(type_id=eve_item_id)
    api_fit.add_implant(type_id=eve_item_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.overload, charge_type_id=eve_item_id)
    api_fit.set_ship(type_id=eve_item_id)
    api_fit.add_skill(type_id=eve_item_id, level=5)
    # Verification
    assert len(api_fighter.autocharges) == 1
    api_val = api_fit.validate(options=ValOptions(ship_limit=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
