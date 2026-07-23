import dataclasses
import typing

from fw.util import Absent

if typing.TYPE_CHECKING:
    from fw.api.aliases import DpsProfileAlias


@dataclasses.dataclass(kw_only=True)
class StatsOptionEhp:

    incoming_dps: DpsProfileAlias | type[Absent] = Absent
