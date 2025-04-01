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
    assert api_sec_mod.op == consts.ApiModOp.pre_assign
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
    assert api_sec_mod.op == consts.ApiModOp.pre_assign
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
    assert api_sec_mod.op == consts.ApiModOp.pre_assign
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
    assert api_sec_mod.op == consts.ApiModOp.pre_assign
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
    assert api_sec_mod.op == consts.ApiModOp.pre_assign
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
    assert api_sec_mod.op == consts.ApiModOp.pre_assign
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
    assert api_sec_mod.op == consts.ApiModOp.pre_assign
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
