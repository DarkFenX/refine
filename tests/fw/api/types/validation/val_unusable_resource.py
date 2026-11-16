import dataclasses
import typing

from fw.consts import ApiValItemStatus


@dataclasses.dataclass
class ValUnusableResFail:

    max: float | None
    users: dict[str, ValUnusableResItemInfo]

    def __init__(self, *, data: list | tuple) -> None:
        self.max, users = data
        self.users = {k: ValUnusableResItemInfo(data=v) for k, v in users.items()}

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.max, self.users] == other


@dataclasses.dataclass
class ValUnusableResItemInfo:

    status: ApiValItemStatus
    used: float

    def __init__(self, *, data: list | tuple) -> None:
        status, self.used = data
        self.status = ApiValItemStatus(status)

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.status, self.used] == other
