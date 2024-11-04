# pylint: disable=C0103
from enum import Enum, IntEnum, StrEnum, unique


@unique
class EveItem(IntEnum):
    missile_launcher_operation = 3319
    high_speed_maneuvering = 3454
    micro_jump_drive_operation = 4385
    nanite_repair_paste = 28668
    stasis_webification_probe = 56748
    capital_micro_jump_drive_operation = 83464


@unique
class EveItemGrp(IntEnum):
    character = 1
    effect_beacon = 920


@unique
class EveItemCat(IntEnum):
    celestial = 2
    ship = 6
    module = 7
    charge = 8
    skill = 16
    drone = 18
    implant = 20
    subsystem = 32
    structure = 65
    fighter = 87


@unique
class EveEffect(IntEnum):
    online = 16
    use_missiles = 101
    missile_em_dmg_bonus = 660
    missile_expl_dmg_bonus = 661
    missile_therm_dmg_bonus = 662
    missile_kin_dmg_bonus = 668
    ammo_influence_cap_need = 804
    drone_dmg_bonus = 1730
    self_rof = 1851
    warp_disrupt_sphere = 3380
    script_duration_bonus = 3602
    script_wdfg_set_script_capneed_hidden = 3615
    script_warp_scramble_range_bonus = 3648
    hardpoint_modifier_effect = 3773
    slot_modifier = 3774
    max_range_hidden_preass_warp_scramble_range = 4894
    fueled_armor_repair = 5275
    remote_webifier_falloff = 6426
    doomsday_aoe_web = 6476
    fighter_ability_launch_bomb = 6485
    ship_module_arar = 6651
    mod_bonus_microwarpdrive = 6730
    mod_bonus_afterburner = 6731
    mod_bonus_warfare_link_armor = 6732
    mod_titan_effect_generator = 6753
    ship_mod_focused_warp_scrambling_script = 6848
    ship_mod_focused_warp_disruption_script = 6849
    weather_darkness = 7060
    debuff_lance = 11691


@unique
class EveEffCat(IntEnum):
    passive = 0
    active = 1
    target = 2
    area = 3
    online = 4
    overload = 5
    dungeon = 6
    system = 7


@unique
class EveModFunc(StrEnum):
    item = 'ItemModifier'
    loc = 'LocationModifier'
    loc_grp = 'LocationGroupModifier'
    loc_srq = 'LocationRequiredSkillModifier'
    own_srq = 'OwnerRequiredSkillModifier'


@unique
class EveModDom(StrEnum):
    item = 'itemID'
    char = 'charID'
    ship = 'shipID'
    struct = 'structureID'
    tgt = 'targetID'
    other = 'otherID'


@unique
class EveModOp(IntEnum):
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
class EveBuff(IntEnum):
    warp_penalty = 4
    disallow_dock_jump = 6
    stasis_webification_burst = 27
    disallow_tether = 57
    remote_repair_impedance = 2201


@unique
class EveBuffAggrMode(StrEnum):
    min = 'Minimum'
    max = 'Maximum'


@unique
class EveBuffOp(StrEnum):
    pre_assign = 'PreAssignment'
    pre_mul = 'PreMul'
    pre_div = 'PreDiv'
    mod_add = 'ModAdd'
    mod_sub = 'ModSub'
    post_mul = 'PostMul'
    post_div = 'PostDiv'
    post_percent = 'PostPercent'
    post_assign = 'PostAssignment'


