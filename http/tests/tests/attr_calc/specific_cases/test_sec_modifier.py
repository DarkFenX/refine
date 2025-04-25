"""
Basing general security modifier attribute value off final value of security zone-specific
attributes is used by structure rigs. Structure rigs use it in fairly straight-forward case -
nothing modifies security-specific attribute, nothing else seems to modify general security
attribute. However, tests cover more complex cases, since exact details on how it works were guessed
by looking at code of decompiled EVE client.
"""

from tests import approx


def test_sec_zones(client, consts):
    # Realistic case, pretty much an engineering ME rig, except for general attr modifier value
    # being 1.1 instead of 1 and hisec modifier value being 1.2 instead of 1
    eve_sec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.security_modifier)
    eve_hisec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hisec_modifier)
    eve_lowsec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.lowsec_modifier)
    eve_nullsec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.nullsec_modifier)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_sec_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(
        attrs={
            eve_sec_attr_id: 1.1,
            eve_hisec_attr_id: 1.2,
            eve_lowsec_attr_id: 1.9,
            eve_nullsec_attr_id: 2.1,
            eve_affectee_attr_id: -2.4},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_rig.update()
    assert api_rig.attrs[eve_sec_attr_id].dogma == approx(1.2)
    assert api_rig.attrs[eve_affectee_attr_id].dogma == approx(-2.88)
    api_sec_mod = api_rig.mods[eve_sec_attr_id].one()
    assert api_sec_mod.op == consts.ApiModOp.base_assign
    assert api_sec_mod.initial_val == approx(1.2)
    assert api_sec_mod.stacking_mult is None
    assert api_sec_mod.applied_val == approx(1.2)
    assert api_sec_mod.affectors.one().item_id == api_rig.id
    assert api_sec_mod.affectors.one().attr_id == eve_hisec_attr_id
    api_affectee_mod = api_rig.mods[eve_affectee_attr_id].one()
    assert api_affectee_mod.op == consts.ApiModOp.post_mul
    assert api_affectee_mod.initial_val == approx(1.2)
    assert api_affectee_mod.stacking_mult is None
    assert api_affectee_mod.applied_val == approx(1.2)
    assert api_affectee_mod.affectors.one().item_id == api_rig.id
    assert api_affectee_mod.affectors.one().attr_id == eve_sec_attr_id
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.hisec_c5)
    # Verification
    api_rig.update()
    assert api_rig.attrs[eve_sec_attr_id].dogma == approx(1.2)
    assert api_rig.attrs[eve_affectee_attr_id].dogma == approx(-2.88)
    api_sec_mod = api_rig.mods[eve_sec_attr_id].one()
    assert api_sec_mod.op == consts.ApiModOp.base_assign
    assert api_sec_mod.initial_val == approx(1.2)
    assert api_sec_mod.stacking_mult is None
    assert api_sec_mod.applied_val == approx(1.2)
    assert api_sec_mod.affectors.one().item_id == api_rig.id
    assert api_sec_mod.affectors.one().attr_id == eve_hisec_attr_id
    api_affectee_mod = api_rig.mods[eve_affectee_attr_id].one()
    assert api_affectee_mod.op == consts.ApiModOp.post_mul
    assert api_affectee_mod.initial_val == approx(1.2)
    assert api_affectee_mod.stacking_mult is None
    assert api_affectee_mod.applied_val == approx(1.2)
    assert api_affectee_mod.affectors.one().item_id == api_rig.id
    assert api_affectee_mod.affectors.one().attr_id == eve_sec_attr_id
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.lowsec)
    # Verification
    api_rig.update()
    assert api_rig.attrs[eve_sec_attr_id].dogma == approx(1.9)
    assert api_rig.attrs[eve_affectee_attr_id].dogma == approx(-4.56)
    api_sec_mod = api_rig.mods[eve_sec_attr_id].one()
    assert api_sec_mod.op == consts.ApiModOp.base_assign
    assert api_sec_mod.initial_val == approx(1.9)
    assert api_sec_mod.stacking_mult is None
    assert api_sec_mod.applied_val == approx(1.9)
    assert api_sec_mod.affectors.one().item_id == api_rig.id
    assert api_sec_mod.affectors.one().attr_id == eve_lowsec_attr_id
    api_affectee_mod = api_rig.mods[eve_affectee_attr_id].one()
    assert api_affectee_mod.op == consts.ApiModOp.post_mul
    assert api_affectee_mod.initial_val == approx(1.9)
    assert api_affectee_mod.stacking_mult is None
    assert api_affectee_mod.applied_val == approx(1.9)
    assert api_affectee_mod.affectors.one().item_id == api_rig.id
    assert api_affectee_mod.affectors.one().attr_id == eve_sec_attr_id
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.lowsec_c5)
    # Verification
    api_rig.update()
    assert api_rig.attrs[eve_sec_attr_id].dogma == approx(1.9)
    assert api_rig.attrs[eve_affectee_attr_id].dogma == approx(-4.56)
    api_sec_mod = api_rig.mods[eve_sec_attr_id].one()
    assert api_sec_mod.op == consts.ApiModOp.base_assign
    assert api_sec_mod.initial_val == approx(1.9)
    assert api_sec_mod.stacking_mult is None
    assert api_sec_mod.applied_val == approx(1.9)
    assert api_sec_mod.affectors.one().item_id == api_rig.id
    assert api_sec_mod.affectors.one().attr_id == eve_lowsec_attr_id
    api_affectee_mod = api_rig.mods[eve_affectee_attr_id].one()
    assert api_affectee_mod.op == consts.ApiModOp.post_mul
    assert api_affectee_mod.initial_val == approx(1.9)
    assert api_affectee_mod.stacking_mult is None
    assert api_affectee_mod.applied_val == approx(1.9)
    assert api_affectee_mod.affectors.one().item_id == api_rig.id
    assert api_affectee_mod.affectors.one().attr_id == eve_sec_attr_id
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.nullsec)
    # Verification
    api_rig.update()
    assert api_rig.attrs[eve_sec_attr_id].dogma == approx(2.1)
    assert api_rig.attrs[eve_affectee_attr_id].dogma == approx(-5.04)
    api_sec_mod = api_rig.mods[eve_sec_attr_id].one()
    assert api_sec_mod.op == consts.ApiModOp.base_assign
    assert api_sec_mod.initial_val == approx(2.1)
    assert api_sec_mod.stacking_mult is None
    assert api_sec_mod.applied_val == approx(2.1)
    assert api_sec_mod.affectors.one().item_id == api_rig.id
    assert api_sec_mod.affectors.one().attr_id == eve_nullsec_attr_id
    api_affectee_mod = api_rig.mods[eve_affectee_attr_id].one()
    assert api_affectee_mod.op == consts.ApiModOp.post_mul
    assert api_affectee_mod.initial_val == approx(2.1)
    assert api_affectee_mod.stacking_mult is None
    assert api_affectee_mod.applied_val == approx(2.1)
    assert api_affectee_mod.affectors.one().item_id == api_rig.id
    assert api_affectee_mod.affectors.one().attr_id == eve_sec_attr_id
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.wspace)
    # Verification
    api_rig.update()
    assert api_rig.attrs[eve_sec_attr_id].dogma == approx(2.1)
    assert api_rig.attrs[eve_affectee_attr_id].dogma == approx(-5.04)
    api_sec_mod = api_rig.mods[eve_sec_attr_id].one()
    assert api_sec_mod.op == consts.ApiModOp.base_assign
    assert api_sec_mod.initial_val == approx(2.1)
    assert api_sec_mod.stacking_mult is None
    assert api_sec_mod.applied_val == approx(2.1)
    assert api_sec_mod.affectors.one().item_id == api_rig.id
    assert api_sec_mod.affectors.one().attr_id == eve_nullsec_attr_id
    api_affectee_mod = api_rig.mods[eve_affectee_attr_id].one()
    assert api_affectee_mod.op == consts.ApiModOp.post_mul
    assert api_affectee_mod.initial_val == approx(2.1)
    assert api_affectee_mod.stacking_mult is None
    assert api_affectee_mod.applied_val == approx(2.1)
    assert api_affectee_mod.affectors.one().item_id == api_rig.id
    assert api_affectee_mod.affectors.one().attr_id == eve_sec_attr_id
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.hazard)
    # Verification
    api_rig.update()
    assert api_rig.attrs[eve_sec_attr_id].dogma == approx(2.1)
    assert api_rig.attrs[eve_affectee_attr_id].dogma == approx(-5.04)
    api_sec_mod = api_rig.mods[eve_sec_attr_id].one()
    assert api_sec_mod.op == consts.ApiModOp.base_assign
    assert api_sec_mod.initial_val == approx(2.1)
    assert api_sec_mod.stacking_mult is None
    assert api_sec_mod.applied_val == approx(2.1)
    assert api_sec_mod.affectors.one().item_id == api_rig.id
    assert api_sec_mod.affectors.one().attr_id == eve_nullsec_attr_id
    api_affectee_mod = api_rig.mods[eve_affectee_attr_id].one()
    assert api_affectee_mod.op == consts.ApiModOp.post_mul
    assert api_affectee_mod.initial_val == approx(2.1)
    assert api_affectee_mod.stacking_mult is None
    assert api_affectee_mod.applied_val == approx(2.1)
    assert api_affectee_mod.affectors.one().item_id == api_rig.id
    assert api_affectee_mod.affectors.one().attr_id == eve_sec_attr_id


