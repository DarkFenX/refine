import dataclasses
import typing
from collections import UserDict

from fw.consts import ApiValItemStatus


class ValItemKindFail(UserDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({k: ValItemKindInfo(data=v) for k, v in data.items()})


@dataclasses.dataclass
class ValItemKindInfo:

    status: ApiValItemStatus
    kind: str | None
    expected_kind: str

    def __init__(self, *, data: list | tuple) -> None:
        status, self.kind, self.expected_kind = data
        self.status = ApiValItemStatus(status)

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.status, self.kind, self.expected_kind] == other
