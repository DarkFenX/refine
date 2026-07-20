import dataclasses
import typing


@dataclasses.dataclass
class ValSlotCountFail:

    used: int
    max: int | None
    users: list[str]

    def __init__(self, *, data: list | tuple) -> None:
        self.used, self.max, users = data
        self.users = sorted(users)

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.used, self.max, self.users] == other
