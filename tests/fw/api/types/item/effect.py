import dataclasses
import typing

if typing.TYPE_CHECKING:
    from fw.consts import ApiEffMode


@dataclasses.dataclass
class EffectInfo:

    running: bool
    mode: ApiEffMode

    def __init__(self, *, data: list | tuple) -> None:
        self.running, self.mode = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.running, self.mode] == other