def test_propagation_general(client, consts):
    # Make sure that values taken from zone-specific attributes merely serve as base value, and do
    # not overwrite general attribute value
    eve_sec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.security_modifier)
    eve_hisec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hisec_modifier)
    eve_lowsec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.lowsec_modifier)
    eve_nullsec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.nullsec_modifier)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_sec_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_rig_effect_id = client.mk_eve_effect(mod_info=[eve_rig_mod])
    eve_rig_id = client.mk_eve_item(
        attrs={
            eve_sec_attr_id: 1.1,
            eve_hisec_attr_id: 1.2,
            eve_lowsec_attr_id: 1.9,
            eve_nullsec_attr_id: 2.1,
            eve_affectee_attr_id: -2.4},
        eff_ids=[eve_rig_effect_id])
    eve_affector_attr_id = client.mk_eve_attr()
    eve_implant_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_sec_attr_id)
    eve_implant_effect_id = client.mk_eve_effect(mod_info=[eve_implant_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_affector_attr_id: 30}, eff_ids=[eve_implant_effect_id])
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.lowsec)
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_rig.update()
    assert api_rig.attrs[eve_hisec_attr_id].dogma == approx(1.2)
    assert api_rig.attrs[eve_lowsec_attr_id].dogma == approx(1.9)
    assert api_rig.attrs[eve_nullsec_attr_id].dogma == approx(2.1)
    assert api_rig.attrs[eve_sec_attr_id].dogma == approx(1.9)
    assert api_rig.attrs[eve_affectee_attr_id].dogma == approx(-4.56)
    assert len(api_rig.mods) == 2
    assert len(api_rig.mods[eve_sec_attr_id]) == 1
    assert len(api_rig.mods[eve_affectee_attr_id]) == 1
    # Action
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_rig.update()
    assert api_rig.attrs[eve_hisec_attr_id].dogma == approx(1.2)
    assert api_rig.attrs[eve_lowsec_attr_id].dogma == approx(1.9)
    assert api_rig.attrs[eve_nullsec_attr_id].dogma == approx(2.1)
    assert api_rig.attrs[eve_sec_attr_id].dogma == approx(2.47)
    assert api_rig.attrs[eve_affectee_attr_id].dogma == approx(-5.928)
    assert len(api_rig.mods) == 2
    assert len(api_rig.mods[eve_sec_attr_id]) == 2
    assert len(api_rig.mods[eve_affectee_attr_id]) == 1
    # Action
    api_implant.remove()
    # Verification
    api_rig.update()
    assert api_rig.attrs[eve_hisec_attr_id].dogma == approx(1.2)
    assert api_rig.attrs[eve_lowsec_attr_id].dogma == approx(1.9)
    assert api_rig.attrs[eve_nullsec_attr_id].dogma == approx(2.1)
    assert api_rig.attrs[eve_sec_attr_id].dogma == approx(1.9)
    assert api_rig.attrs[eve_affectee_attr_id].dogma == approx(-4.56)
    assert len(api_rig.mods) == 2
    assert len(api_rig.mods[eve_sec_attr_id]) == 1
    assert len(api_rig.mods[eve_affectee_attr_id]) == 1


