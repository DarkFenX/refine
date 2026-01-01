import dataclasses

from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class StatsOptionItemOutCps:

    ignore_state: bool | type[Absent] = Absent
