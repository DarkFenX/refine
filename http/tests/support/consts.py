# pylint: disable=C0103
from enum import StrEnum, IntEnum, unique


@unique
class State(StrEnum):
    ghost = 'ghost'
    offline = 'offline'
    online = 'online'
    active = 'active'
    overload = 'overload'


@unique
class ItemCat(IntEnum):
    charge = 8
    drone = 18
    fighter = 87
    implant = 20
    module = 7
    ship = 6
    skill = 16
    subsystem = 32


@unique
class Effect(IntEnum):
    online = 16
    self_rof = 1851
    missile_em_dmg_bonus = 660
    missile_therm_dmg_bonus = 662
    missile_kin_dmg_bonus2 = 668
    missile_expl_dmg_bonus = 661


@unique
class EffCat(IntEnum):
    passive = 0
    active = 1
    target = 2
    area = 3
    online = 4
    overload = 5
    dungeon = 6
    system = 7


@unique
class ModFunc(StrEnum):
    item = 'ItemModifier'
    loc = 'LocationModifier'
    loc_grp = 'LocationGroupModifier'
    loc_srq = 'LocationRequiredSkillModifier'
    own_srq = 'OwnerRequiredSkillModifier'


@unique
class ModDom(StrEnum):
    item = 'itemID'
    char = 'charID'
    ship = 'shipID'
    struct = 'structureID'
    tgt = 'targetID'
    other = 'otherID'


@unique
class ModOp(IntEnum):
    pre_assign = -1
    pre_mul = 0
    pre_div = 1
    mod_add = 2
    mod_sub = 3
    post_mul = 4
    post_div = 5
    post_percent = 6
    post_assign = 7


@unique
class Attr(IntEnum):
    # Resources
    cpu = 50
    cpu_output = 48
    power = 30
    power_output = 11
    # Damage
    em_dmg = 114
    therm_dmg = 118
    kin_dmg = 117
    expl_dmg = 116
    # Misc
    skill_level = 280
    speed = 51
    rof_bonus = 293
    dmg_mult_bonus = 292


@unique
class Rack(StrEnum):
    high = 'high'
    mid = 'mid'
    low = 'low'


@unique
class EffMode(StrEnum):
    full_compliance = 'full'
    state_compliance = 'state'
    force_run = 'run'
    force_stop = 'stop'
