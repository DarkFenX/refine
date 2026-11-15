import dataclasses
import typing

from fw.consts import ApiValItemStatus


@dataclasses.dataclass
class ValResourceFail:

    used: float
    max: float | None
    users: dict[str, ValResourceItemInfo]

    def __init__(self, *, data: list | tuple) -> None:
        self.used, self.max, users = data
        self.users = {k: ValResourceItemInfo(data=v) for k, v in users.items()}

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.used, self.max, self.users] == other


@dataclasses.dataclass
class ValResourceItemInfo:

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
