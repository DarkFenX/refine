import dataclasses
import typing

from fw.consts import ApiValItemStatus


@dataclasses.dataclass
class ValCapModuleFail:

    max_subcap_volume: float
    module_volumes: dict[str, ValCapModuleInfo]

    def __init__(self, *, data: list | tuple) -> None:
        self.max_subcap_volume, module_volumes = data
        self.module_volumes = {k: ValCapModuleInfo(data=v) for k, v in module_volumes.items()}

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.max_subcap_volume, self.module_volumes] == other


@dataclasses.dataclass
class ValCapModuleInfo:

    status: ApiValItemStatus
    volume: float

    def __init__(self, *, data: list | tuple) -> None:
        status, self.volume = data
        self.status = ApiValItemStatus(status)

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.status, self.volume] == other
