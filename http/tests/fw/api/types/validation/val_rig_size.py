
import dataclasses
import typing


@dataclasses.dataclass
class ValRigSizeFail:

    allowed_size: float
    rig_sizes: dict[str, float | None]

    def __init__(self, *, data: list | tuple) -> None:
        self.allowed_size, self.rig_sizes = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.allowed_size, self.rig_sizes] == other
