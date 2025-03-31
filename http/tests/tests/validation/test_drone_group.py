"""
Drone group restriction isn't used in EVE as of 2025-03-05. It used to be a thing briefly for
motherships/supercarriers when they were transitioned into fighter drones, before CCP implemented
fighters as we know them today. Nevertheless, the lib supports this validation, just in case it ever
gets back.
"""


from tests import approx, check_no_field
from tests.fw.api import ValOptions


def test_drone_add_remove(client, consts):
    # Also check matching against multiple allowed groups
    eve_group1_id = client.mk_eve_item_group()
    eve_group2_id = client.mk_eve_item_group()
    eve_group3_id = client.mk_eve_item_group()
    eve_limit_attr1_id = client.mk_eve_attr(
        id_=consts.EveAttr.allowed_drone_group1,
        unit_id=consts.EveAttrUnit.group_id)
    eve_limit_attr2_id = client.mk_eve_attr(
        id_=consts.EveAttr.allowed_drone_group2,
        unit_id=consts.EveAttrUnit.group_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_limit_attr1_id: eve_group1_id, eve_limit_attr2_id: eve_group2_id})
    eve_drone_id = client.mk_eve_item(grp_id=eve_group3_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_group=True))
    assert api_val.passed is False
    assert api_val.details.drone_group == (sorted([eve_group1_id, eve_group2_id]), {api_drone.id: eve_group3_id})
    # Action
    api_drone.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_ship_add_set_remove(client, consts):
    eve_group1_id = client.mk_eve_item_group()
    eve_group2_id = client.mk_eve_item_group()
    eve_group3_id = client.mk_eve_item_group()
    eve_limit_attr1_id = client.mk_eve_attr(
        id_=consts.EveAttr.allowed_drone_group1,
        unit_id=consts.EveAttrUnit.group_id)
    eve_limit_attr2_id = client.mk_eve_attr(
        id_=consts.EveAttr.allowed_drone_group2,
        unit_id=consts.EveAttrUnit.group_id)
    eve_ship1_id = client.mk_eve_ship(attrs={eve_limit_attr1_id: eve_group1_id})
    eve_ship2_id = client.mk_eve_ship()
    eve_ship3_id = client.mk_eve_ship(attrs={eve_limit_attr2_id: eve_group2_id})
    eve_drone_id = client.mk_eve_item(grp_id=eve_group3_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_group=True))
    assert api_val.passed is False
    assert api_val.details.drone_group == ([eve_group1_id], {api_drone.id: eve_group3_id})
    # Action
    api_fit.set_ship(type_id=eve_ship2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_ship3_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_group=True))
    assert api_val.passed is False
    assert api_val.details.drone_group == ([eve_group2_id], {api_drone.id: eve_group3_id})
    # Action
    api_fit.remove_ship()
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_drone_add_remove_non_limited_ship(client, consts):
    eve_group1_id = client.mk_eve_item_group()
    eve_group2_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.allowed_drone_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_ship1_id = client.mk_eve_ship()
    eve_ship2_id = client.mk_eve_ship(attrs={eve_limit_attr_id: eve_group1_id})
    eve_drone_id = client.mk_eve_item(grp_id=eve_group2_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_ship2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_group=True))
    assert api_val.passed is False
    assert api_val.details.drone_group == ([eve_group1_id], {api_drone.id: eve_group2_id})
    # Action
    api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_drone.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_ship2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_rounding(client, consts):
    # Also check that only unique groups are returned
    eve_group1_id = client.mk_eve_item_group()
    eve_group2_id = client.mk_eve_item_group()
    eve_limit_attr1_id = client.mk_eve_attr(
        id_=consts.EveAttr.allowed_drone_group1,
        unit_id=consts.EveAttrUnit.group_id)
    eve_limit_attr2_id = client.mk_eve_attr(
        id_=consts.EveAttr.allowed_drone_group2,
        unit_id=consts.EveAttrUnit.group_id)
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_limit_attr1_id: eve_group1_id - 0.4, eve_limit_attr2_id: eve_group1_id + 0.4})
    eve_drone1_id = client.mk_eve_item(grp_id=eve_group1_id)
    eve_drone2_id = client.mk_eve_item(grp_id=eve_group2_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone1_id)
    api_drone2 = api_fit.add_drone(type_id=eve_drone2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_group=True))
    assert api_val.passed is False
    assert api_val.details.drone_group == ([eve_group1_id], {api_drone2.id: eve_group2_id})


