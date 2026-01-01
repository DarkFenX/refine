import dataclasses
import typing

from fw.util import Absent

if typing.TYPE_CHECKING:
    from .opt_shared import StatTimeBurst, StatTimeSim


@dataclasses.dataclass(kw_only=True)
class StatsOptionFitOutNps:

    item_kinds: StatNeutItemKinds | type[Absent] = Absent
    time_options: StatTimeBurst | StatTimeSim | type[Absent] = Absent
    projectee_item_id: str | type[Absent] = Absent


@dataclasses.dataclass(kw_only=True)
class StatsOptionItemOutNps:

    time_options: StatTimeBurst | StatTimeSim | type[Absent] = Absent
    include_charges: bool | type[Absent] = Absent
    ignore_state: bool | type[Absent] = Absent
    projectee_item_id: str | type[Absent] = Absent


@dataclasses.dataclass(kw_only=True)
class StatNeutItemKinds:

    default: bool | type[Absent] = Absent
    module: bool | type[Absent] = Absent
    minion: bool | type[Absent] = Absent
    bomb: bool | type[Absent] = Absent
