import dataclasses
import typing

from tests.fw.util import Absent, dc_to_dict

if typing.TYPE_CHECKING:
    from tests.fw.api.aliases import DpsProfile


@dataclasses.dataclass(kw_only=True)
class StatsOptionEhp:

    incoming_dps: DpsProfile | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)
