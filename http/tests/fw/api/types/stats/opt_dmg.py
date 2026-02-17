import dataclasses
import typing

from fw.util import Absent

if typing.TYPE_CHECKING:
    from .opt_shared import StatTimeBurst, StatTimeSim


@dataclasses.dataclass(kw_only=True)
class StatsOptionFitDps:

    item_kinds: StatDmgItemKinds | type[Absent] = Absent
    time_options: StatTimeBurst | StatTimeSim | type[Absent] = Absent
    projectee_item_id: str | type[Absent] = Absent


@dataclasses.dataclass(kw_only=True)
class StatsOptionFitVolley:

    item_kinds: StatDmgItemKinds | type[Absent] = Absent
    time_options: StatTimeBurst | StatTimeSim | type[Absent] = Absent
    projectee_item_id: str | type[Absent] = Absent


@dataclasses.dataclass(kw_only=True)
class StatsOptionItemDps:

    time_options: StatTimeBurst | StatTimeSim | type[Absent] = Absent
    include_charges: bool | type[Absent] = Absent
    ignore_state: bool | type[Absent] = Absent
    projectee_item_id: str | type[Absent] = Absent


@dataclasses.dataclass(kw_only=True)
class StatsOptionItemVolley:

    time_options: StatTimeBurst | StatTimeSim | type[Absent] = Absent
    include_charges: bool | type[Absent] = Absent
    ignore_state: bool | type[Absent] = Absent
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
