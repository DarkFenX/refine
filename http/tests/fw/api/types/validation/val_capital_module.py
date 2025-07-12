
import dataclasses
import typing


@dataclasses.dataclass
class ValCapModuleFail:

    max_subcap_volume: float
    module_volumes: dict[str, float]

    def __init__(self, *, data: list | tuple) -> None:
        self.max_subcap_volume, self.module_volumes = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.max_subcap_volume, self.module_volumes] == other
