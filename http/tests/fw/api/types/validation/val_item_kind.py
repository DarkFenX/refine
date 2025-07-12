
import dataclasses
import typing


class ValItemKindFail(dict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({k: ValItemKindInfo(data=v) for k, v in data.items()})


@dataclasses.dataclass
class ValItemKindInfo:

    kind: str | None
    expected_kind: str

    def __init__(self, *, data: list | tuple) -> None:
        self.kind, self.expected_kind = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.kind, self.expected_kind] == other
