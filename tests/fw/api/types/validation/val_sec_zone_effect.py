import dataclasses
import typing

from fw.api.types.helpers import effect_http_to_fw
from fw.consts import ApiValItemStatus

if typing.TYPE_CHECKING:
    from fw.consts import ApiSecZone


@dataclasses.dataclass
class ValEffectSecZoneFail:

    zone: ApiSecZone
    items: dict[str, ValEffectSecZoneItemInfo]

    def __init__(self, *, data: list | tuple) -> None:
        self.zone, items = data
        self.items = {k: ValEffectSecZoneItemInfo(data=v) for k, v in items.items()}

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.zone, self.items] == other


@dataclasses.dataclass
class ValEffectSecZoneItemInfo:

    status: ApiValItemStatus
    effects: dict[int | str, list[ApiSecZone]]

    def __init__(self, *, data: list | tuple) -> None:
        status, effects = data
        self.status = ApiValItemStatus(status)
        self.effects = {effect_http_to_fw(effect_id=k): sorted(v) for k, v in effects.items()}

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.status, self.effects] == other
