import dataclasses
import typing

from fw.util import Absent

if typing.TYPE_CHECKING:
    from fw.consts import ApiCtlAffectors


@dataclasses.dataclass(kw_only=True)
class StatsOptionJump:

    range: float | str | type[Absent] = Absent
    passenger_fit_ids: list[str] | type[Absent] = Absent
    passenger_mass_increasers: float | ApiCtlAffectors | type[Absent] = Absent
