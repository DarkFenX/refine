import dataclasses
import typing


@dataclasses.dataclass
class ValDroneGroupFail:

    allowed_group_ids: list[int]
    drone_groups: dict[str, int]

    def __init__(self, *, data: list | tuple) -> None:
        allowed_group_ids, self.drone_groups = data
        self.allowed_group_ids = sorted(allowed_group_ids)

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        if isinstance(other, list) and len(other) >= 1:
            other[0] = sorted(other[0])
        return [self.allowed_group_ids, self.drone_groups] == other
