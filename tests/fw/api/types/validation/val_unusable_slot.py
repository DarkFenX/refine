import dataclasses
import typing

from fw.consts import ApiValItemStatus


@dataclasses.dataclass
class ValUnusableSlotFail:

    max: int | None
    users: dict[str, ApiValItemStatus]

    def __init__(self, *, data: list | tuple) -> None:
        self.max, users = data
        self.users = {k: ApiValItemStatus(v) for k, v in users.items()}

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.max, self.users] == other
