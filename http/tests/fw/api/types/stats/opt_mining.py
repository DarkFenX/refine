import dataclasses
import typing

from fw.util import Absent

if typing.TYPE_CHECKING:
    from .opt_shared import StatTimeBurst, StatTimeSim


@dataclasses.dataclass(kw_only=True)
class StatsOptionFitMining:

    item_kinds: StatMiningItemKinds | type[Absent] = Absent
    time_options: StatTimeBurst | StatTimeSim | type[Absent] = Absent


@dataclasses.dataclass(kw_only=True)
class StatsOptionItemMining:

    time_options: StatTimeBurst | StatTimeSim | type[Absent] = Absent
    ignore_state: bool | type[Absent] = Absent


@dataclasses.dataclass(kw_only=True)
class StatMiningItemKinds:

    default: bool | type[Absent] = Absent
    module: bool | type[Absent] = Absent
    minion: bool | type[Absent] = Absent
