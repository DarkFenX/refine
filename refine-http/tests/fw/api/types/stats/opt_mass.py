import dataclasses
import typing

from fw.util import Absent

if typing.TYPE_CHECKING:
    from fw.consts import ApiCtlAffector


@dataclasses.dataclass(kw_only=True)
class StatsOptionMass:

    affectors: float | ApiCtlAffector | type[Absent] = Absent
