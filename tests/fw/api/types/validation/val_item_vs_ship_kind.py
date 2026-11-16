import dataclasses
import typing

from fw.consts import ApiValItemStatus


@dataclasses.dataclass
class ValItemVsShipKindFail:

    ship_kind: str
    items: dict[str, ValItemVsShipKindItemInfo]

    def __init__(self, *, data: list | tuple) -> None:
        self.ship_kind, items = data
        self.items = {k: ValItemVsShipKindItemInfo(data=v) for k, v in items.items()}

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.ship_kind, self.items] == other


@dataclasses.dataclass
class ValItemVsShipKindItemInfo:

    status: ApiValItemStatus
    needed_ship_kind: str

    def __init__(self, *, data: list | tuple) -> None:
        status, self.needed_ship_kind = data
        self.status = ApiValItemStatus(status)

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.status, self.needed_ship_kind] == other
