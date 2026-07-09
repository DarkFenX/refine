import dataclasses

from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class StatsOptionJump:

    range: float | str | type[Absent] = Absent
    passenger_fit_ids: list[str] | type[Absent] = Absent
