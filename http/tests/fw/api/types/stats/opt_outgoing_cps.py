import dataclasses
import typing

from fw.util import Absent

if typing.TYPE_CHECKING:
    from .opt_shared import StatTimeBurst, StatTimeSim


@dataclasses.dataclass(kw_only=True)
class StatsOptionFitOutCps:

    time_options: StatTimeBurst | StatTimeSim | type[Absent] = Absent


@dataclasses.dataclass(kw_only=True)
class StatsOptionItemOutCps:

    time_options: StatTimeBurst | StatTimeSim | type[Absent] = Absent
    ignore_state: bool | type[Absent] = Absent
