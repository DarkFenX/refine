import dataclasses

from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class StatTimeBurst:

    mode: str = 'burst'
    spool: str | type[Absent] = Absent


@dataclasses.dataclass(kw_only=True)
class StatTimeSim:

    mode: str = 'sim'
    time: float | type[Absent] = Absent
    reload_optionals: bool | type[Absent] = Absent
    rearm_minions: bool | type[Absent] = Absent
