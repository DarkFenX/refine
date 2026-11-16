import dataclasses
import typing

from fw.util import Absent

if typing.TYPE_CHECKING:
    from fw.consts import ApiOptionalReload, ApiRearmMinion


@dataclasses.dataclass(kw_only=True)
class StatTime:

    mode: str
    spool: str | type[Absent] = Absent
    time: float | type[Absent] | None = Absent
    optional_reloads: ApiOptionalReload | type[Absent] | None = Absent
    rearm_minions: ApiRearmMinion | type[Absent] | None = Absent

    @classmethod
    def burst(cls, *, spool: str | type[Absent] = Absent) -> typing.Self:
        return cls(mode='burst', spool=spool)

    @classmethod
    def sim(
            cls, *,
            time: float | type[Absent] | None = Absent,
            optional_reloads: ApiOptionalReload | type[Absent] | None = Absent,
            rearm_minions: ApiRearmMinion | type[Absent] | None = Absent,
    ) -> typing.Self:
        return cls(mode='sim', time=time, optional_reloads=optional_reloads, rearm_minions=rearm_minions)