def test_propagation_specific(client, consts):
    # Check that changes to zone-specific attributes trigger recalculation of general security
    # attribute and its dependants.
    eve_sec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.security_modifier)
    eve_hisec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hisec_modifier)
    eve_lowsec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.lowsec_modifier)
    eve_nullsec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.nullsec_modifier)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_sec_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_rig_effect_id = client.mk_eve_effect(mod_info=[eve_rig_mod])
    eve_rig_id = client.mk_eve_item(
        attrs={
            eve_sec_attr_id: 1.1,
            eve_hisec_attr_id: 1.2,
            eve_lowsec_attr_id: 1.9,
            eve_nullsec_attr_id: 2.1,
            eve_affectee_attr_id: -2.4},
        eff_ids=[eve_rig_effect_id])
    eve_affector_attr_id = client.mk_eve_attr()
    eve_hisec_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_hisec_attr_id)
    eve_hisec_effect_id = client.mk_eve_effect(mod_info=[eve_hisec_mod])
    eve_hisec_implant_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_hisec_effect_id])
    eve_lowsec_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_lowsec_attr_id)
    eve_lowsec_effect_id = client.mk_eve_effect(mod_info=[eve_lowsec_mod])
    eve_lowsec_implant_id = client.mk_eve_item(attrs={eve_affector_attr_id: 30}, eff_ids=[eve_lowsec_effect_id])
    eve_nullsec_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_nullsec_attr_id)
    eve_nullsec_effect_id = client.mk_eve_effect(mod_info=[eve_nullsec_mod])
    eve_nullsec_implant_id = client.mk_eve_item(attrs={eve_affector_attr_id: 40}, eff_ids=[eve_nullsec_effect_id])
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_rig.update()
    assert api_rig.attrs[eve_hisec_attr_id].dogma == approx(1.2)
    assert api_rig.attrs[eve_lowsec_attr_id].dogma == approx(1.9)
    assert api_rig.attrs[eve_nullsec_attr_id].dogma == approx(2.1)
    assert api_rig.attrs[eve_sec_attr_id].dogma == approx(1.2)
    assert api_rig.attrs[eve_affectee_attr_id].dogma == approx(-2.88)
    assert len(api_rig.mods) == 2
    assert len(api_rig.mods[eve_sec_attr_id]) == 1
    assert len(api_rig.mods[eve_affectee_attr_id]) == 1
    # Action
    api_hisec_implant = api_fit.add_implant(type_id=eve_hisec_implant_id)
    # Verification
    api_rig.update()
    assert api_rig.attrs[eve_hisec_attr_id].dogma == approx(1.44)
    assert api_rig.attrs[eve_lowsec_attr_id].dogma == approx(1.9)
    assert api_rig.attrs[eve_nullsec_attr_id].dogma == approx(2.1)
    assert api_rig.attrs[eve_sec_attr_id].dogma == approx(1.44)
    assert api_rig.attrs[eve_affectee_attr_id].dogma == approx(-3.456)
    assert len(api_rig.mods) == 3
    assert len(api_rig.mods[eve_hisec_attr_id]) == 1
    assert len(api_rig.mods[eve_sec_attr_id]) == 1
    assert len(api_rig.mods[eve_affectee_attr_id]) == 1
    # Action
    api_hisec_implant.remove()
    # Verification
    api_rig.update()
    assert api_rig.attrs[eve_hisec_attr_id].dogma == approx(1.2)
    assert api_rig.attrs[eve_lowsec_attr_id].dogma == approx(1.9)
    assert api_rig.attrs[eve_nullsec_attr_id].dogma == approx(2.1)
    assert api_rig.attrs[eve_sec_attr_id].dogma == approx(1.2)
    assert api_rig.attrs[eve_affectee_attr_id].dogma == approx(-2.88)
    assert len(api_rig.mods) == 2
    assert len(api_rig.mods[eve_sec_attr_id]) == 1
    assert len(api_rig.mods[eve_affectee_attr_id]) == 1
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.lowsec)
    # Verification
    api_rig.update()
    assert api_rig.attrs[eve_hisec_attr_id].dogma == approx(1.2)
    assert api_rig.attrs[eve_lowsec_attr_id].dogma == approx(1.9)
    assert api_rig.attrs[eve_nullsec_attr_id].dogma == approx(2.1)
    assert api_rig.attrs[eve_sec_attr_id].dogma == approx(1.9)
    assert api_rig.attrs[eve_affectee_attr_id].dogma == approx(-4.56)
    assert len(api_rig.mods) == 2
    assert len(api_rig.mods[eve_sec_attr_id]) == 1
    assert len(api_rig.mods[eve_affectee_attr_id]) == 1
    # Action
    api_lowsec_implant = api_fit.add_implant(type_id=eve_lowsec_implant_id)
    # Verification
    api_rig.update()
    assert api_rig.attrs[eve_hisec_attr_id].dogma == approx(1.2)
    assert api_rig.attrs[eve_lowsec_attr_id].dogma == approx(2.47)
    assert api_rig.attrs[eve_nullsec_attr_id].dogma == approx(2.1)
    assert api_rig.attrs[eve_sec_attr_id].dogma == approx(2.47)
    assert api_rig.attrs[eve_affectee_attr_id].dogma == approx(-5.928)
    assert len(api_rig.mods) == 3
    assert len(api_rig.mods[eve_lowsec_attr_id]) == 1
    assert len(api_rig.mods[eve_sec_attr_id]) == 1
    assert len(api_rig.mods[eve_affectee_attr_id]) == 1
    # Action
    api_lowsec_implant.remove()
    # Verification
    api_rig.update()
    assert api_rig.attrs[eve_hisec_attr_id].dogma == approx(1.2)
    assert api_rig.attrs[eve_lowsec_attr_id].dogma == approx(1.9)
    assert api_rig.attrs[eve_nullsec_attr_id].dogma == approx(2.1)
    assert api_rig.attrs[eve_sec_attr_id].dogma == approx(1.9)
    assert api_rig.attrs[eve_affectee_attr_id].dogma == approx(-4.56)
    assert len(api_rig.mods) == 2
    assert len(api_rig.mods[eve_sec_attr_id]) == 1
    assert len(api_rig.mods[eve_affectee_attr_id]) == 1
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.nullsec)
    # Verification
    api_rig.update()
    assert api_rig.attrs[eve_hisec_attr_id].dogma == approx(1.2)
    assert api_rig.attrs[eve_lowsec_attr_id].dogma == approx(1.9)
    assert api_rig.attrs[eve_nullsec_attr_id].dogma == approx(2.1)
    assert api_rig.attrs[eve_sec_attr_id].dogma == approx(2.1)
    assert api_rig.attrs[eve_affectee_attr_id].dogma == approx(-5.04)
    assert len(api_rig.mods) == 2
    assert len(api_rig.mods[eve_sec_attr_id]) == 1
    assert len(api_rig.mods[eve_affectee_attr_id]) == 1
    # Action
    api_nullsec_implant = api_fit.add_implant(type_id=eve_nullsec_implant_id)
    # Verification
    api_rig.update()
    assert api_rig.attrs[eve_hisec_attr_id].dogma == approx(1.2)
    assert api_rig.attrs[eve_lowsec_attr_id].dogma == approx(1.9)
    assert api_rig.attrs[eve_nullsec_attr_id].dogma == approx(2.94)
    assert api_rig.attrs[eve_sec_attr_id].dogma == approx(2.94)
    assert api_rig.attrs[eve_affectee_attr_id].dogma == approx(-7.056)
    assert len(api_rig.mods) == 3
    assert len(api_rig.mods[eve_nullsec_attr_id]) == 1
    assert len(api_rig.mods[eve_sec_attr_id]) == 1
    assert len(api_rig.mods[eve_affectee_attr_id]) == 1
    # Action
    api_nullsec_implant.remove()
    # Verification
    api_rig.update()
    assert api_rig.attrs[eve_hisec_attr_id].dogma == approx(1.2)
    assert api_rig.attrs[eve_lowsec_attr_id].dogma == approx(1.9)
    assert api_rig.attrs[eve_nullsec_attr_id].dogma == approx(2.1)
    assert api_rig.attrs[eve_sec_attr_id].dogma == approx(2.1)
    assert api_rig.attrs[eve_affectee_attr_id].dogma == approx(-5.04)
    assert len(api_rig.mods) == 2
    assert len(api_rig.mods[eve_sec_attr_id]) == 1
    assert len(api_rig.mods[eve_affectee_attr_id]) == 1


