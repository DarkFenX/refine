import dataclasses
import typing

from fw.consts import ApiValItemStatus


@dataclasses.dataclass
class ValDroneGroupFail:

    allowed_group_ids: list[int]
    drone_groups: dict[str, ValDroneGroupInfo]

    def __init__(self, *, data: list | tuple) -> None:
        allowed_group_ids, drone_groups = data
        self.allowed_group_ids = sorted(allowed_group_ids)
        self.drone_groups = {k: ValDroneGroupInfo(data=v) for k, v in drone_groups.items()}

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        if isinstance(other, list) and len(other) >= 1:
            other[0] = sorted(other[0])
        return [self.allowed_group_ids, self.drone_groups] == other


@dataclasses.dataclass
class ValDroneGroupInfo:

    status: ApiValItemStatus
    group_id: int

    def __init__(self, *, data: list | tuple) -> None:
        status, self.group_id = data
        self.status = ApiValItemStatus(status)

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.status, self.group_id] == other
