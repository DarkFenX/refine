import dataclasses

from tests.fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class StatCapSrcKinds:

    default: bool | type[Absent] = Absent
    regen: bool | type[Absent] = Absent
    cap_boosters: bool | type[Absent] = Absent
    consumers: bool | type[Absent] = Absent
    incoming_transfers: bool | type[Absent] = Absent
    incoming_neuts: bool | type[Absent] = Absent

    def to_dict(self) -> dict:
        return dataclasses.asdict(self, dict_factory=lambda d: {k: v for k, v in d if v is not Absent})