def test_no_value_general(client, consts):
    eve_sec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.security_modifier)
    eve_hisec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hisec_modifier)
    eve_lowsec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.lowsec_modifier)
    eve_nullsec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.nullsec_modifier)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_sec_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(
        attrs={
            eve_hisec_attr_id: 1.2,
            eve_lowsec_attr_id: 1.9,
            eve_nullsec_attr_id: 2.1,
            eve_affectee_attr_id: -2.4},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.nullsec)
    api_fit = api_sol.create_fit()
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification - first update triggers calculation of affectee attribute (and thus general sec
    # attribute value), but does not include value of general sec attribute, second update includes
    # it
    api_rig.update().update()
    assert api_rig.attrs[eve_sec_attr_id].dogma == approx(2.1)
    assert api_rig.attrs[eve_affectee_attr_id].dogma == approx(-5.04)


def test_no_value_specific(client, consts):
    eve_sec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.security_modifier)
    eve_hisec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hisec_modifier)
    eve_lowsec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.lowsec_modifier)
    client.mk_eve_attr(id_=consts.EveAttr.nullsec_modifier, def_val=1.5)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_sec_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(
        attrs={
            eve_sec_attr_id: 1.1,
            eve_hisec_attr_id: 1.2,
            eve_lowsec_attr_id: 1.9,
            eve_affectee_attr_id: -2.4},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.nullsec)
    api_fit = api_sol.create_fit()
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification - first update triggers calculation of affectee attribute (and thus nullsec
    # attribute value), but does not include value of nullsec attribute, second update includes it
    api_rig.update().update()
    assert api_rig.attrs[eve_sec_attr_id].dogma == approx(1.5)
    assert api_rig.attrs[eve_affectee_attr_id].dogma == approx(-3.6)


def setup_op_precedence_test(*, client, consts, high_is_good):
    eve_sec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.security_modifier, high_is_good=high_is_good)
    eve_hisec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hisec_modifier)
    eve_lowsec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.lowsec_modifier)
    eve_nullsec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.nullsec_modifier)
    eve_rig_id = client.mk_eve_item(
        attrs={eve_sec_attr_id: 1.1, eve_hisec_attr_id: 1.2, eve_lowsec_attr_id: 1.9, eve_nullsec_attr_id: 2.1})
    eve_affector_attr_id = client.mk_eve_attr()
    eve_implant_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.pre_assign,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_sec_attr_id)
    eve_implant_effect_id = client.mk_eve_effect(mod_info=[eve_implant_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_affector_attr_id: 1.5}, eff_ids=[eve_implant_effect_id])
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.lowsec)
    api_fit = api_sol.create_fit()
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    api_rig.update()
    return (
        eve_hisec_attr_id,
        eve_lowsec_attr_id,
        eve_nullsec_attr_id,
        eve_sec_attr_id,
        eve_affector_attr_id,
        api_rig,
        api_implant)


