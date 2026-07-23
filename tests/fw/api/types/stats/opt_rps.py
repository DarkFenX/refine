import dataclasses
import typing

from fw.util import Absent

if typing.TYPE_CHECKING:
    from fw.api.aliases import DpsProfileAlias
    from .opt_shared import StatTimeBurst, StatTimeSim


@dataclasses.dataclass(kw_only=True)
class StatsOptionRps:

    time_options: StatTimeBurst | StatTimeSim | type[Absent] = Absent
    shield_perc: float | type[Absent] = Absent


@dataclasses.dataclass(kw_only=True)
class StatsOptionErps:

    incoming_dps: DpsProfileAlias | type[Absent] = Absent
    time_options: StatTimeBurst | StatTimeSim | type[Absent] = Absent
    shield_perc: float | type[Absent] = Absent
