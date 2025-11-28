import dataclasses

from tests.fw.util import Absent, dc_to_dict


@dataclasses.dataclass(kw_only=True)
class StatsOptionItemOutCps:

    ignore_state: bool | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dc_to_dict(data=self)
