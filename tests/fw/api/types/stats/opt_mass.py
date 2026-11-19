import dataclasses
import typing

from fw.util import Absent

if typing.TYPE_CHECKING:
    from fw.consts import ApiCtlAffectors


@dataclasses.dataclass(kw_only=True)
class StatsOptionMass:

    increasers: float | ApiCtlAffectors | type[Absent] = Absent