@unique
class EveAttr(IntEnum):
    mass = 4
    capacitor_need = 6
    power_output = 11
    low_slots = 12
    med_slots = 13
    hi_slots = 14
    speed_factor = 20
    power = 30
    max_velocity = 37
    cpu_output = 48
    cpu = 50
    speed = 51
    max_range = 54
    dmg_mult = 64
    duration_bonus = 66
    duration = 73
    armor_dmg_amount = 84
    launcher_slots_left = 101
    turret_slots_left = 102
    warp_scramble_range = 103
    warp_scramble_status = 104
    warp_scramble_strength = 105
    em_dmg = 114
    expl_dmg = 116
    kin_dmg = 117
    therm_dmg = 118
    falloff = 158
    missile_dmg_mult = 212
    skill_level = 280
    dmg_mult_bonus = 292
    rof_bonus = 293
    cap_need_bonus = 317
    sig_radius = 552
    sig_radius_bonus = 554
    speed_boost_factor = 567
    mass_addition = 796
    disallow_assistance = 854
    sig_radius_bonus_bonus = 1227
    max_range_hidden = 1317
    capacitor_need_hidden = 1319
    warp_scramble_range_bonus = 1327
    activation_blocked = 1349
    activation_blocked_strength = 1350
    turret_hardpoint_modifier = 1368
    launcher_hardpoint_modifier = 1369
    hi_slot_modifier = 1374
    med_slot_modifier = 1375
    low_slot_modifier = 1376
    charged_armor_dmg_mult = 1886
    gate_scramble_status = 1973
    gate_scramble_strength = 1974
    remote_resistance_id = 2138
    fighter_ability_launch_bomb_type = 2324
    speed_factor_floor = 2266
    doomsday_aoe_range = 2279
    warfare_buff_1_id = 2468
    warfare_buff_1_value = 2469
    warfare_buff_2_id = 2470
    warfare_buff_2_value = 2471
    warfare_buff_3_id = 2472
    warfare_buff_3_value = 2473
    warfare_buff_4_id = 2536
    warfare_buff_4_value = 2537


@unique
class ApiItemKind(StrEnum):
    autocharge = 'autocharge'
    booster = 'booster'
    character = 'character'
    charge = 'charge'
    drone = 'drone'
    fighter = 'fighter'
    fw_effect = 'fw_effect'
    implant = 'implant'
    module = 'module'
    proj_effect = 'proj_effect'
    rig = 'rig'
    ship = 'ship'
    skill = 'skill'
    stance = 'stance'
    subsystem = 'subsystem'
    sw_effect = 'sw_effect'


@unique
class ApiState(StrEnum):
    ghost = 'ghost'
    offline = 'offline'
    online = 'online'
    active = 'active'
    overload = 'overload'


@unique
class ApiRack(StrEnum):
    high = 'high'
    mid = 'mid'
    low = 'low'


@unique
class ApiEffMode(StrEnum):
    full_compliance = 'full'
    state_compliance = 'state'
    force_run = 'run'
    force_stop = 'stop'


@unique
class ApiModOp(StrEnum):
    pre_assign = 'pre_assign'
    pre_mul = 'pre_mul'
    pre_div = 'pre_div'
    mod_add = 'add'
    mod_sub = 'sub'
    post_mul = 'post_mul'
    post_div = 'post_div'
    post_percent = 'post_perc'
    post_assign = 'post_assign'
    max_limit = 'max_limit'
    extra_mul = 'extra_mul'


@unique
class ApiAggrMode(StrEnum):
    stack = 'stack'


@unique
class ApiModAddMode(StrEnum):
    equip = 'equip'


@unique
class ApiSideEffectOp(StrEnum):
    add = 'add'
    perc = 'perc'


@unique
class ApiSolInfoMode(StrEnum):
    id = 'id'
    full = 'full'


@unique
class ApiFitInfoMode(StrEnum):
    id = 'id'
    full = 'full'


@unique
class ApiFleetInfoMode(StrEnum):
    id = 'id'
    full = 'full'


@unique
class ApiItemInfoMode(StrEnum):
    id = 'id'
    partial = 'partial'
    full = 'full'


@unique
class PenaltyStr(float, Enum):
    p1 = 1
    p2 = 0.8691199808003977
    p3 = 0.5705831435105605
    p4 = 0.2829551540232615
    p5 = 0.10599264974270461
    p6 = 0.02999116653328059
    p7 = 0.006410183117533543
    p8 = 0.0010349204826687127
    p9 = 0.0001262126825458958
    p10 = 0.000011626753929631052
    p11 = 0.0000008090464068743218
