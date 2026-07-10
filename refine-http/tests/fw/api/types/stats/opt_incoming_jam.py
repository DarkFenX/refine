import dataclasses
import typing

from fw.util import Absent

if typing.TYPE_CHECKING:
    from .opt_shared import StatTimeBurst, StatTimeSim


@dataclasses.dataclass(kw_only=True)
class StatsOptionInJam:

    time_options: StatTimeBurst | StatTimeSim | type[Absent] = Absent
