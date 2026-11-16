import dataclasses
import typing

from fw.util import Absent

if typing.TYPE_CHECKING:
    from .opt_shared import StatTime


@dataclasses.dataclass(kw_only=True)
class StatsOptionInJam:

    time: StatTime | type[Absent] = Absent
