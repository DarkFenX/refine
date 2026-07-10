import dataclasses
import typing

if typing.TYPE_CHECKING:
    from fw.consts import ApiRearmMinion


@dataclasses.dataclass(kw_only=True)
class ItemRearmMinionInfo:

    value: ApiRearmMinion
    overridden: bool

    def __init__(self, *, data: list | tuple) -> None:
        self.value, self.overridden = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.value, self.overridden] == other
