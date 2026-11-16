import dataclasses
import typing
from collections import UserDict

from fw.api.types.helpers import effect_http_to_fw
from fw.consts import ApiValItemStatus


class ValEffectStopperFail(UserDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({k: ValEffectStopperItemInfo(data=v) for k, v in data.items()})


@dataclasses.dataclass
class ValEffectStopperItemInfo:

    status: ApiValItemStatus
    effect_ids: list[int | str]

    def __init__(self, *, data: list | tuple) -> None:
        status, effect_ids = data
        self.status = ApiValItemStatus(status)
        self.effect_ids = sorted(effect_http_to_fw(effect_id=e) for e in effect_ids)

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.status, self.effect_ids] == other
