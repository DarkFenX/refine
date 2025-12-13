import dataclasses
import typing

if typing.TYPE_CHECKING:
    from tests.fw.consts import ApiSecZone


@dataclasses.dataclass
class ValEffectSecZoneFail:

    zone: ApiSecZone
    items: dict[str, dict[str, list[ApiSecZone]]]

    def __init__(self, *, data: list | tuple) -> None:
        self.zone, items = data
        self.items = {k1: {k2: sorted(v2) for k2, v2 in v1.items()} for k1, v1 in items.items()}

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.zone, self.items] == other
