import dataclasses
import typing

from fw.util import Absent

if typing.TYPE_CHECKING:
    from fw.consts import ApiStatCrits, ApiStatItemCharges, ApiStatItemState
    from .opt_shared import StatTimeBurst, StatTimeSim


@dataclasses.dataclass(kw_only=True)
class StatsOptionFitDmg:

    item_kinds: StatDmgItemKinds | type[Absent] = Absent
    time: StatTimeBurst | StatTimeSim | type[Absent] = Absent
    crits: ApiStatCrits | type[Absent] = Absent
    projectee_item_id: str | type[Absent] = Absent


@dataclasses.dataclass(kw_only=True)
class StatsOptionItemDmg:

    time: StatTimeBurst | StatTimeSim | type[Absent] = Absent
    crits: ApiStatCrits | type[Absent] = Absent
    charges: ApiStatItemCharges | type[Absent] = Absent
    state: ApiStatItemState | type[Absent] = Absent
    projectee_item_id: str | type[Absent] = Absent


@dataclasses.dataclass(kw_only=True)
class StatDmgItemKinds:

    default: bool | type[Absent] = Absent
    turret: bool | type[Absent] = Absent
    missile: bool | type[Absent] = Absent
    breacher: bool | type[Absent] = Absent
    vorton: bool | type[Absent] = Absent
    bomb: bool | type[Absent] = Absent
    smartbomb: bool | type[Absent] = Absent
    superweapon: bool | type[Absent] = Absent
    minion_mobile: bool | type[Absent] = Absent
    minion_static: bool | type[Absent] = Absent
