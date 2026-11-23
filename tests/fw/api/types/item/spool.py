import dataclasses
import typing

from fw.api.types.helpers import effect_http_to_fw


@dataclasses.dataclass
class ItemSpoolInfo:

    setting: str
    cycles: dict[int | str, SpoolCycleInfo]

    def __init__(self, *, data: list | tuple) -> None:
        setting, cycles = data
        self.setting = setting
        self.cycles = {effect_http_to_fw(effect_id=k): SpoolCycleInfo(data=v) for k, v in cycles.items()}

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.setting, self.cycles] == other


@dataclasses.dataclass
class SpoolCycleInfo:

    current: int
    max: int

    def __init__(self, *, data: list | tuple) -> None:
        self.current, self.max = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.current, self.max] == other

