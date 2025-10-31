import dataclasses

from tests.fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class StatRemoteRepItemKinds:

    default: bool | type[Absent] = Absent
    module: bool | type[Absent] = Absent
    minion: bool | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dataclasses.asdict(self, dict_factory=lambda d: {k: v for k, v in d if v is not Absent})