def test_op_precedence_high_is_good(client, consts):
    (eve_hisec_attr_id,
     eve_lowsec_attr_id,
     eve_nullsec_attr_id,
     eve_sec_attr_id,
     eve_affector_attr_id,
     api_rig,
     api_implant) = setup_op_precedence_test(client=client, consts=consts, high_is_good=True)
    # Verification - base-assignment goes before pre-assignment, regardless of attribute's
    # high-is-good flag, thus gets discarded in modification info as ineffective
    assert api_rig.attrs[eve_hisec_attr_id].dogma == approx(1.2)
    assert api_rig.attrs[eve_lowsec_attr_id].dogma == approx(1.9)
    assert api_rig.attrs[eve_nullsec_attr_id].dogma == approx(2.1)
    assert api_rig.attrs[eve_sec_attr_id].dogma == approx(1.5)
    api_mod = api_rig.mods[eve_sec_attr_id].one()
    assert api_mod.op == consts.ApiModOp.pre_assign
    assert api_mod.initial_val == approx(1.5)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(1.5)
    assert api_mod.affectors.one().item_id == api_implant.id
    assert api_mod.affectors.one().attr_id == eve_affector_attr_id


def test_op_precedence_high_is_bad(client, consts):
    (eve_hisec_attr_id,
     eve_lowsec_attr_id,
     eve_nullsec_attr_id,
     eve_sec_attr_id,
     eve_affector_attr_id,
     api_rig,
     api_implant) = setup_op_precedence_test(client=client, consts=consts, high_is_good=False)
    # Verification - base-assignment goes before pre-assignment, regardless of attribute's
    # high-is-good flag, thus gets discarded in modification info as ineffective
    assert api_rig.attrs[eve_hisec_attr_id].dogma == approx(1.2)
    assert api_rig.attrs[eve_lowsec_attr_id].dogma == approx(1.9)
    assert api_rig.attrs[eve_nullsec_attr_id].dogma == approx(2.1)
    assert api_rig.attrs[eve_sec_attr_id].dogma == approx(1.5)
    api_mod = api_rig.mods[eve_sec_attr_id].one()
    assert api_mod.op == consts.ApiModOp.pre_assign
    assert api_mod.initial_val == approx(1.5)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(1.5)
    assert api_mod.affectors.one().item_id == api_implant.id
    assert api_mod.affectors.one().attr_id == eve_affector_attr_id