def test_known_failures(client, consts):
    eve_group1_id = client.mk_eve_item_group()
    eve_group2_id = client.mk_eve_item_group()
    eve_group3_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.allowed_drone_group1,
        unit_id=consts.EveAttrUnit.group_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_limit_attr_id: eve_group1_id})
    eve_drone1_id = client.mk_eve_item(grp_id=eve_group1_id)
    eve_drone2_id = client.mk_eve_item(grp_id=eve_group2_id)
    eve_drone3_id = client.mk_eve_item(grp_id=eve_group3_id)
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_implant(type_id=eve_other_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.in_bay)
    api_drone2 = api_fit.add_drone(type_id=eve_drone2_id, state=consts.ApiMinionState.in_bay)
    api_drone3 = api_fit.add_drone(type_id=eve_drone3_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_group=(True, [api_drone2.id])))
    assert api_val.passed is False
    assert api_val.details.drone_group == ([eve_group1_id], {api_drone3.id: eve_group3_id})
    api_val = api_fit.validate(options=ValOptions(drone_group=(True, [api_drone3.id])))
    assert api_val.passed is False
    assert api_val.details.drone_group == ([eve_group1_id], {api_drone2.id: eve_group2_id})
    api_val = api_fit.validate(options=ValOptions(drone_group=(True, [api_drone2.id, api_drone3.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(drone_group=(True, [api_drone2.id, api_other.id, api_drone3.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_no_attr(client, consts):
    eve_group1_id = client.mk_eve_item_group()
    eve_group2_id = client.mk_eve_item_group()
    eve_limit_attr1_id = consts.EveAttr.allowed_drone_group1
    eve_ship_id = client.mk_eve_ship(attrs={eve_limit_attr1_id: eve_group1_id})
    eve_drone_id = client.mk_eve_item(grp_id=eve_group2_id)
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(grp_id=eve_group1_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_group=True))
    assert api_val.passed is False
    assert api_val.details.drone_group == ([eve_group1_id], {api_drone.id: eve_group2_id})


def test_modified_limit(client, consts):
    eve_group1_id = client.mk_eve_item_group()
    eve_group2_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.allowed_drone_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_limit_attr_id: eve_group1_id})
    eve_drone_id = client.mk_eve_item(grp_id=eve_group2_id)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_limit_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: eve_group2_id}, eff_ids=[eve_mod_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    assert api_ship.update().attrs[eve_limit_attr_id].extra == approx(eve_group1_id)
    api_val = api_fit.validate(options=ValOptions(drone_group=True))
    assert api_val.passed is False
    assert api_val.details.drone_group == ([eve_group1_id], {api_drone.id: eve_group2_id})
    # Action
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification - attribute is modified, but not for purposes of validation
    assert api_ship.update().attrs[eve_limit_attr_id].extra == approx(eve_group2_id)
    api_val = api_fit.validate(options=ValOptions(drone_group=True))
    assert api_val.passed is False
    assert api_val.details.drone_group == ([eve_group1_id], {api_drone.id: eve_group2_id})


def test_mutation_drone_group(client, consts):
    eve_group1_id = client.mk_eve_item_group()
    eve_group2_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.allowed_drone_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_ship1_id = client.mk_eve_ship(attrs={eve_limit_attr_id: eve_group1_id})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_limit_attr_id: eve_group2_id})
    eve_base_drone_id = client.mk_eve_item(grp_id=eve_group2_id)
    eve_mutated_drone_id = client.mk_eve_item(grp_id=eve_group1_id)
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_drone_id], eve_mutated_drone_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship1_id)
    api_drone = api_fit.add_drone(type_id=eve_base_drone_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_group=True))
    assert api_val.passed is False
    assert api_val.details.drone_group == ([eve_group1_id], {api_drone.id: eve_group2_id})
    # Action
    api_fit.set_ship(type_id=eve_ship2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_drone.change_drone(mutation=eve_mutator_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_group=True))
    assert api_val.passed is False
    assert api_val.details.drone_group == ([eve_group2_id], {api_drone.id: eve_group1_id})
    # Action
    api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_drone.change_drone(mutation=None)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_group=True))
    assert api_val.passed is False
    assert api_val.details.drone_group == ([eve_group1_id], {api_drone.id: eve_group2_id})


def test_not_loaded_ship(client):
    eve_ship_id = client.alloc_item_id()
    eve_drone_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_not_loaded_drone(client, consts):
    eve_group_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.allowed_drone_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_ship_id = client.mk_eve_item(attrs={eve_limit_attr_id: eve_group_id})
    eve_drone_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_criterion_item_kind(client, consts):
    eve_group1_id = client.mk_eve_item_group()
    eve_group2_id = client.mk_eve_item_group()
    eve_group3_id = client.mk_eve_ship_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.allowed_drone_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_ship_id = client.mk_eve_ship(grp_id=eve_group3_id, attrs={eve_limit_attr_id: eve_group1_id})
    eve_item_id = client.mk_eve_item(grp_id=eve_group2_id)
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_fighter_id = client.mk_eve_item(
        grp_id=eve_group2_id,
        attrs={eve_autocharge_attr_id: eve_item_id},
        eff_ids=[eve_autocharge_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_booster(type_id=eve_item_id)
    api_fit.set_character(type_id=eve_item_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fit.add_fw_effect(type_id=eve_item_id)
    api_fit.add_implant(type_id=eve_item_id)
    api_fit.add_module(type_id=eve_item_id, state=consts.ApiModuleState.overload, charge_type_id=eve_item_id)
    api_fit.add_rig(type_id=eve_item_id)
    api_fit.add_service(type_id=eve_item_id, state=consts.ApiServiceState.online)
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_skill(type_id=eve_item_id, level=5)
    api_fit.set_stance(type_id=eve_item_id)
    api_fit.add_subsystem(type_id=eve_item_id)
    # Verification
    assert len(api_fighter.autocharges) == 1
    api_val = api_fit.validate(options=ValOptions(drone_group=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
