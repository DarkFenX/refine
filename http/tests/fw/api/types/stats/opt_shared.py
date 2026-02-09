import dataclasses
import typing

from fw.util import Absent

if typing.TYPE_CHECKING:
    from fw.consts import ApiOptionalReload, ApiRearmMinion


@dataclasses.dataclass(kw_only=True)
class StatTimeBurst:

    mode: str = 'burst'
    spool: str | type[Absent] = Absent


@dataclasses.dataclass(kw_only=True)
class StatTimeSim:

    mode: str = 'sim'
    time: float | type[Absent] | None = Absent
    optional_reloads: ApiOptionalReload | type[Absent] | None = Absent
    rearm_minions: ApiRearmMinion | type[Absent] | None = Absent
