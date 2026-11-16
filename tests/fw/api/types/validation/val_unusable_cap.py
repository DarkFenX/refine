import dataclasses
import typing

from fw.consts import ApiValItemStatus


@dataclasses.dataclass
class ValUnusableCapFail:

    max_cap: float
    items: dict[str, ValUnusableCapItemInfo]

    def __init__(self, *, data: list | tuple) -> None:
        self.max_cap, items = data
        self.items = {k: ValUnusableCapItemInfo(data=v) for k, v in items.items()}

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.max_cap, self.items] == other


@dataclasses.dataclass
class ValUnusableCapItemInfo:

    status: ApiValItemStatus
    cap_use: float

    def __init__(self, *, data: list | tuple) -> None:
        status, self.cap_use = data
        self.status = ApiValItemStatus(status)

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.status, self.cap_use] == other
