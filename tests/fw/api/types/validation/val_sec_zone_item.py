import dataclasses
import typing

from fw.consts import ApiValItemStatus

if typing.TYPE_CHECKING:
    from fw.consts import ApiSecZone


@dataclasses.dataclass
class ValItemSecZoneFail:

    zone: ApiSecZone
    items: dict[str, ValItemSecZoneItemInfo]

    def __init__(self, *, data: list | tuple) -> None:
        self.zone, items = data
        self.items = {k: ValItemSecZoneItemInfo(data=v) for k, v in items.items()}

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.zone, self.items] == other


@dataclasses.dataclass
class ValItemSecZoneItemInfo:

    status: ApiValItemStatus
    allowed_zones: list[ApiSecZone]

    def __init__(self, *, data: list | tuple) -> None:
        status, allowed_zones = data
        self.status = ApiValItemStatus(status)
        self.allowed_zones = sorted(allowed_zones)

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.status, self.allowed_zones] == other
