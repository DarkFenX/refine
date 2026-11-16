import dataclasses
import typing

from fw.consts import ApiValItemStatus


@dataclasses.dataclass
class ValRigSizeFail:

    allowed_size: float
    rig_sizes: dict[str, ValRigSizeItemInfo]

    def __init__(self, *, data: list | tuple) -> None:
        self.allowed_size, rig_sizes = data
        self.rig_sizes = {k: ValRigSizeItemInfo(data=v) for k, v in rig_sizes.items()}

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.allowed_size, self.rig_sizes] == other


@dataclasses.dataclass
class ValRigSizeItemInfo:

    status: ApiValItemStatus
    size: float | None

    def __init__(self, *, data: list | tuple) -> None:
        status, self.size = data
        self.status = ApiValItemStatus(status)

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.status, self.size] == other
