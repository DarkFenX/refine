import dataclasses
import typing
from collections import UserDict

from fw.consts import ApiValItemStatus


class ValMaxGroupFail(UserDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({int(k): ValMaxGroupGroup(data=v) for k, v in data.items()})


@dataclasses.dataclass
class ValMaxGroupGroup:

    group_item_count: int
    items: dict[str, ValMaxGroupItemInfo]

    def __init__(self, *, data: list | tuple) -> None:
        self.group_item_count, items = data
        self.items = {k: ValMaxGroupItemInfo(data=v) for k, v in items.items()}

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.group_item_count, self.items] == other


@dataclasses.dataclass
class ValMaxGroupItemInfo:

    status: ApiValItemStatus
    limit: int

    def __init__(self, *, data: list | tuple) -> None:
        status, self.limit = data
        self.status = ApiValItemStatus(status)

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.status, self.limit] == other
