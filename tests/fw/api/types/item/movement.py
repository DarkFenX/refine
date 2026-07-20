import dataclasses
import typing


@dataclasses.dataclass(kw_only=True)
class Movement:

    azimuth: float
    elevation: float
    speed: float

    def __init__(self, *, data: list | tuple) -> None:
        self.azimuth, self.elevation, self.speed = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.azimuth, self.elevation, self.speed] == other
